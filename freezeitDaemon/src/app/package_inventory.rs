use std::collections::BTreeMap;

use crate::domain::policy::{ManagedApp, ProtectedReason};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRecord {
    pub package_name: String,
    pub user_id: u32,
    pub uid: u32,
    pub label: String,
    pub is_system_app: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProtectedPackageContext {
    pub launcher_packages: Vec<String>,
    pub input_method_packages: Vec<String>,
    pub root_manager_packages: Vec<String>,
    pub hook_manager_packages: Vec<String>,
}

pub fn parse_cmd_package_list(output: &str) -> Vec<PackageRecord> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let package = line.strip_prefix("package:")?;
            let (package_name, uid_text) = package.split_once(" uid:")?;
            let uid = uid_text.parse().ok()?;
            Some(PackageRecord {
                package_name: package_name.to_owned(),
                user_id: 0,
                uid,
                label: package_name.to_owned(),
                is_system_app: false,
            })
        })
        .collect()
}

pub fn build_inventory(records: Vec<PackageRecord>) -> BTreeMap<(String, u32), ManagedApp> {
    build_inventory_with_context(records, &ProtectedPackageContext::default())
}

pub fn build_inventory_with_context(
    records: Vec<PackageRecord>,
    context: &ProtectedPackageContext,
) -> BTreeMap<(String, u32), ManagedApp> {
    records
        .into_iter()
        .map(|record| {
            let protected_reason = protected_reason_for_with_context(
                &record.package_name,
                record.is_system_app,
                context,
            );
            let app = ManagedApp {
                package_name: record.package_name.clone(),
                user_id: record.user_id,
                uid: record.uid,
                label: record.label,
                is_system_app: record.is_system_app,
                protected_reason,
                policy_id: format!("{}:{}", record.user_id, record.package_name),
                last_seen_baseline: String::new(),
            };
            ((record.package_name, record.user_id), app)
        })
        .collect()
}

pub fn protected_reason_for(package_name: &str, is_system_app: bool) -> Option<ProtectedReason> {
    protected_reason_for_with_context(
        package_name,
        is_system_app,
        &ProtectedPackageContext::default(),
    )
}

pub fn protected_reason_for_with_context(
    package_name: &str,
    is_system_app: bool,
    context: &ProtectedPackageContext,
) -> Option<ProtectedReason> {
    if context
        .launcher_packages
        .iter()
        .any(|package| package == package_name)
    {
        return Some(ProtectedReason::Launcher);
    }
    if context
        .input_method_packages
        .iter()
        .any(|package| package == package_name)
    {
        return Some(ProtectedReason::InputMethod);
    }
    if context
        .root_manager_packages
        .iter()
        .any(|package| package == package_name)
    {
        return Some(ProtectedReason::RootManager);
    }
    if context
        .hook_manager_packages
        .iter()
        .any(|package| package == package_name)
    {
        return Some(ProtectedReason::HookManager);
    }

    match package_name {
        "io.github.jark006.freezeit" => Some(ProtectedReason::Manager),
        "com.topjohnwu.magisk" | "io.github.huskydg.magisk" => Some(ProtectedReason::RootManager),
        "org.lsposed.manager" | "io.github.lsposed.manager" => Some(ProtectedReason::HookManager),
        "android" | "com.android.systemui" | "com.android.phone" => {
            Some(ProtectedReason::SystemCritical)
        }
        _ if is_system_app => Some(ProtectedReason::SystemCritical),
        _ => None,
    }
}

pub fn reconcile_uid(app: &ManagedApp, current: &PackageRecord) -> Result<(), String> {
    if app.package_name != current.package_name || app.user_id != current.user_id {
        return Err("package identity mismatch".to_owned());
    }
    if app.uid != current.uid {
        return Err("uid changed; package inventory reconciliation required".to_owned());
    }
    Ok(())
}
