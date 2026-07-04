pub const SIGSTOP_NUMBER: i32 = 19;
pub const SIGCONT_NUMBER: i32 = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalAction {
    Stop,
    Continue,
}

impl SignalAction {
    pub fn signal_number(self) -> i32 {
        match self {
            Self::Stop => SIGSTOP_NUMBER,
            Self::Continue => SIGCONT_NUMBER,
        }
    }
}

pub fn is_signal_allowed(test_mode: bool, pid: i32) -> bool {
    test_mode && pid > 0
}

pub fn send_signal(pid: i32, action: SignalAction) -> Result<(), crate::app::error::DaemonError> {
    if pid <= 0 {
        return Err(crate::app::error::DaemonError::system(
            "refusing to signal non-positive pid",
        ));
    }

    let result = unsafe { libc::kill(pid, action.signal_number()) };
    if result == 0 {
        Ok(())
    } else {
        Err(crate::app::error::DaemonError::from(
            std::io::Error::last_os_error(),
        ))
    }
}
