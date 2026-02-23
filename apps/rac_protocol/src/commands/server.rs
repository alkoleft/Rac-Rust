use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

mod generated {
    include!("server_generated.rs");
}

pub use generated::ServerRecord;

#[derive(Debug, Serialize)]
pub struct ServerListResp {
    pub servers: Vec<ServerRecord>,
}

#[derive(Debug, Serialize)]
pub struct ServerInfoResp {
    pub server: ServerRecord,
}

pub fn server_list(client: &mut RacClient, cluster: Uuid16) -> Result<ServerListResp> {
    let reply = client.call(RacRequest::ServerList { cluster })?;
    let body = rpc_body(&reply)?;
    let servers = parse_server_list(body)?;
    Ok(ServerListResp {
        servers,
    })
}

pub fn server_info(
    client: &mut RacClient,
    cluster: Uuid16,
    server: Uuid16,
) -> Result<ServerInfoResp> {
    let reply = client.call(RacRequest::ServerInfo { cluster, server })?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    Ok(ServerInfoResp {
        server: parse_server_record(&mut cursor)?,
    })
}

fn parse_server_list(body: &[u8]) -> Result<Vec<ServerRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut servers = Vec::with_capacity(count);
    for _ in 0..count {
        servers.push(parse_server_record(&mut cursor)?);
    }
    Ok(servers)
}

fn parse_server_record(cursor: &mut RecordCursor<'_>) -> Result<ServerRecord> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("server record truncated"));
    }
    ServerRecord::decode(cursor)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    fn expected_name() -> String {
        let bytes = vec![
            0xd0, 0xa6, 0xd0, 0xb5, 0xd0, 0xbd, 0xd1, 0x82, 0xd1, 0x80, 0xd0, 0xb0,
            0xd0, 0xbb, 0xd1, 0x8c, 0xd0, 0xbd, 0xd1, 0x8b, 0xd0, 0xb9, 0x20, 0xd1,
            0x81, 0xd0, 0xb5, 0xd1, 0x80, 0xd0, 0xb2, 0xd0, 0xb5, 0xd1, 0x80,
        ];
        String::from_utf8(bytes).expect("name utf8")
    }

    #[test]
    fn parse_server_list_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/server_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let servers = parse_server_list(body).expect("parse list");

        assert_eq!(servers.len(), 1);
        let server = &servers[0];
        assert_eq!(
            server.server,
            [
                0x6a, 0xa3, 0xa8, 0x8a, 0x93, 0x46, 0x44, 0x99, 0x80, 0x34, 0xa4, 0xa7,
                0x2d, 0x7e, 0xe8, 0xe8,
            ]
        );
        assert_eq!(server.agent_host, "alko-home");
        assert_eq!(server.agent_port, 1540);
        assert_eq!(server.name, expected_name());
        assert_eq!(server.using, 1);
        assert_eq!(server.dedicate_managers, 0);
        assert_eq!(server.gap_1, 0);
        assert_eq!(server.safe_call_memory_limit, 4);
        assert_eq!(server.gap_2, 0);
        assert_eq!(server.infobases_limit, 8);
        assert_eq!(server.gap_3, 0);
        assert_eq!(server.gap_4, 16_777_216);
        assert_eq!(server.gap_4_pad, 0);
        assert_eq!(server.cluster_port, 1541);
        assert_eq!(server.connections_limit, 256);
        assert_eq!(server.port_range_end, 1591);
        assert_eq!(server.port_range_start, 1560);
        assert_eq!(server.critical_total_memory, 10_240_000_000);
        assert_eq!(server.gap_5, 0);
        assert_eq!(server.temporary_allowed_total_memory, 789_456_123);
        assert_eq!(server.gap_6, 0);
        assert_eq!(server.temporary_allowed_total_memory_time_limit, 300);
        assert_eq!(server.service_principal_name, "spn test");
        assert_eq!(server.restart_schedule, "");
        assert_eq!(server.gap_7, 0);
    }

    #[test]
    fn parse_server_info_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/server_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let mut cursor = RecordCursor::new(body, 0);
        let server = parse_server_record(&mut cursor).expect("parse info");

        assert_eq!(
            server.server,
            [
                0x6a, 0xa3, 0xa8, 0x8a, 0x93, 0x46, 0x44, 0x99, 0x80, 0x34, 0xa4, 0xa7,
                0x2d, 0x7e, 0xe8, 0xe8,
            ]
        );
        assert_eq!(server.agent_host, "alko-home");
        assert_eq!(server.agent_port, 1540);
        assert_eq!(server.name, expected_name());
        assert_eq!(server.using, 1);
        assert_eq!(server.dedicate_managers, 0);
        assert_eq!(server.gap_1, 0);
        assert_eq!(server.safe_call_memory_limit, 4);
        assert_eq!(server.gap_2, 0);
        assert_eq!(server.infobases_limit, 8);
        assert_eq!(server.gap_3, 0);
        assert_eq!(server.gap_4, 16_777_216);
        assert_eq!(server.gap_4_pad, 0);
        assert_eq!(server.cluster_port, 1541);
        assert_eq!(server.connections_limit, 256);
        assert_eq!(server.port_range_end, 1591);
        assert_eq!(server.port_range_start, 1560);
        assert_eq!(server.critical_total_memory, 10_240_000_000);
        assert_eq!(server.gap_5, 0);
        assert_eq!(server.temporary_allowed_total_memory, 789_456_123);
        assert_eq!(server.gap_6, 0);
        assert_eq!(server.temporary_allowed_total_memory_time_limit, 300);
        assert_eq!(server.service_principal_name, "spn test");
        assert_eq!(server.restart_schedule, "");
        assert_eq!(server.gap_7, 0);
    }
}
