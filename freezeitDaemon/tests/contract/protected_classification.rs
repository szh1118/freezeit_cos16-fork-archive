use freezeit_daemon::{
    app::package_inventory::{
        build_inventory, build_inventory_with_context, parse_cmd_package_list,
        protected_reason_for, PackageRecord, ProtectedPackageContext,
    },
    domain::policy::ProtectedReason,
};

#[test]
fn protects_manager_root_hook_and_system_critical_packages_by_default() {
    assert_eq!(
        protected_reason_for("io.github.jark006.freezeit", false),
        Some(ProtectedReason::Manager)
    );
    assert_eq!(
        protected_reason_for("com.topjohnwu.magisk", false),
        Some(ProtectedReason::RootManager)
    );
    assert_eq!(
        protected_reason_for("org.lsposed.manager", false),
        Some(ProtectedReason::HookManager)
    );
    assert_eq!(
        protected_reason_for("com.android.systemui", true),
        Some(ProtectedReason::SystemCritical)
    );
}

#[test]
fn parses_cmd_package_list_with_uid_output() {
    let records = parse_cmd_package_list("package:com.example.app uid:10123\n");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].package_name, "com.example.app");
    assert_eq!(records[0].uid, 10_123);
}

#[test]
fn package_inventory_applies_protected_defaults() {
    let inventory = build_inventory(vec![PackageRecord {
        package_name: "io.github.jark006.freezeit".to_owned(),
        user_id: 0,
        uid: 10_570,
        label: "Freezeit".to_owned(),
        is_system_app: false,
    }]);

    let app = inventory
        .get(&("io.github.jark006.freezeit".to_owned(), 0))
        .expect("manager app");
    assert!(app.is_protected());
}

#[test]
fn device_role_context_protects_launcher_and_input_method_packages() {
    let inventory = build_inventory_with_context(
        vec![
            PackageRecord {
                package_name: "com.example.launcher".to_owned(),
                user_id: 0,
                uid: 10_200,
                label: "Launcher".to_owned(),
                is_system_app: false,
            },
            PackageRecord {
                package_name: "com.example.ime".to_owned(),
                user_id: 0,
                uid: 10_201,
                label: "Keyboard".to_owned(),
                is_system_app: false,
            },
        ],
        &ProtectedPackageContext {
            launcher_packages: vec!["com.example.launcher".to_owned()],
            input_method_packages: vec!["com.example.ime".to_owned()],
            root_manager_packages: Vec::new(),
            hook_manager_packages: Vec::new(),
        },
    );

    assert_eq!(
        inventory
            .get(&("com.example.launcher".to_owned(), 0))
            .and_then(|app| app.protected_reason),
        Some(ProtectedReason::Launcher)
    );
    assert_eq!(
        inventory
            .get(&("com.example.ime".to_owned(), 0))
            .and_then(|app| app.protected_reason),
        Some(ProtectedReason::InputMethod)
    );
}
