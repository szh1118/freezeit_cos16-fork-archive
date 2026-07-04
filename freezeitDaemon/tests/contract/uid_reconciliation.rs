use freezeit_daemon::{
    app::package_inventory::{reconcile_uid, PackageRecord},
    domain::policy::ManagedApp,
};

#[test]
fn rejects_stale_uid_for_same_package_identity() {
    let app = managed_app(10_123);
    let current = PackageRecord {
        package_name: "com.example.app".to_owned(),
        user_id: 0,
        uid: 10_999,
        label: "Example".to_owned(),
        is_system_app: false,
    };

    let error = reconcile_uid(&app, &current).expect_err("stale uid rejected");
    assert!(error.contains("uid changed"));
}

#[test]
fn accepts_matching_package_name_user_and_uid() {
    let app = managed_app(10_123);
    let current = PackageRecord {
        package_name: "com.example.app".to_owned(),
        user_id: 0,
        uid: 10_123,
        label: "Example".to_owned(),
        is_system_app: false,
    };

    reconcile_uid(&app, &current).expect("matching identity accepted");
}

fn managed_app(uid: u32) -> ManagedApp {
    ManagedApp {
        package_name: "com.example.app".to_owned(),
        user_id: 0,
        uid,
        label: "Example".to_owned(),
        is_system_app: false,
        protected_reason: None,
        policy_id: "default".to_owned(),
        last_seen_baseline: "test".to_owned(),
    }
}
