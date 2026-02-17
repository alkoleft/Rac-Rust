use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct ServerListResp {
    pub servers: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ServerInfoResp {
    pub server: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn server_list(client: &mut RacClient, cluster: Uuid16) -> Result<ServerListResp> {
    let reply = client.call(RacRequest::ServerList { cluster })?;
    let body = rpc_body(&reply)?;
    let servers = parse_server_list(body)?;
    Ok(ServerListResp {
        servers,
        raw_payload: Some(reply),
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
    let parsed = parse_server_record(&mut cursor)?;
    Ok(ServerInfoResp {
        server: parsed.uuid,
        fields: parsed.strings,
        raw_payload: Some(reply),
    })
}

struct ServerRecordParts {
    uuid: Uuid16,
    strings: Vec<String>,
}

fn parse_server_list(body: &[u8]) -> Result<Vec<Uuid16>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut servers = Vec::with_capacity(count);
    for _ in 0..count {
        let record = parse_server_record(&mut cursor)?;
        servers.push(record.uuid);
    }
    Ok(servers)
}

fn parse_server_record(cursor: &mut RecordCursor<'_>) -> Result<ServerRecordParts> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("server record truncated"));
    }
    let uuid = cursor.take_uuid()?;
    let mut strings = Vec::new();
    strings.push(cursor.take_str8()?);
    let _agent_port = cursor.take_u16_be()?;
    strings.push(cursor.take_str8()?);

    let _using = cursor.take_u32_le()?;
    let _dedicate_managers = cursor.take_u32_le()?;
    let _gap_1 = cursor.take_u32_le()?;
    let _safe_call_memory = cursor.take_u32_be()?;
    let _gap_2 = cursor.take_u32_le()?;
    let _infobases_limit = cursor.take_u32_le()?;
    let _gap_3 = cursor.take_u32_le()?;
    let _gap_4 = cursor.take_u32_le()?;
    let _cluster_port = cursor.take_u16_be()?;
    let _port_range_end = cursor.take_u16_be()?;
    let _port_range_start = cursor.take_u16_be()?;
    let _critical_total_memory = cursor.take_u64_be()?;
    let _gap_5 = cursor.take_u32_be()?;
    let _temporary_allowed_total_memory = cursor.take_u32_be()?;
    let _gap_6 = cursor.take_u32_be()?;
    let _temporary_allowed_total_memory_time = cursor.take_u32_be()?;

    strings.push(cursor.take_str8()?);
    strings.push(cursor.take_str8()?);
    let _gap_7 = cursor.take_u8()?;

    Ok(ServerRecordParts { uuid, strings })
}
