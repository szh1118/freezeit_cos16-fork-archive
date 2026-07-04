use std::{fs, path::Path};

use crate::app::error::DaemonError;

pub fn load_initial_config() -> Result<(), DaemonError> {
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonPaths {
    pub module_dir: String,
    pub app_config: String,
    pub app_label: String,
    pub settings_db: String,
}

impl DaemonPaths {
    pub fn from_module_dir(module_dir: impl Into<String>) -> Self {
        let module_dir = module_dir.into();
        Self {
            app_config: format!("{module_dir}/appcfg.txt"),
            app_label: format!("{module_dir}/applabel.txt"),
            settings_db: format!("{module_dir}/settings.db"),
            module_dir,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedPolicyFiles {
    pub app_config: Option<String>,
    pub app_label: Option<String>,
    pub settings: Option<Vec<u8>>,
}

impl LoadedPolicyFiles {
    pub fn is_available(&self) -> bool {
        self.app_config.is_some() || self.app_label.is_some() || self.settings.is_some()
    }
}

pub fn load_policy_files(paths: &DaemonPaths) -> Result<LoadedPolicyFiles, DaemonError> {
    Ok(LoadedPolicyFiles {
        app_config: read_optional_text(&paths.app_config)?,
        app_label: read_optional_text(&paths.app_label)?,
        settings: read_optional_bytes(&paths.settings_db)?,
    })
}

pub fn load_policy_files_recovering(paths: &DaemonPaths) -> LoadedPolicyFiles {
    LoadedPolicyFiles {
        app_config: read_optional_text(&paths.app_config).ok().flatten(),
        app_label: read_optional_text(&paths.app_label).ok().flatten(),
        settings: read_optional_bytes(&paths.settings_db).ok().flatten(),
    }
}

pub fn serialize_manager_app_config(lines: &[String]) -> Vec<u8> {
    lines.join("\n").into_bytes()
}

pub fn parse_manager_app_config(payload: &[u8]) -> Result<Vec<String>, DaemonError> {
    let text = std::str::from_utf8(payload)
        .map_err(|error| DaemonError::config(format!("app config is not utf-8: {error}")))?;
    Ok(text
        .lines()
        .filter_map(crate::config::migration::normalize_legacy_line)
        .collect())
}

fn read_optional_text(path: impl AsRef<Path>) -> Result<Option<String>, DaemonError> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(None);
    }

    Ok(Some(fs::read_to_string(path)?))
}

fn read_optional_bytes(path: impl AsRef<Path>) -> Result<Option<Vec<u8>>, DaemonError> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(None);
    }

    Ok(Some(fs::read(path)?))
}
