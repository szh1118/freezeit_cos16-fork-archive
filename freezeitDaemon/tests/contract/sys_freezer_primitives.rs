use std::fs;

use freezeit_daemon::{
    domain::capability::CapabilityStatus,
    sys::{
        binder::{
            binder_freezer_ioctl_number, detect_binder_freezer_capability_from_candidates,
            BinderFreezeRequest,
        },
        cgroup::{
            detect_cgroup_v2_freezer_capability, read_freeze_state, write_freeze_state,
            CgroupFreezerPreference, FreezeState,
        },
    },
};

#[test]
fn cgroup_freeze_state_round_trips_through_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let freeze_file = temp.path().join("cgroup.freeze");
    fs::write(&freeze_file, "0").expect("seed freeze file");

    assert_eq!(
        read_freeze_state(&freeze_file).expect("read"),
        FreezeState::Thawed
    );

    write_freeze_state(&freeze_file, FreezeState::Frozen).expect("write frozen");
    assert_eq!(fs::read_to_string(&freeze_file).expect("read raw"), "1");

    write_freeze_state(&freeze_file, FreezeState::Thawed).expect("write thawed");
    assert_eq!(fs::read_to_string(&freeze_file).expect("read raw"), "0");
}

#[test]
fn binder_freezer_ioctl_numbers_are_stable_and_distinct() {
    assert_ne!(
        binder_freezer_ioctl_number(BinderFreezeRequest::Freeze),
        binder_freezer_ioctl_number(BinderFreezeRequest::Unfreeze)
    );
}

#[test]
fn cgroup_v2_detection_reads_controllers_and_prefers_android_app_paths() {
    let temp = tempfile::tempdir().expect("tempdir");
    let cgroup_root = temp.path().join("sys/fs/cgroup");
    let app_freeze = cgroup_root.join("apps/uid_10123/pid_123/cgroup.freeze");
    let system_freeze = cgroup_root.join("system/uid_10123/pid_123/cgroup.freeze");
    fs::create_dir_all(app_freeze.parent().expect("app parent")).expect("mkdir app");
    fs::create_dir_all(system_freeze.parent().expect("system parent")).expect("mkdir system");
    fs::write(cgroup_root.join("cgroup.controllers"), "cpu freezer memory").expect("controllers");
    fs::write(&app_freeze, "0").expect("app freeze");
    fs::write(&system_freeze, "0").expect("system freeze");

    let capability = detect_cgroup_v2_freezer_capability(&cgroup_root).expect("detect");

    assert_eq!(capability.status, CapabilityStatus::Available);
    assert_eq!(
        capability.preference,
        CgroupFreezerPreference::AndroidAppCgroupV2
    );
    assert_eq!(capability.freeze_files.first(), Some(&app_freeze));
    assert!(capability.evidence.contains("cgroup.controllers"));
    assert!(capability.evidence.contains("freezer"));
}

#[test]
fn cgroup_v2_detection_accepts_android_freeze_files_when_root_controllers_omit_freezer() {
    let temp = tempfile::tempdir().expect("tempdir");
    let cgroup_root = temp.path().join("sys/fs/cgroup");
    let app_freeze = cgroup_root.join("apps/uid_10123/pid_123/cgroup.freeze");
    fs::create_dir_all(app_freeze.parent().expect("app parent")).expect("mkdir app");
    fs::write(cgroup_root.join("cgroup.controllers"), "cpu memory").expect("controllers");
    fs::write(&app_freeze, "0").expect("app freeze");

    let capability = detect_cgroup_v2_freezer_capability(&cgroup_root).expect("detect");

    assert_eq!(capability.status, CapabilityStatus::Available);
    assert_eq!(
        capability.preference,
        CgroupFreezerPreference::AndroidAppCgroupV2
    );
    assert!(capability.evidence.contains("contains freezer=false"));
}

#[test]
fn binder_device_without_verified_probe_is_reported_untested_not_available() {
    let temp = tempfile::tempdir().expect("tempdir");
    let binder = temp.path().join("dev/binder");
    fs::create_dir_all(binder.parent().expect("binder parent")).expect("mkdir");
    fs::write(&binder, "").expect("binder device placeholder");
    let candidates = vec![binder];

    let capability = detect_binder_freezer_capability_from_candidates(&candidates);

    assert_eq!(capability.status, CapabilityStatus::Untested);
    assert_eq!(capability.device_path.as_deref(), candidates[0].to_str());
    assert!(capability.evidence.contains("BINDER_FREEZE"));
    assert!(capability.evidence.contains("target probe required"));
}
