use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct ConnectionListResp {
    pub connections: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionInfoResp {
    pub connection: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn connection_list(client: &mut RacClient, cluster: Uuid16) -> Result<ConnectionListResp> {
    let reply = client.call(RacRequest::ConnectionList { cluster })?;
    let body = rpc_body(&reply)?;
    let connections = parse_connection_list(body)?;
    Ok(ConnectionListResp {
        connections,
        raw_payload: Some(reply),
    })
}

pub fn connection_info(
    client: &mut RacClient,
    cluster: Uuid16,
    connection: Uuid16,
) -> Result<ConnectionInfoResp> {
    let reply = client.call(RacRequest::ConnectionInfo {
        cluster,
        connection,
    })?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let record = parse_connection_record(&mut cursor)?;
    Ok(ConnectionInfoResp {
        connection: record.uuid,
        fields: record.strings,
        raw_payload: Some(reply),
    })
}

struct ConnectionRecordParts {
    uuid: Uuid16,
    strings: Vec<String>,
}

fn parse_connection_list(body: &[u8]) -> Result<Vec<Uuid16>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut connections = Vec::with_capacity(count);
    for _ in 0..count {
        let record = parse_connection_record(&mut cursor)?;
        connections.push(record.uuid);
    }
    Ok(connections)
}

fn parse_connection_record(cursor: &mut RecordCursor<'_>) -> Result<ConnectionRecordParts> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("connection record truncated"));
    }
    let uuid = cursor.take_uuid()?;
    let mut strings = Vec::new();
    strings.push(cursor.take_str8()?);
    let _blocked_by_ls = cursor.take_u32_be()?;
    let _connected_at = cursor.take_u64_be()?;
    let _conn_id = cursor.take_u32_be()?;
    strings.push(cursor.take_str8()?);
    let _infobase = cursor.take_uuid()?;
    let _process = cursor.take_uuid()?;
    let _session_number = cursor.take_u32_be()?;
    Ok(ConnectionRecordParts { uuid, strings })
}
