use freezeit_daemon::{
    config::migration::{
        migrate_legacy_files, migrate_legacy_policy, parse_legacy_label_line,
        parse_legacy_policy_line,
    },
    domain::policy::{ForegroundStrategy, FreezeMode, FreezePolicy},
    protocol::manager_v1::{decode_app_config, encode_app_config, ManagerAppConfigRecord},
};

#[test]
fn migrates_legacy_policy_line_to_freeze_policy() {
    let record = parse_legacy_policy_line("com.example.app 31 1").expect("legacy line");
    let policy = migrate_legacy_policy(&record);

    assert_eq!(record.package_or_uid, "com.example.app");
    assert!(matches!(
        policy,
        FreezePolicy::Selected {
            mode: FreezeMode::Freeze,
            foreground_strategy: ForegroundStrategy::Permissive,
            allow_network_restriction: true,
            ..
        }
    ));
}

#[test]
fn migrates_legacy_labels_and_preserves_settings_bytes() {
    let label =
        parse_legacy_label_line("com.example.app####Example App").expect("legacy label line");
    assert_eq!(label.package_name, "com.example.app");
    assert_eq!(label.label, "Example App");

    let migrated = migrate_legacy_files(
        "com.example.app 40 1\ncom.example.other 31 0\n",
        "com.example.app####Example App\n",
        &[8, 0, 10, 4],
    );

    assert_eq!(migrated.policies.len(), 2);
    assert_eq!(migrated.labels, vec![label]);
    assert_eq!(migrated.settings, vec![8, 0, 10, 4]);
}

#[test]
fn manager_app_config_records_round_trip_as_legacy_binary_triples() {
    let records = vec![ManagerAppConfigRecord {
        uid: 10_123,
        mode: 30,
        permissive: true,
    }];

    let payload = encode_app_config(&records);
    assert_eq!(payload.len(), 12);
    assert_eq!(decode_app_config(&payload).expect("decode"), records);
}
