use std::{collections::BTreeMap, fs, process::Command};

use crate::{
    app::{
        compatibility::CompatibilityBaseline,
        error::DaemonError,
        freezer_backend::{
            mark_processes_frozen, mark_processes_running, BackendEnvironment, DecisionAction,
            FreezeDecision, SystemAwareCgroupBinderBackend,
        },
        health::ModuleHealth,
        operation_log::OperationLog,
        package_inventory::{parse_cmd_package_list, reconcile_uid, PackageRecord},
    },
    config::{
        loader::{load_policy_files, load_policy_files_recovering, DaemonPaths, LoadedPolicyFiles},
        migration::parse_legacy_policy_line,
    },
    domain::{
        capability::ControlCapability,
        operation::{ControlAction, ControlOperation, OperationResult},
        policy::{FallbackAction, ForegroundStrategy, FreezeMode, FreezePolicy, ManagedApp},
        runtime::RuntimeProcess,
    },
    protocol::{
        manager_v1::{
            encode_app_config, encode_xposed_config_payload, handle_read_only_command,
            normalize_settings, ManagerAppConfigRecord, ManagerCommand, ReadOnlyState,
        },
        manager_v2::{
            capability_report_json, compatibility_report_json, health_report_json,
            operation_log_json, self_check_json,
        },
        xposed::{classify_bridge_error, classify_hook_health_payload},
    },
    sys::{socket, xposed_bridge},
};

pub fn run() -> Result<(), DaemonError> {
    let mut state = startup_read_only_state();
    if let Err(error) = sync_loaded_config_to_hook(&mut state, xposed_bridge::set_config) {
        append_log_once(
            &mut state.log,
            &format!("hook config sync failed: {error}\n"),
        );
    }
    socket::run_manager_server_forever(state)
}

pub fn startup_read_only_state() -> ReadOnlyState {
    startup_read_only_state_from_paths(&DaemonPaths::from_module_dir("/data/adb/modules/freezeit"))
}

pub fn startup_read_only_state_from_paths(paths: &DaemonPaths) -> ReadOnlyState {
    let mut state = ReadOnlyState::default();
    state.settings_path = Some(paths.settings_db.clone());
    state.app_config_path = Some(paths.app_config.clone());
    apply_module_prop(&mut state, &format!("{}/module.prop", paths.module_dir));
    state.changelog = read_first_existing_text(&[
        format!("{}/CHANGELOG.md", paths.module_dir),
        format!("{}/changelog.md", paths.module_dir),
        format!("{}/changelog.txt", paths.module_dir),
    ])
    .unwrap_or_default();
    if let Some(log) = read_first_existing_text(&[
        format!("{}/boot.log", paths.module_dir),
        format!("{}/freezeit.log", paths.module_dir),
        format!("{}/daemon.log", paths.module_dir),
    ]) {
        state.log = sanitize_startup_log(&log);
        if !state.log.ends_with('\n') {
            state.log.push('\n');
        }
    }
    let policy = load_policy_files_recovering(paths);
    state.settings = normalize_settings(policy.settings);
    let package_records = load_package_records();
    state.app_config =
        load_manager_app_config_records(policy.app_config.as_deref(), &package_records);
    state.android_version = detect_android_version();
    state.kernel_version = detect_kernel_version();
    state.cluster_num = detect_cpu_cluster_count();
    state.ext_memory_mib = detect_ext_memory_mib();
    state.work_mode = "FreezerV2 / Rust daemon".to_owned();
    state.daemon_health = "active".to_owned();
    append_daemon_status_log(&mut state);
    let capabilities =
        SystemAwareCgroupBinderBackend::new(BackendEnvironment::default()).discover_capabilities();
    state.capability_report_json = capability_report_json(&capabilities);
    state.compatibility_report_json = compatibility_report_json(
        &CompatibilityBaseline::target_cph2653_android16(),
        &capabilities,
    );
    match xposed_bridge::query_hook_health() {
        Ok(payload) => {
            let status = classify_hook_health_payload(&payload);
            state.hook_health = status.health_label().to_owned();
            state.xp_log = payload;
        }
        Err(error) => {
            let status = classify_bridge_error(&error);
            state.hook_health = status.health_label().to_owned();
            state.xp_log = format!("hook bridge {}", status.health_label());
        }
    }
    state.health_report_json = format!(
        "{{\"status\":\"{}\",\"daemonReady\":true,\"hookHealth\":\"{}\"}}",
        if state.hook_health == "active" {
            "active"
        } else {
            "degraded"
        },
        state.hook_health
    );
    state.self_check_json = format!("{{\"controlAllowed\":{}}}", state.hook_health == "active");
    state
}

