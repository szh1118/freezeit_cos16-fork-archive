use std::{
    io::{Read, Write},
    mem,
    os::fd::FromRawFd,
    os::unix::net::UnixStream,
    time::Duration,
};

use crate::{
    app::error::DaemonError,
    protocol::xposed::{encode_frame, parse_foreground_uid_payload, XposedCommand},
};

pub const XPOSED_SOCKET_ABSTRACT_NAME: &str = "FreezeitXposedServer";

pub fn query_hook_health() -> Result<String, DaemonError> {
    request_text(XposedCommand::GetHookHealth, &[])
}

pub fn query_foreground_uids() -> Result<Vec<u32>, DaemonError> {
    let response = request_bytes(XposedCommand::GetForeground, &[])?;
    parse_foreground_uid_payload(&response)
}

pub fn set_config(payload: &[u8]) -> Result<bool, DaemonError> {
    let response = request_bytes(XposedCommand::SetConfig, payload)?;
    if response.len() < 4 {
        return Err(DaemonError::protocol(
            "xposed set config response header is incomplete",
        ));
    }

    Ok(i32::from_le_bytes([response[0], response[1], response[2], response[3]]) == 2)
}

pub fn request_text(command: XposedCommand, payload: &[u8]) -> Result<String, DaemonError> {
    let response = request_bytes(command, payload)?;
    String::from_utf8(response)
        .map_err(|error| DaemonError::protocol(format!("xposed response is not utf-8: {error}")))
}

pub fn request_bytes(command: XposedCommand, payload: &[u8]) -> Result<Vec<u8>, DaemonError> {
    let request = encode_frame(command, payload)?;
    let mut stream = connect_abstract_socket(XPOSED_SOCKET_ABSTRACT_NAME)?;
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .map_err(DaemonError::from)?;
    stream
        .set_write_timeout(Some(Duration::from_secs(3)))
        .map_err(DaemonError::from)?;
    stream.write_all(&request).map_err(DaemonError::from)?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(DaemonError::from)?;
    Ok(response)
}

fn connect_abstract_socket(name: &str) -> Result<UnixStream, DaemonError> {
    let name_bytes = name.as_bytes();
    let max_name_len =
        mem::size_of::<libc::sockaddr_un>() - mem::size_of::<libc::sa_family_t>() - 1;
    if name_bytes.len() > max_name_len {
        return Err(DaemonError::system("xposed socket name is too long"));
    }

    // SAFETY: socket/connect are called with a fully initialized sockaddr_un and
    // the returned fd is transferred into UnixStream exactly once on success.
    unsafe {
        let fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
        if fd < 0 {
            return Err(DaemonError::from(std::io::Error::last_os_error()));
        }

        let mut addr: libc::sockaddr_un = mem::zeroed();
        addr.sun_family = libc::AF_UNIX as libc::sa_family_t;
        addr.sun_path[0] = 0;
        for (index, byte) in name_bytes.iter().enumerate() {
            addr.sun_path[index + 1] = *byte as libc::c_char;
        }

        let len = (mem::size_of::<libc::sa_family_t>() + 1 + name_bytes.len()) as libc::socklen_t;
        let result = libc::connect(fd, &addr as *const _ as *const libc::sockaddr, len);
        if result < 0 {
            let error = std::io::Error::last_os_error();
            libc::close(fd);
            return Err(DaemonError::from(error));
        }

        Ok(UnixStream::from_raw_fd(fd))
    }
}
