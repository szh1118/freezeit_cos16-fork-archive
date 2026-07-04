use crate::app::error::DaemonError;

pub const XPOSED_SOCKET_NAME: &str = "\0FreezeitXposedServer";
pub const FREEZEIT_COMMAND_BASE: i32 = 1_359_322_925;
pub const HEADER_LEN: usize = 8;
pub const MAX_PAYLOAD_LEN: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum XposedCommand {
    GetForeground = FREEZEIT_COMMAND_BASE + 1,
    GetScreen = FREEZEIT_COMMAND_BASE + 2,
    GetXpLog = FREEZEIT_COMMAND_BASE + 3,
    SetConfig = FREEZEIT_COMMAND_BASE + 20,
    SetWakeupLock = FREEZEIT_COMMAND_BASE + 21,
    BreakNetwork = FREEZEIT_COMMAND_BASE + 41,
    UpdatePending = FREEZEIT_COMMAND_BASE + 60,
    GetHookHealth = FREEZEIT_COMMAND_BASE + 70,
    GetRuntimeAppStates = FREEZEIT_COMMAND_BASE + 71,
    GetSystemFreezerHints = FREEZEIT_COMMAND_BASE + 72,
}

impl TryFrom<i32> for XposedCommand {
    type Error = DaemonError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::GetForeground as i32 => Ok(Self::GetForeground),
            x if x == Self::GetScreen as i32 => Ok(Self::GetScreen),
            x if x == Self::GetXpLog as i32 => Ok(Self::GetXpLog),
            x if x == Self::SetConfig as i32 => Ok(Self::SetConfig),
            x if x == Self::SetWakeupLock as i32 => Ok(Self::SetWakeupLock),
            x if x == Self::BreakNetwork as i32 => Ok(Self::BreakNetwork),
            x if x == Self::UpdatePending as i32 => Ok(Self::UpdatePending),
            x if x == Self::GetHookHealth as i32 => Ok(Self::GetHookHealth),
            x if x == Self::GetRuntimeAppStates as i32 => Ok(Self::GetRuntimeAppStates),
            x if x == Self::GetSystemFreezerHints as i32 => Ok(Self::GetSystemFreezerHints),
            _ => Err(DaemonError::protocol(format!(
                "unknown xposed command {value}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XposedFrame {
    pub command: XposedCommand,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookBridgeStatus {
    Active,
    Missing(String),
    Degraded(String),
}

impl HookBridgeStatus {
    pub fn is_ready_for_control(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn health_label(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Missing(_) => "missing",
            Self::Degraded(_) => "degraded",
        }
    }
}

pub fn classify_bridge_error(error: &DaemonError) -> HookBridgeStatus {
    let message = error.to_string();
    if message.contains("No such file")
        || message.contains("Connection refused")
        || message.contains("not found")
    {
        HookBridgeStatus::Missing(message)
    } else {
        HookBridgeStatus::Degraded(message)
    }
}

pub fn classify_hook_health_payload(payload: &str) -> HookBridgeStatus {
    if payload.contains("\"status\":\"active\"") {
        HookBridgeStatus::Active
    } else if payload.contains("\"status\":\"degraded\"") {
        HookBridgeStatus::Degraded(payload.to_owned())
    } else {
        HookBridgeStatus::Degraded(format!("unrecognized hook health payload: {payload}"))
    }
}

pub fn parse_foreground_uid_payload(payload: &[u8]) -> Result<Vec<u32>, DaemonError> {
    if payload.len() < 4 {
        return Err(DaemonError::protocol(
            "foreground uid payload count is missing",
        ));
    }

    let count = i32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    if count < 0 {
        return Err(DaemonError::protocol(
            "foreground uid payload count is negative",
        ));
    }

    let count = count as usize;
    let expected_len = 4 + count * 4;
    if payload.len() < expected_len {
        return Err(DaemonError::protocol(format!(
            "foreground uid payload length mismatch: expected at least {expected_len}, got {}",
            payload.len()
        )));
    }

    Ok((0..count)
        .map(|index| {
            let offset = 4 + index * 4;
            i32::from_le_bytes([
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            ]) as u32
        })
        .collect())
}

pub fn parse_frame(bytes: &[u8]) -> Result<XposedFrame, DaemonError> {
    if bytes.len() < HEADER_LEN {
        return Err(DaemonError::protocol("xposed frame header is incomplete"));
    }

    let command = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let payload_len = i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    if payload_len < 0 {
        return Err(DaemonError::protocol(
            "xposed frame payload length is negative",
        ));
    }

    let payload_len = payload_len as usize;
    if payload_len > MAX_PAYLOAD_LEN {
        return Err(DaemonError::protocol("xposed frame payload is too large"));
    }

    let expected_len = HEADER_LEN + payload_len;
    if bytes.len() != expected_len {
        return Err(DaemonError::protocol(format!(
            "xposed frame length mismatch: expected {expected_len}, got {}",
            bytes.len()
        )));
    }

    Ok(XposedFrame {
        command: XposedCommand::try_from(command)?,
        payload: bytes[HEADER_LEN..].to_vec(),
    })
}

pub fn encode_frame(command: XposedCommand, payload: &[u8]) -> Result<Vec<u8>, DaemonError> {
    if payload.len() > MAX_PAYLOAD_LEN {
        return Err(DaemonError::protocol("xposed frame payload is too large"));
    }

    let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
    bytes.extend_from_slice(&(command as i32).to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as i32).to_le_bytes());
    bytes.extend_from_slice(payload);
    Ok(bytes)
}
