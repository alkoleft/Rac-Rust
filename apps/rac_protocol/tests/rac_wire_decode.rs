use rac_protocol::rac_wire::{decode_rpc_method, encode_varuint, parse_frames};

fn decode_hex_str(input: &str) -> Vec<u8> {
    let s = input.trim();
    assert!(s.len() % 2 == 0, "hex length must be even");
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    for i in (0..bytes.len()).step_by(2) {
        let hi = (bytes[i] as char).to_digit(16).expect("hex hi");
        let lo = (bytes[i + 1] as char).to_digit(16).expect("hex lo");
        out.push(((hi << 4) | lo) as u8);
    }
    out
}

#[test]
fn manager_list_response_method() {
    let hex = include_str!("../../../artifacts/manager_list_response.hex");
    let payload = decode_hex_str(hex);
    let method = decode_rpc_method(&payload).expect("rpc method");
    assert_eq!(method, 0x13);
}

#[test]
fn manager_info_response_method() {
    let hex = include_str!("../../../artifacts/manager_info_response.hex");
    let payload = decode_hex_str(hex);
    let method = decode_rpc_method(&payload).expect("rpc method");
    assert_eq!(method, 0x15);
}

#[test]
fn cluster_admin_list_response_method() {
    let hex = include_str!("../../../artifacts/cluster_admin_list_response.hex");
    let payload = decode_hex_str(hex);
    let method = decode_rpc_method(&payload).expect("rpc method");
    assert_eq!(method, 0x03);
}

#[test]
fn parse_frames_single_payload() {
    let hex = include_str!("../../../artifacts/manager_list_response.hex");
    let payload = decode_hex_str(hex);
    let mut frame = Vec::new();
    frame.push(0x0e);
    frame.extend_from_slice(&encode_varuint(payload.len()));
    frame.extend_from_slice(&payload);

    let frames = parse_frames(&frame, 0).expect("parse frames");
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].opcode, 0x0e);
    assert_eq!(frames[0].payload, payload);
}
