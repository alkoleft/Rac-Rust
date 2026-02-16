use std::fs;
use std::path::Path;

use rac_protocol::rac_wire::{
    decode_rpc_method, detect_swp_init_len, parse_frames, scan_prefixed_uuids,
};

fn load_frames<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    let path_ref = path.as_ref();
    match fs::read(path_ref) {
        Ok(data) => Some(data),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!(
                "capture stream not found ({}); skipping test",
                path_ref.display()
            );
            None
        }
        Err(err) => panic!("read stream {}: {err}", path_ref.display()),
    }
}

fn rpc_methods_from_stream(data: &[u8]) -> Vec<u8> {
    let start = detect_swp_init_len(data).unwrap_or(0);
    let frames = parse_frames(data, start).expect("parse frames");
    frames
        .iter()
        .filter(|f| f.opcode == 0x0e)
        .filter_map(|f| decode_rpc_method(&f.payload))
        .collect()
}

#[test]
fn cluster_list_methods_match() {
    let Some(data) =
        load_frames("logs/session_1771103982_389032_127_0_0_1_37378/client_to_server.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&data);
    assert_eq!(methods, vec![0x0b]);

    let Some(s2c) =
        load_frames("logs/session_1771103982_389032_127_0_0_1_37378/server_to_client.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&s2c);
    assert_eq!(methods, vec![0x0c]);
}

#[test]
fn cluster_list_contains_uuid() {
    let Some(s2c) =
        load_frames("logs/session_1771103982_389032_127_0_0_1_37378/server_to_client.stream.bin")
    else {
        return;
    };
    let start = detect_swp_init_len(&s2c).unwrap_or(0);
    let frames = parse_frames(&s2c, start).expect("parse frames");
    let payload = frames
        .iter()
        .find(|f| decode_rpc_method(&f.payload) == Some(0x0c))
        .expect("cluster list reply")
        .payload
        .clone();
    let body = &payload[5..];
    let uuids = scan_prefixed_uuids(body);
    let cluster = "1619820a-d36f-4d8a-a716-1516b1dea077";
    assert!(uuids.iter().any(|(_, u)| u == cluster));
}

#[test]
fn manager_list_methods_match() {
    let Some(data) =
        load_frames("logs/session_1771103984_389177_127_0_0_1_37414/client_to_server.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&data);
    assert_eq!(methods, vec![0x09, 0x12]);

    let Some(s2c) =
        load_frames("logs/session_1771103984_389177_127_0_0_1_37414/server_to_client.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&s2c);
    assert_eq!(methods, vec![0x13]);
}

#[test]
fn infobase_info_methods_match() {
    let Some(data) =
        load_frames("logs/session_1771103995_390019_127_0_0_1_49436/client_to_server.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&data);
    assert_eq!(methods, vec![0x09, 0x0a, 0x30]);

    let Some(s2c) =
        load_frames("logs/session_1771103995_390019_127_0_0_1_49436/server_to_client.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&s2c);
    assert_eq!(methods, vec![0x31]);
}

#[test]
fn agent_version_methods_match() {
    let Some(data) =
        load_frames("logs/session_1771103983_389122_127_0_0_1_37406/client_to_server.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&data);
    assert_eq!(methods, vec![0x87]);

    let Some(s2c) =
        load_frames("logs/session_1771103983_389122_127_0_0_1_37406/server_to_client.stream.bin")
    else {
        return;
    };
    let methods = rpc_methods_from_stream(&s2c);
    assert_eq!(methods, vec![0x88]);
}
