use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::error::{RacError, Result};
use crate::rac_wire::scan_len_prefixed_strings;
use crate::rac_wire::uuid_from_slice;
use crate::Uuid16;

use super::{first_uuid, rpc_body};

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
    Ok(InfobaseSummaryInfoResp {
        infobase: first_uuid(body)?,
        fields: scan_len_prefixed_strings(body)
            .into_iter()
            .map(|(_, s)| s)
            .collect(),
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
    Ok(InfobaseInfoResp {
        infobase: first_uuid(body)?,
        fields: scan_len_prefixed_strings(body)
            .into_iter()
            .map(|(_, s)| s)
            .collect(),
        raw_payload: Some(reply),
    })
}

fn parse_infobase_summary_list_body(body: &[u8]) -> Result<Vec<InfobaseSummary>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let count = body[0] as usize;
    let mut off = 1;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        if off + 16 > body.len() {
            return Err(RacError::Decode("infobase summary list: truncated uuid"));
        }
        let uuid = uuid_from_slice(&body[off..off + 16])?;
        off += 16;

        if off >= body.len() {
            return Err(RacError::Decode("infobase summary list: missing tag"));
        }
        // Some servers include a 0x2c tag byte, others go straight to descr length.
        let mut has_tag = body[off] == 0x2c;
        if !has_tag && off + 1 < body.len() {
            let len_no_tag = body[off] as usize;
            if off + 1 + len_no_tag > body.len() {
                let len_with_tag = body[off + 1] as usize;
                if off + 2 + len_with_tag <= body.len() {
                    has_tag = true;
                }
            }
        }
        if has_tag {
            off += 1;
            if off >= body.len() {
                return Err(RacError::Decode(
                    "infobase summary list: missing descr length",
                ));
            }
        }

        let descr_len = body[off] as usize;
        off += 1;
        if off + descr_len > body.len() {
            return Err(RacError::Decode("infobase summary list: truncated descr"));
        }
        let descr = String::from_utf8_lossy(&body[off..off + descr_len]).to_string();
        off += descr_len;

        if off >= body.len() {
            return Err(RacError::Decode(
                "infobase summary list: missing name length",
            ));
        }
        let name_len = body[off] as usize;
        off += 1;
        if off + name_len > body.len() {
            return Err(RacError::Decode("infobase summary list: truncated name"));
        }
        let name = String::from_utf8_lossy(&body[off..off + name_len]).to_string();
        off += name_len;

        out.push(InfobaseSummary {
            infobase: uuid,
            name,
            descr,
        });
    }
    Ok(out)
}
