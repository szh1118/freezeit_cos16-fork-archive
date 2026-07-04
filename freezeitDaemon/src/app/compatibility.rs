use crate::domain::capability::{CapabilityName, CapabilityStatus, ControlCapability};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityBaseline {
    pub device_model: String,
    pub android_version: String,
    pub sdk: u32,
    pub fingerprint: String,
    pub kernel: String,
    pub root_ready: bool,
    pub hook_ready: bool,
    pub freezer_ready: bool,
}

impl CompatibilityBaseline {
    pub fn target_cph2653_android16() -> Self {
        Self {
            device_model: "CPH2653".to_owned(),
            android_version: "16".to_owned(),
            sdk: 36,
            fingerprint: "OnePlus/CPH2653EEA/OP5D55L1:16/BP2A.250605.015/V.R4T3.1338e95_e24685_de185d:user/release-keys".to_owned(),
            kernel: "6.6.89-android15".to_owned(),
            root_ready: true,
            hook_ready: true,
            freezer_ready: true,
        }
    }

    pub fn compatibility_json(&self, capabilities: &[ControlCapability]) -> String {
        let capabilities = capabilities
            .iter()
            .map(|capability| {
                format!(
                    "{{\"name\":\"{}\",\"status\":\"{}\",\"evidence\":\"{}\"}}",
                    capability_name(capability.name),
                    capability_status(capability.status),
                    escape_json(&capability.evidence)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"deviceModel\":\"{}\",\"androidVersion\":\"{}\",\"sdk\":{},\"fingerprint\":\"{}\",\"kernel\":\"{}\",\"rootReady\":{},\"hookReady\":{},\"freezerReady\":{},\"capabilities\":[{}]}}",
            escape_json(&self.device_model),
            escape_json(&self.android_version),
            self.sdk,
            escape_json(&self.fingerprint),
            escape_json(&self.kernel),
            self.root_ready,
            self.hook_ready,
            self.freezer_ready,
            capabilities
        )
    }

    pub fn allows_control(&self, capabilities: &[ControlCapability]) -> bool {
        self.root_ready
            && self.hook_ready
            && self.freezer_ready
            && capabilities
                .iter()
                .all(|capability| capability.status == CapabilityStatus::Available)
    }
}

fn capability_name(name: CapabilityName) -> &'static str {
    match name {
        CapabilityName::Root => "root",
        CapabilityName::PackageInventory => "package_inventory",
        CapabilityName::LsposedSystemServer => "lsposed_system_server",
        CapabilityName::CgroupV2Freezer => "cgroup_v2_freezer",
        CapabilityName::BinderFreezer => "binder_freezer",
        CapabilityName::SignalControl => "signal_control",
        CapabilityName::NetworkBreak => "network_break",
        CapabilityName::WakelockControl => "wakelock_control",
    }
}

fn capability_status(status: CapabilityStatus) -> &'static str {
    match status {
        CapabilityStatus::Available => "available",
        CapabilityStatus::Missing => "missing",
        CapabilityStatus::Degraded => "degraded",
        CapabilityStatus::Untested => "untested",
    }
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect(),
            '\n' => "\\n".chars().collect(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            other => vec![other],
        })
        .collect()
}