pub fn sync_loaded_config_to_hook(
    state: &mut ReadOnlyState,
    set_app_config: impl FnOnce(&[u8]) -> Result<bool, DaemonError>,
) -> Result<bool, DaemonError> {
    let app_config_payload = encode_app_config(&state.app_config);
    let xposed_payload = encode_xposed_config_payload(&state.settings, &app_config_payload)?;
    let synced = set_app_config(&xposed_payload)?;
    state.hook_config_synced = synced;
    if synced {
        append_log_once(
            &mut state.log,
            &format!(
                "hook config synced: managed_apps={} settings={}\n",
                state.app_config.len(),
                state.settings.len()
            ),
        );
    } else {
        append_log_once(&mut state.log, "hook config sync rejected\n");
    }
    Ok(synced)
}

fn sanitize_startup_log(log: &str) -> String {
    let mut sanitized = String::new();
    let mut skipping_rom_mismatch = false;
    for line in log.lines() {
        if line.contains("WARNING ROM fingerprint mismatch") {
            skipping_rom_mismatch = true;
            continue;
        }
        if skipping_rom_mismatch {
            if line.starts_with(char::is_whitespace)
                || line.starts_with("baseline=")
                || line.starts_with("device=")
            {
                continue;
            }
            skipping_rom_mismatch = false;
        }
        sanitized.push_str(line);
        sanitized.push('\n');
    }
    sanitized
}

fn append_daemon_status_log(state: &mut ReadOnlyState) {
    state.log.push_str(&format!(
        "daemon active: apps={} settings={} android={} kernel={}\n",
        state.app_config.len(),
        state.settings.len(),
        state.android_version,
        state.kernel_version
    ));
}

fn append_log_once(log: &mut String, line: &str) {
    let trimmed = line.trim_end();
    if !log.lines().any(|existing| existing == trimmed) {
        log.push_str(line);
        if !line.ends_with('\n') {
            log.push('\n');
        }
    }
}

fn apply_module_prop(state: &mut ReadOnlyState, path: &str) {
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim();
        match key.trim() {
            "id" => state.module_id = value.to_owned(),
            "name" => state.module_name = value.to_owned(),
            "version" => state.version = value.to_owned(),
            "versionCode" => {
                if let Ok(version_code) = value.parse() {
                    state.version_code = version_code;
                }
            }
            "author" => state.author = value.to_owned(),
            _ => {}
        }
    }
}

fn read_first_existing_text(paths: &[String]) -> Option<String> {
    paths.iter().find_map(|path| fs::read_to_string(path).ok())
}

fn load_package_records() -> Vec<PackageRecord> {
    for (program, args) in [
        ("cmd", ["package", "list", "packages", "-U"]),
        ("pm", ["list", "packages", "-U", ""]),
    ] {
        let args = args
            .iter()
            .copied()
            .filter(|arg| !arg.is_empty())
            .collect::<Vec<_>>();
        if let Ok(output) = Command::new(program).args(args).output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                let records = parse_cmd_package_list(&text);
                if !records.is_empty() {
                    return records;
                }
            }
        }
    }
    Vec::new()
}

fn load_manager_app_config_records(
    app_config: Option<&str>,
    package_records: &[PackageRecord],
) -> Vec<ManagerAppConfigRecord> {
    let package_uids = package_records
        .iter()
        .map(|record| (record.package_name.as_str(), record.uid))
        .collect::<BTreeMap<_, _>>();
    app_config
        .into_iter()
        .flat_map(str::lines)
        .filter_map(parse_legacy_policy_line)
        .filter_map(|record| {
            let uid = parse_legacy_uid_token(&record.package_or_uid)
                .or_else(|| package_uids.get(record.package_or_uid.as_str()).copied())?;
            Some(ManagerAppConfigRecord {
                uid,
                mode: record.mode,
                permissive: record.permissive,
            })
        })
        .collect()
}

