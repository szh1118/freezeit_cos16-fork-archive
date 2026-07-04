#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityName {
    Root,
    PackageInventory,
    LsposedSystemServer,
    CgroupV2Freezer,
    BinderFreezer,
    SignalControl,
    NetworkBreak,
    WakelockControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityStatus {
    Available,
    Missing,
    Degraded,
    Untested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Normal,
    Caution,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlCapability {
    pub name: CapabilityName,
    pub status: CapabilityStatus,
    pub evidence: String,
    pub checked_at_ms: u128,
    pub risk_level: RiskLevel,
}

impl ControlCapability {
    pub fn missing(name: CapabilityName, evidence: impl Into<String>) -> Self {
        Self {
            name,
            status: CapabilityStatus::Missing,
            evidence: evidence.into(),
            checked_at_ms: 0,
            risk_level: RiskLevel::Disabled,
        }
    }
}
