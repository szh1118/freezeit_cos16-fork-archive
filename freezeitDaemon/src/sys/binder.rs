use std::path::{Path, PathBuf};

use crate::domain::capability::CapabilityStatus;

pub fn binder_device_candidates() -> [&'static str; 2] {
    ["/dev/binder", "/dev/binderfs/binder"]
}

pub fn discover_binder_device() -> Option<&'static str> {
    binder_device_candidates()
        .into_iter()
        .find(|candidate| Path::new(candidate).exists())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinderFreezeRequest {
    Freeze,
    Unfreeze,
}

pub fn binder_freezer_ioctl_number(request: BinderFreezeRequest) -> u64 {
    match request {
        BinderFreezeRequest::Freeze => 0x4004_620e,
        BinderFreezeRequest::Unfreeze => 0x4004_620f,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinderFreezerCapability {
    pub status: CapabilityStatus,
    pub device_path: Option<String>,
    pub evidence: String,
}

pub fn detect_binder_freezer_capability() -> BinderFreezerCapability {
    let candidates = binder_device_candidates()
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    detect_binder_freezer_capability_from_candidates(&candidates)
}

pub fn detect_binder_freezer_capability_from_candidates(
    candidates: &[PathBuf],
) -> BinderFreezerCapability {
    let Some(path) = candidates.iter().find(|candidate| candidate.exists()) else {
        return BinderFreezerCapability {
            status: CapabilityStatus::Missing,
            device_path: None,
            evidence: "no binder device found".to_owned(),
        };
    };

    BinderFreezerCapability {
        status: CapabilityStatus::Untested,
        device_path: Some(path.display().to_string()),
        evidence: format!(
            "binder device present; BINDER_FREEZE ioctl=0x{:x}, BINDER_UNFREEZE ioctl=0x{:x}; target probe required before marking available",
            binder_freezer_ioctl_number(BinderFreezeRequest::Freeze),
            binder_freezer_ioctl_number(BinderFreezeRequest::Unfreeze)
        ),
    }
}