fn parse_legacy_uid_token(token: &str) -> Option<u32> {
    if let Ok(uid) = token.parse::<u32>() {
        return Some(uid);
    }
    token
        .split_once("uid")
        .and_then(|(_, uid)| uid.parse::<u32>().ok())
}

fn detect_android_version() -> String {
    command_output("getprop", &["ro.build.version.release"])
        .filter(|version| !version.is_empty())
        .unwrap_or_else(|| std::env::consts::OS.to_owned())
}

fn detect_kernel_version() -> String {
    command_output("uname", &["-r"])
        .filter(|version| !version.is_empty())
        .or_else(|| {
            fs::read_to_string("/proc/version")
                .ok()
                .and_then(|text| text.split_whitespace().nth(2).map(str::to_owned))
        })
        .unwrap_or_else(|| std::env::consts::ARCH.to_owned())
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn detect_cpu_cluster_count() -> u32 {
    let Ok(entries) = fs::read_dir("/sys/devices/system/cpu") else {
        return 1;
    };
    let core_count = entries
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| {
            name.strip_prefix("cpu").is_some_and(|suffix| {
                !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit())
            })
        })
        .count() as u32;
    core_count.max(1)
}

fn detect_ext_memory_mib() -> u32 {
    fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|text| {
            text.lines().find_map(|line| {
                let mut parts = line.split_whitespace();
                (parts.next()? == "SwapTotal:")
                    .then(|| parts.next()?.parse::<u32>().ok().map(|kb| kb / 1024))
                    .flatten()
            })
        })
        .unwrap_or(0)
}

