use crate::rac_wire::format::encode_rpc;

pub fn decode_rpc_method(payload: &[u8]) -> Option<u8> {
    if payload.len() >= 5 && payload[0..4] == [0x01, 0x00, 0x00, 0x01] {
        Some(payload[4])
    } else {
        None
    }
}

pub fn init_packet() -> &'static [u8] {
    &[
        0x1c, 0x53, 0x57, 0x50, 0x01, 0x00, 0x01, 0x00, 0x01, 0x16, 0x01, 0x0f, 0x63, 0x6f,
        0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x04,
        0x00, 0x00, 0x07, 0xd0,
    ]
}

pub fn encode_service_negotiation() -> &'static [u8] {
    &[
        0x18, 0x76, 0x38, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x41, 0x64,
        0x6d, 0x69, 0x6e, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x04, 0x31, 0x36,
        0x2e, 0x30, 0x80,
    ]
}

pub fn encode_close() -> &'static [u8] {
    &[0x01]
}

pub fn encode_agent_version() -> Vec<u8> {
    encode_rpc(0x87, &[])
}

pub fn encode_cluster_context(cluster_uuid: [u8; 16]) -> Vec<u8> {
    let mut body = Vec::with_capacity(16 + 2);
    body.extend_from_slice(&cluster_uuid);
    body.extend_from_slice(&[0x00, 0x00]);
    encode_rpc(0x09, &body)
}

pub fn encode_infobase_context(cluster_uuid: [u8; 16]) -> Vec<u8> {
    let mut body = Vec::with_capacity(16 + 2);
    body.extend_from_slice(&cluster_uuid);
    body.extend_from_slice(&[0x00, 0x00]);
    encode_rpc(0x0a, &body)
}

pub fn encode_cluster_scoped(method_id: u8, cluster_uuid: [u8; 16]) -> Vec<u8> {
    let mut body = Vec::with_capacity(16);
    body.extend_from_slice(&cluster_uuid);
    encode_rpc(method_id, &body)
}

pub fn encode_cluster_scoped_object(method_id: u8, cluster_uuid: [u8; 16], object_uuid: [u8; 16]) -> Vec<u8> {
    let mut body = Vec::with_capacity(16 + 16);
    body.extend_from_slice(&cluster_uuid);
    body.extend_from_slice(&object_uuid);
    encode_rpc(method_id, &body)
}
