use std::fs;

use freezeit_daemon::{
    app::controller::load_policy_with_retries,
    config::loader::{load_policy_files, load_policy_files_recovering, DaemonPaths},
};

#[test]
fn loads_existing_legacy_policy_files() {
    let temp = tempfile::tempdir().expect("tempdir");
    let module_dir = temp.path().to_string_lossy().to_string();
    fs::write(temp.path().join("appcfg.txt"), "10000com.example.app").expect("appcfg");
    fs::write(temp.path().join("applabel.txt"), "com.example.app Example").expect("applabel");
    fs::write(temp.path().join("settings.db"), [1_u8, 2, 3]).expect("settings");

    let paths = DaemonPaths::from_module_dir(module_dir);
    let loaded = load_policy_files(&paths).expect("policy loads");

    assert_eq!(loaded.app_config.as_deref(), Some("10000com.example.app"));
    assert_eq!(loaded.app_label.as_deref(), Some("com.example.app Example"));
    assert_eq!(loaded.settings.as_deref(), Some(&[1_u8, 2, 3][..]));
}

#[test]
fn missing_policy_files_are_not_fatal_during_unlock_retry() {
    let temp = tempfile::tempdir().expect("tempdir");
    let paths = DaemonPaths::from_module_dir(temp.path().to_string_lossy().to_string());

    let loaded = load_policy_with_retries(&paths, 2).expect("missing files do not fail");

    assert!(!loaded.is_available());
}

#[test]
fn recovering_loader_keeps_available_files_when_text_config_is_corrupt() {
    let temp = tempfile::tempdir().expect("tempdir");
    let paths = DaemonPaths::from_module_dir(temp.path().to_string_lossy().to_string());
    fs::write(&paths.app_config, [0xff, 0xfe]).expect("corrupt app config");
    fs::write(&paths.settings_db, [1_u8, 2, 3, 4]).expect("settings");

    let loaded = load_policy_files_recovering(&paths);

    assert!(loaded.app_config.is_none());
    assert_eq!(loaded.settings.as_deref(), Some(&[1_u8, 2, 3, 4][..]));
}