pub fn handle_manager_read_only(
    command: ManagerCommand,
    state: &ReadOnlyState,
) -> Result<Vec<u8>, DaemonError> {
    handle_read_only_command(command, state)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticState {
    pub health: ModuleHealth,
    pub capabilities: Vec<ControlCapability>,
    pub operation_log: OperationLog,
}

pub fn read_only_state_with_diagnostics(diagnostics: &DiagnosticState) -> ReadOnlyState {
    let mut state = ReadOnlyState::default();
    state.health_report_json = health_report_json(&diagnostics.health);
    state.capability_report_json = capability_report_json(&diagnostics.capabilities);
    state.compatibility_report_json = compatibility_report_json(
        &CompatibilityBaseline::target_cph2653_android16(),
        &diagnostics.capabilities,
    );
    state.operation_log_json = operation_log_json(&diagnostics.operation_log);
    state.operation_log_text = diagnostics.operation_log.to_legacy_text();
    state.self_check_json = self_check_json(&diagnostics.health, &diagnostics.capabilities);
    state
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeControlState {
    pub operation_log: OperationLog,
    frozen_apps: std::collections::BTreeSet<(String, u32)>,
    pending_freezes: BTreeMap<(String, u32), u128>,
    next_operation_id: u64,
}

impl Default for RuntimeControlState {
    fn default() -> Self {
        Self {
            operation_log: OperationLog::new(128),
            frozen_apps: std::collections::BTreeSet::new(),
            pending_freezes: BTreeMap::new(),
            next_operation_id: 1,
        }
    }
}

pub fn run_control_pass(
    state: &mut RuntimeControlState,
    app_config: &[ManagerAppConfigRecord],
    mut discover_processes: impl FnMut(&str, u32) -> Result<Vec<RuntimeProcess>, DaemonError>,
    mut freeze_process: impl FnMut(&RuntimeProcess) -> Result<(), DaemonError>,
    mut unfreeze_process: impl FnMut(&RuntimeProcess) -> Result<(), DaemonError>,
    foreground_uids: &[u32],
    timestamp_ms: u128,
) -> Result<(), DaemonError> {
    run_control_pass_with_settings(
        state,
        app_config,
        &[],
        &mut discover_processes,
        &mut freeze_process,
        &mut unfreeze_process,
        foreground_uids,
        timestamp_ms,
    )
}

pub fn run_control_pass_with_settings(
    state: &mut RuntimeControlState,
    app_config: &[ManagerAppConfigRecord],
    settings: &[u8],
    mut discover_processes: impl FnMut(&str, u32) -> Result<Vec<RuntimeProcess>, DaemonError>,
    mut freeze_process: impl FnMut(&RuntimeProcess) -> Result<(), DaemonError>,
    mut unfreeze_process: impl FnMut(&RuntimeProcess) -> Result<(), DaemonError>,
    foreground_uids: &[u32],
    timestamp_ms: u128,
) -> Result<(), DaemonError> {
    for record in app_config {
        if !is_control_policy_mode(record.mode) {
            continue;
        }
        let fallback_package_name = format!("uid{}", record.uid);
        let processes = discover_processes(&fallback_package_name, record.uid)?;
        if processes.is_empty() {
            continue;
        }
        let package_name = processes
            .first()
            .map(|process| process.package_name.clone())
            .unwrap_or_else(|| fallback_package_name.clone());
        let app = managed_app_from_record(&package_name, record.uid);
        let policy = policy_from_record(record);
        let identity = (package_name.clone(), record.uid);

        if foreground_uids.contains(&record.uid) {
            state.pending_freezes.remove(&identity);
            if state.frozen_apps.remove(&identity) {
                for process in &processes {
                    unfreeze_process(process)?;
                }
                let mut operation =
                    SystemAwareCgroupBinderBackend::new(backend_environment(&processes))
                        .unfreeze_operation(&app, &processes, "foreground uid active");
                operation.backend = backend_name(&processes).to_owned();
                stamp_operation(&mut operation, state, timestamp_ms);
                state.operation_log.push(operation);
            }
            continue;
        }

        if state.frozen_apps.contains(&identity) {
            continue;
        }

        let delay_ms = manager_policy_delay_ms(record, settings);
        if delay_ms > 0 {
            match state.pending_freezes.get(&identity).copied() {
                Some(due_at_ms) if timestamp_ms < due_at_ms => continue,
                Some(_) => {}
                None => {
                    state
                        .pending_freezes
                        .insert(identity.clone(), timestamp_ms + u128::from(delay_ms));
                    let mut operation = ControlOperation {
                        operation_id: 0,
                        timestamp_ms: 0,
                        package_name,
                        uid: record.uid,
                        pid_list: processes.iter().map(|process| process.pid).collect(),
                        action: ControlAction::Postpone,
                        backend: backend_name(&processes).to_owned(),
                        reason: format!("pending freeze delay {delay_ms}ms"),
                        result: OperationResult::Postponed,
                        details: operation_details(&processes),
                    };
                    stamp_operation(&mut operation, state, timestamp_ms);
                    state.operation_log.push(operation);
                    continue;
                }
            }
        }

        let mut pending_processes = processes.clone();
        for process in &mut pending_processes {
            process.control_state = crate::domain::runtime::ControlState::PendingFreeze;
        }
        let backend = SystemAwareCgroupBinderBackend::new(backend_environment(&pending_processes));
        let decision = backend.can_freeze(&app, &policy, &pending_processes);
        let (action, result) = match decision.action {
            DecisionAction::Freeze => {
                let mut freeze_error = None;
                for process in &pending_processes {
                    if let Err(error) = freeze_process(process) {
                        freeze_error = Some(error);
                        break;
                    }
                }
                if let Some(error) = freeze_error {
                    let fallback = backend.fallback_after_freeze_apply_error(&policy, &error);
                    let (action, result) = operation_from_fallback(fallback.action);
                    let mut operation = ControlOperation {
                        operation_id: 0,
                        timestamp_ms: 0,
                        package_name,
                        uid: record.uid,
                        pid_list: pending_processes
                            .iter()
                            .map(|process| process.pid)
                            .collect(),
                        action,
                        backend: backend_name(&pending_processes).to_owned(),
                        reason: fallback.reason,
                        result,
                        details: operation_details(&pending_processes),
                    };
                    stamp_operation(&mut operation, state, timestamp_ms);
                    state.operation_log.push(operation);
                    continue;
                }

                let post_freeze_processes = discover_processes(&fallback_package_name, record.uid)?;
                let original_pids = pending_processes
                    .iter()
                    .map(|process| process.pid)
                    .collect::<std::collections::BTreeSet<_>>();
                let new_pids = post_freeze_processes
                    .iter()
                    .filter(|process| {
                        process.uid == record.uid && !original_pids.contains(&process.pid)
                    })
                    .map(|process| process.pid)
                    .collect::<Vec<_>>();
                if !new_pids.is_empty() {
                    for process in &post_freeze_processes {
                        let _ = unfreeze_process(process);
                    }
                    state.pending_freezes.remove(&identity);
                    let mut operation = ControlOperation {
                        operation_id: 0,
                        timestamp_ms: 0,
                        package_name,
                        uid: record.uid,
                        pid_list: post_freeze_processes
                            .iter()
                            .map(|process| process.pid)
                            .collect(),
                        action: ControlAction::Freeze,
                        backend: backend_name(&post_freeze_processes).to_owned(),
                        reason: format!(
                            "{}; new same-uid process appeared after freeze",
                            decision.reason
                        ),
                        result: OperationResult::Partial,
                        details: format!(
                            "{} new_pids={new_pids:?}",
                            operation_details(&post_freeze_processes)
                        ),
                    };
                    stamp_operation(&mut operation, state, timestamp_ms);
                    state.operation_log.push(operation);
                    continue;
                }
                state.frozen_apps.insert(identity);
                state
                    .pending_freezes
                    .remove(&(package_name.clone(), record.uid));
                (ControlAction::Freeze, OperationResult::Success)
            }
            DecisionAction::Postpone => (ControlAction::Postpone, OperationResult::Postponed),
            DecisionAction::AlternateFreezer => (ControlAction::Fallback, OperationResult::Skipped),
            DecisionAction::Signal => {
                for process in &pending_processes {
                    freeze_process(process)?;
                }
                state.frozen_apps.insert(identity);
                state
                    .pending_freezes
                    .remove(&(package_name.clone(), record.uid));
                (ControlAction::Freeze, OperationResult::Success)
            }
            DecisionAction::Terminate => (ControlAction::Terminate, OperationResult::Skipped),
            DecisionAction::Skip => (ControlAction::Skip, OperationResult::Skipped),
        };

        let mut operation = ControlOperation {
            operation_id: 0,
            timestamp_ms: 0,
            package_name,
            uid: record.uid,
            pid_list: pending_processes
                .iter()
                .map(|process| process.pid)
                .collect(),
            action,
            backend: backend_name(&pending_processes).to_owned(),
            reason: decision.reason,
            result,
            details: operation_details(&pending_processes),
        };
        stamp_operation(&mut operation, state, timestamp_ms);
        state.operation_log.push(operation);
    }

    Ok(())
}

fn operation_details(processes: &[RuntimeProcess]) -> String {
    let mut details = format!("process_count={}", processes.len());
    let evidence = processes
        .iter()
        .filter_map(|process| {
            process
                .binder_state
                .as_ref()
                .map(|state| format!("pid{}:{state}", process.pid))
        })
        .collect::<Vec<_>>();
    if !evidence.is_empty() {
        details.push_str(" idle_evidence=");
        details.push_str(&evidence.join("|"));
    }
    details
}

pub fn is_control_policy_mode(mode: i32) -> bool {
    matches!(mode, 10 | 20 | 21 | 30 | 31)
}

fn backend_environment(processes: &[RuntimeProcess]) -> BackendEnvironment {
    BackendEnvironment {
        cgroup_available: !processes.is_empty()
            && processes
                .iter()
                .all(|process| process.cgroup_freeze_path.is_some()),
        binder_available: true,
        network_available: true,
        wakelock_available: true,
        screen_state_available: true,
        hook_fresh: true,
    }
}

fn backend_name(processes: &[RuntimeProcess]) -> &'static str {
    if !processes.is_empty()
        && processes
            .iter()
            .all(|process| process.cgroup_freeze_path.is_some())
    {
        "cgroup.freeze"
    } else {
        "signal-control-pass"
    }
}

fn stamp_operation(
    operation: &mut ControlOperation,
    state: &mut RuntimeControlState,
    timestamp_ms: u128,
) {
    operation.operation_id = state.next_operation_id;
    state.next_operation_id += 1;
    operation.timestamp_ms = timestamp_ms;
}

fn managed_app_from_record(package_name: &str, uid: u32) -> ManagedApp {
    ManagedApp {
        package_name: package_name.to_owned(),
        user_id: 0,
        uid,
        label: package_name.to_owned(),
        is_system_app: false,
        protected_reason: None,
        policy_id: "manager-v1".to_owned(),
        last_seen_baseline: "runtime".to_owned(),
    }
}

fn policy_from_record(record: &ManagerAppConfigRecord) -> FreezePolicy {
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
        allow_network_restriction: record.mode == 31,
        allow_wakelock_restriction: false,
        fallback_strategy: vec![FallbackAction::Signal, FallbackAction::Skip],
        updated_at_ms: 0,
    }
}

