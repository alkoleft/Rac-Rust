use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct InfobaseSummaryListResp {
    pub infobases: Vec<Uuid16>,
    pub summaries: Vec<InfobaseSummary>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct InfobaseSummary {
    pub infobase: Uuid16,
    pub name: String,
    pub descr: String,
}

#[derive(Debug, Serialize)]
pub struct InfobaseSummaryInfoResp {
    pub infobase: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct InfobaseInfoResp {
    pub infobase: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn infobase_summary_list(
    client: &mut RacClient,
    cluster: Uuid16,
) -> Result<InfobaseSummaryListResp> {
    let reply = client.call(RacRequest::InfobaseSummaryList { cluster })?;
    let body = rpc_body(&reply)?;
    let summaries = parse_infobase_summary_list_body(body)?;
    Ok(InfobaseSummaryListResp {
        infobases: summaries.iter().map(|s| s.infobase).collect(),
        summaries,
        raw_payload: Some(reply),
    })
}

pub fn infobase_summary_info(
    client: &mut RacClient,
    cluster: Uuid16,
    infobase: Uuid16,
) -> Result<InfobaseSummaryInfoResp> {
    let reply = client.call(RacRequest::InfobaseSummaryInfo { cluster, infobase })?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let infobase = cursor.take_uuid()?;
    let fields = read_str8_fields(&mut cursor)?;
    Ok(InfobaseSummaryInfoResp {
        infobase,
        fields,
        raw_payload: Some(reply),
    })
}

pub fn infobase_info(
    client: &mut RacClient,
    cluster: Uuid16,
    infobase: Uuid16,
) -> Result<InfobaseInfoResp> {
    let reply = client.call(RacRequest::InfobaseInfo { cluster, infobase })?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let infobase = cursor.take_uuid()?;
    let fields = read_str8_fields(&mut cursor)?;
    Ok(InfobaseInfoResp {
        infobase,
        fields,
        raw_payload: Some(reply),
    })
}

fn parse_infobase_summary_list_body(body: &[u8]) -> Result<Vec<InfobaseSummary>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let uuid = cursor.take_uuid()?;
        let first = cursor.take_u8()?;
        let descr_len = if first == 0x2c {
            cursor.take_u8()? as usize
        } else {
            first as usize
        };
        let descr = if descr_len == 0 {
            String::new()
        } else {
            let bytes = cursor.take_bytes(descr_len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        let name_len = cursor.take_u8()? as usize;
        let name = if name_len == 0 {
            String::new()
        } else {
            let bytes = cursor.take_bytes(name_len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        out.push(InfobaseSummary {
            infobase: uuid,
            name,
            descr,
        });
    }
    Ok(out)
}

fn read_str8_fields(cursor: &mut RecordCursor<'_>) -> Result<Vec<String>> {
    let mut out = Vec::new();
    while cursor.remaining_len() > 0 {
        out.push(cursor.take_str8()?);
    }
    Ok(out)
}
