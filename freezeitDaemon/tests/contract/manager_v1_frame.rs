use freezeit_daemon::protocol::manager_v1::{
    encode_frame, parse_frame, ManagerCommand, HEADER_LEN,
};

#[test]
fn round_trips_empty_v1_frame() {
    let bytes = encode_frame(ManagerCommand::GetPropInfo, &[]).expect("frame encodes");

    assert_eq!(bytes.len(), HEADER_LEN);

    let frame = parse_frame(&bytes).expect("frame parses");
    assert_eq!(frame.command, ManagerCommand::GetPropInfo);
    assert!(frame.payload.is_empty());
}

#[test]
fn round_trips_payload_and_checksum() {
    let payload = b"pkg.name=freeze";
    let bytes = encode_frame(ManagerCommand::SetAppCfg, payload).expect("frame encodes");

    let frame = parse_frame(&bytes).expect("frame parses");
    assert_eq!(frame.command, ManagerCommand::SetAppCfg);
    assert_eq!(frame.payload, payload);
}

#[test]
fn rejects_checksum_mismatch() {
    let mut bytes = encode_frame(ManagerCommand::SetSettingsVar, &[1, 2]).expect("frame encodes");
    bytes[5] ^= 0xff;

    let error = parse_frame(&bytes).expect_err("checksum mismatch must fail");
    assert!(error.to_string().contains("checksum"));
}