fn manager_policy_delay_ms(record: &ManagerAppConfigRecord, settings: &[u8]) -> u64 {
    let seconds = match record.mode {
        10 => settings.get(4).copied().unwrap_or(0),
        20 | 21 | 30 | 31 => settings.get(2).copied().unwrap_or(0),
        _ => 0,
    };
    u64::from(seconds) * 1000
}

fn operation_from_fallback(action: DecisionAction) -> (ControlAction, OperationResult) {
    match action {
        DecisionAction::Freeze => (ControlAction::Freeze, OperationResult::Success),
        DecisionAction::Postpone => (ControlAction::Postpone, OperationResult::Postponed),
        DecisionAction::AlternateFreezer | DecisionAction::Signal => {
            (ControlAction::Fallback, OperationResult::Skipped)
        }
        DecisionAction::Terminate => (ControlAction::Terminate, OperationResult::Skipped),
        DecisionAction::Skip => (ControlAction::Skip, OperationResult::Skipped),
    }
}

pub fn run_manager_server_once(state: &ReadOnlyState) -> Result<(), DaemonError> {
    let listener = socket::bind_manager_listener()?;
    let (stream, _) = listener.accept()?;
    let mut state = state.clone();
    socket::handle_single_manager_stream(stream, &mut state)
}

