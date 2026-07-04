use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::app::error::DaemonError;
use crate::domain::capability::CapabilityStatus;

pub const CGROUP_FREEZE_FILE: &str = "cgroup.freeze";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgroupFreezerPreference {
    AndroidAppCgroupV2,
    SystemCgroupV2,
    GenericCgroupV2,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CgroupFreezerCapability {
    pub status: CapabilityStatus,
    pub preference: CgroupFreezerPreference,
    pub freeze_files: Vec<PathBuf>,
    pub evidence: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreezeState {
    Thawed,
    Frozen,
}

impl FreezeState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Thawed => "0",
            Self::Frozen => "1",
        }
    }
}

pub fn discover_freeze_files(root: impl AsRef<Path>) -> Result<Vec<PathBuf>, DaemonError> {
    let root = root.as_ref();
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    discover_freeze_files_inner(root, &mut paths)?;
    Ok(paths)
}

pub fn detect_cgroup_v2_freezer_capability(
    cgroup_root: impl AsRef<Path>,
) -> Result<CgroupFreezerCapability, DaemonError> {
    let cgroup_root = cgroup_root.as_ref();
    let controllers = cgroup_root.join("cgroup.controllers");
    let controllers_text = fs::read_to_string(&controllers).unwrap_or_default();
    let controller_has_freezer = controllers_text
        .split_whitespace()
        .any(|controller| controller == "freezer");

    let apps_root = cgroup_root.join("apps");
    let system_root = cgroup_root.join("system");
    let app_files = discover_freeze_files(&apps_root)?;
    let system_files = discover_freeze_files(&system_root)?;
    let generic_files = discover_freeze_files(cgroup_root)?;

    let (preference, freeze_files) = if !app_files.is_empty() {
        (CgroupFreezerPreference::AndroidAppCgroupV2, app_files)
    } else if !system_files.is_empty() {
        (CgroupFreezerPreference::SystemCgroupV2, system_files)
    } else if !generic_files.is_empty() {
        (CgroupFreezerPreference::GenericCgroupV2, generic_files)
    } else {
        (CgroupFreezerPreference::Missing, Vec::new())
    };

    let status = if !freeze_files.is_empty() {
        CapabilityStatus::Available
    } else {
        CapabilityStatus::Missing
    };

    Ok(CgroupFreezerCapability {
        status,
        preference,
        evidence: format!(
            "{} contains freezer={controller_has_freezer}; freeze_files={}",
            controllers.display(),
            freeze_files.len()
        ),
        freeze_files,
    })
}

fn discover_freeze_files_inner(path: &Path, paths: &mut Vec<PathBuf>) -> Result<(), DaemonError> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child_path = entry.path();
        if child_path
            .file_name()
            .is_some_and(|name| name == CGROUP_FREEZE_FILE)
        {
            paths.push(child_path);
        } else if child_path.is_dir() {
            discover_freeze_files_inner(&child_path, paths)?;
        }
    }
    Ok(())
}

pub fn read_freeze_state(path: impl AsRef<Path>) -> Result<FreezeState, DaemonError> {
    match fs::read_to_string(path)?.trim() {
        "0" => Ok(FreezeState::Thawed),
        "1" => Ok(FreezeState::Frozen),
        value => Err(DaemonError::system(format!(
            "unknown cgroup.freeze state {value}"
        ))),
    }
}

pub fn write_freeze_state(path: impl AsRef<Path>, state: FreezeState) -> Result<(), DaemonError> {
    fs::write(path, state.as_str())?;
    Ok(())
}
