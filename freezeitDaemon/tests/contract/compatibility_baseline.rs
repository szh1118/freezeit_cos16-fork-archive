use freezeit_daemon::{
    app::compatibility::CompatibilityBaseline,
    domain::capability::{CapabilityName, ControlCapability},
    protocol::manager_v2::compatibility_report_json,
};

#[test]
fn compatibility_report_contains_target_identity_and_capabilities() {
    let baseline = CompatibilityBaseline::target_cph2653_android16();
    let json = baseline.compatibility_json(&[ControlCapability::missing(
        CapabilityName::LsposedSystemServer,
        "hook unavailable",
    )]);

    assert!(json.contains("\"deviceModel\":\"CPH2653\""));
    assert!(json.contains("\"androidVersion\":\"16\""));
    assert!(json.contains("\"sdk\":36"));
    assert!(json.contains("\"lsposed_system_server\""));
    assert!(json.contains("\"status\":\"missing\""));
}

#[test]
fn compatibility_report_disables_control_when_required_capability_is_missing() {
    let baseline = CompatibilityBaseline::target_cph2653_android16();

    assert!(!baseline.allows_control(&[ControlCapability::missing(
        CapabilityName::CgroupV2Freezer,
        "missing cgroup.freeze",
    )]));
}

#[test]
fn manager_v2_exposes_compatibility_report_json() {
    let baseline = CompatibilityBaseline::target_cph2653_android16();
    let json = compatibility_report_json(&baseline, &[]);

    assert!(json.contains("\"deviceModel\":\"CPH2653\""));
    assert!(json.contains("\"capabilities\":[]"));
}