pub fn load_policy_with_retries(
    paths: &DaemonPaths,
    attempts: usize,
) -> Result<LoadedPolicyFiles, DaemonError> {
    let attempts = attempts.max(1);
    let mut last_result = None;

    for _ in 0..attempts {
        let loaded = load_policy_files(paths)?;
        if loaded.is_available() {
            return Ok(loaded);
        }
        last_result = Some(loaded);
    }

    Ok(last_result.unwrap_or(LoadedPolicyFiles {
        app_config: None,
        app_label: None,
        settings: None,
    }))
}

pub fn decide_freeze(
    app: &crate::domain::policy::ManagedApp,
    policy: &crate::domain::policy::FreezePolicy,
    processes: &[crate::domain::runtime::RuntimeProcess],
) -> FreezeDecision {
    SystemAwareCgroupBinderBackend::new(BackendEnvironment::default())
        .can_freeze(app, policy, processes)
}

pub fn decide_freeze_after_reconciliation(
    app: &crate::domain::policy::ManagedApp,
    current_package: &PackageRecord,
    policy: &crate::domain::policy::FreezePolicy,
    processes: &[crate::domain::runtime::RuntimeProcess],
) -> Result<FreezeDecision, DaemonError> {
    reconcile_uid(app, current_package).map_err(DaemonError::system)?;
    Ok(decide_freeze(app, policy, processes))
}

pub fn mark_frozen(processes: &mut [crate::domain::runtime::RuntimeProcess]) {
    mark_processes_frozen(processes);
}

pub fn mark_running(processes: &mut [crate::domain::runtime::RuntimeProcess]) {
    mark_processes_running(processes);
}

pub fn recover_after_restart(
    operation_id: u64,
    timestamp_ms: u128,
    package_name: &str,
    uid: u32,
    processes: &[crate::domain::runtime::RuntimeProcess],
) -> ControlOperation {
    ControlOperation {
        operation_id,
        timestamp_ms,
        package_name: package_name.to_owned(),
        uid,
        pid_list: processes.iter().map(|process| process.pid).collect(),
        action: ControlAction::Recover,
        backend: "restart-reconciliation".to_owned(),
        reason: "daemon restart reconciliation".to_owned(),
        result: OperationResult::Success,
        details: format!(
            "observed {} process(es) before new control",
            processes.len()
        ),
    }
}
