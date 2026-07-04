use freezeit_daemon::protocol::xposed::{
    classify_hook_health_payload, encode_frame, parse_foreground_uid_payload, parse_frame,
    HookBridgeStatus, XposedCommand, FREEZEIT_COMMAND_BASE, HEADER_LEN,
};

#[test]
fn command_constants_match_contract_base() {
    assert_eq!(
        XposedCommand::GetForeground as i32,
        FREEZEIT_COMMAND_BASE + 1
    );
    assert_eq!(
        XposedCommand::GetHookHealth as i32,
        FREEZEIT_COMMAND_BASE + 70
    );
    assert_eq!(
        XposedCommand::GetSystemFreezerHints as i32,
        FREEZEIT_COMMAND_BASE + 72
    );
}

#[test]
fn round_trips_xposed_frame() {
    let payload = br#"{"ready":true}"#;
    let bytes = encode_frame(XposedCommand::GetHookHealth, payload).expect("frame encodes");

    assert_eq!(bytes.len(), HEADER_LEN + payload.len());

    let frame = parse_frame(&bytes).expect("frame parses");
    assert_eq!(frame.command, XposedCommand::GetHookHealth);
    assert_eq!(frame.payload, payload);
}

#[test]
fn rejects_negative_payload_length() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(XposedCommand::GetXpLog as i32).to_le_bytes());
    bytes.extend_from_slice(&(-1_i32).to_le_bytes());

    let error = parse_frame(&bytes).expect_err("negative length must fail");
    assert!(error.to_string().contains("negative"));
}

#[test]
fn classifies_hook_health_payloads() {
    assert_eq!(
        classify_hook_health_payload(r#"{"status":"active","system_server_ready":true}"#),
        HookBridgeStatus::Active
    );
    assert!(matches!(
        classify_hook_health_payload(r#"{"status":"degraded","system_server_ready":false}"#),
        HookBridgeStatus::Degraded(_)
    ));
}

#[test]
fn parses_legacy_foreground_uid_payload() {
    let mut payload = Vec::new();
    payload.extend_from_slice(&2_i32.to_le_bytes());
    payload.extend_from_slice(&10_555_i32.to_le_bytes());
    payload.extend_from_slice(&10_386_i32.to_le_bytes());

    assert_eq!(
        parse_foreground_uid_payload(&payload).expect("foreground payload parses"),
        vec![10_555, 10_386]
    );
}
