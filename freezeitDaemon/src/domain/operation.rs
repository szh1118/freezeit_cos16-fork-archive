#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlAction {
    Freeze,
    Unfreeze,
    Terminate,
    Postpone,
    Fallback,
    Skip,
    Recover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationResult {
    Success,
    Partial,
    Failed,
    Skipped,
    Postponed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlOperation {
    pub operation_id: u64,
    pub timestamp_ms: u128,
    pub package_name: String,
    pub uid: u32,
    pub pid_list: Vec<i32>,
    pub action: ControlAction,
    pub backend: String,
    pub reason: String,
    pub result: OperationResult,
    pub details: String,
}
