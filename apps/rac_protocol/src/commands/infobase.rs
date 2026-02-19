use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;

use super::rpc_body;

mod generated {
    include!("infobase_generated.rs");
}

pub use generated::InfobaseSummary;
use generated::InfobaseFieldsRecord;

#[derive(Debug, Serialize)]
pub struct InfobaseSummaryListResp {
    pub infobases: Vec<Uuid16>,
    pub summaries: Vec<InfobaseSummary>,
    pub raw_payload: Option<Vec<u8>>,
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
    let record = InfobaseFieldsRecord::decode(&mut cursor)?;
    Ok(InfobaseSummaryInfoResp {
        infobase: record.infobase,
        fields: record.fields,
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
    let record = InfobaseFieldsRecord::decode(&mut cursor)?;
    Ok(InfobaseInfoResp {
        infobase: record.infobase,
        fields: record.fields,
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
        out.push(InfobaseSummary::decode(&mut cursor)?);
    }
    Ok(out)
}
