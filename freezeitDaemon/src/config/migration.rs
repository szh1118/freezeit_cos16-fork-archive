use crate::app::error::DaemonError;
use crate::domain::policy::{FallbackAction, ForegroundStrategy, FreezeMode, FreezePolicy};

pub fn migrate_legacy_config() -> Result<(), DaemonError> {
    Ok(())
}

pub fn normalize_legacy_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyPolicyRecord {
    pub package_or_uid: String,
    pub mode: i32,
    pub permissive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyLabelRecord {
    pub package_name: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigratedLegacyConfig {
    pub policies: Vec<(LegacyPolicyRecord, FreezePolicy)>,
    pub labels: Vec<LegacyLabelRecord>,
    pub settings: Vec<u8>,
}

pub fn parse_legacy_policy_line(line: &str) -> Option<LegacyPolicyRecord> {
    let line = normalize_legacy_line(line)?;
    let mut parts = line.split_whitespace();
    Some(LegacyPolicyRecord {
        package_or_uid: parts.next()?.to_owned(),
        mode: parts.next()?.parse().ok()?,
        permissive: parts.next().unwrap_or("1") != "0",
    })
}

pub fn parse_legacy_label_line(line: &str) -> Option<LegacyLabelRecord> {
    let line = normalize_legacy_line(line)?;
    let (package_name, label) = line.split_once("####")?;
    let package_name = package_name.trim();
    let label = label.trim();
    if package_name.is_empty() || label.is_empty() {
        return None;
    }

    Some(LegacyLabelRecord {
        package_name: package_name.to_owned(),
        label: label.to_owned(),
    })
}

pub fn migrate_legacy_files(
    app_config: &str,
    app_label: &str,
    settings: &[u8],
) -> MigratedLegacyConfig {
    let policies = app_config
        .lines()
        .filter_map(parse_legacy_policy_line)
        .map(|record| {
            let policy = migrate_legacy_policy(&record);
            (record, policy)
        })
        .collect();
    let labels = app_label
        .lines()
        .filter_map(parse_legacy_label_line)
        .collect();

    MigratedLegacyConfig {
        policies,
        labels,
        settings: settings.to_vec(),
    }
}

pub fn migrate_legacy_policy(record: &LegacyPolicyRecord) -> FreezePolicy {
    let mode = match record.mode {
        10 => FreezeMode::Terminate,
        20 | 21 | 30 | 31 => FreezeMode::Freeze,
        40 | 50 => FreezeMode::Protected,
        _ => FreezeMode::Free,
    };

    FreezePolicy::Selected {
        mode,
        delay_ms: 0,
        foreground_strategy: if record.permissive {
            ForegroundStrategy::Permissive
        } else {
            ForegroundStrategy::Strict
        },
        allow_network_restriction: matches!(record.mode, 21 | 31),
        allow_wakelock_restriction: false,
        fallback_strategy: vec![FallbackAction::Postpone, FallbackAction::Skip],
        updated_at_ms: 0,
    }
}
