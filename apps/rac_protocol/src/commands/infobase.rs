use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request, Response};
use crate::rpc::decode_utils::rpc_body;
use crate::rac_wire::{
    METHOD_INFOBASE_INFO_REQ, METHOD_INFOBASE_INFO_RESP, METHOD_INFOBASE_SUMMARY_INFO_REQ,
    METHOD_INFOBASE_SUMMARY_INFO_RESP, METHOD_INFOBASE_SUMMARY_LIST_REQ,
    METHOD_INFOBASE_SUMMARY_LIST_RESP,
};
use crate::Uuid16;


mod generated {
    include!("infobase_generated.rs");
}

pub use generated::InfobaseSummary;
use generated::InfobaseFieldsRecord;

#[derive(Debug, Serialize)]
pub struct InfobaseSummaryListResp {
    pub infobases: Vec<Uuid16>,
    pub summaries: Vec<InfobaseSummary>,
}

#[derive(Debug, Serialize)]
pub struct InfobaseSummaryInfoResp {
    pub infobase: Uuid16,
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct InfobaseInfoResp {
    pub infobase: Uuid16,
    pub fields: Vec<String>,
}

struct InfobaseSummaryListRpc {
    cluster: Uuid16,
}

impl Request for InfobaseSummaryListRpc {
    type Response = InfobaseSummaryListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_INFOBASE_SUMMARY_LIST_REQ,
            method_resp: Some(METHOD_INFOBASE_SUMMARY_LIST_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(self.cluster.to_vec())
    }
}

impl Response for InfobaseSummaryListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let summaries = parse_infobase_summary_list_body(body)?;
        Ok(Self {
            infobases: summaries.iter().map(|s| s.infobase).collect(),
            summaries,
        })
    }
}

pub fn infobase_summary_list(
    client: &mut RacClient,
    cluster: Uuid16,
) -> Result<InfobaseSummaryListResp> {
    client.call_typed(InfobaseSummaryListRpc { cluster })
}

struct InfobaseSummaryInfoRpc {
    cluster: Uuid16,
    infobase: Uuid16,
}

impl Request for InfobaseSummaryInfoRpc {
    type Response = InfobaseSummaryInfoResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_INFOBASE_SUMMARY_INFO_REQ,
            method_resp: Some(METHOD_INFOBASE_SUMMARY_INFO_RESP),
            requires_cluster_context: true,
            requires_infobase_context: true,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(32);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.infobase);
        Ok(out)
    }
}

impl Response for InfobaseSummaryInfoResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let mut cursor = RecordCursor::new(body, 0);
        let record = InfobaseFieldsRecord::decode(&mut cursor)?;
        Ok(Self {
            infobase: record.infobase,
            fields: record.fields,
        })
    }
}

pub fn infobase_summary_info(
    client: &mut RacClient,
    cluster: Uuid16,
    infobase: Uuid16,
) -> Result<InfobaseSummaryInfoResp> {
    client.call_typed(InfobaseSummaryInfoRpc { cluster, infobase })
}

struct InfobaseInfoRpc {
    cluster: Uuid16,
    infobase: Uuid16,
}

impl Request for InfobaseInfoRpc {
    type Response = InfobaseInfoResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_INFOBASE_INFO_REQ,
            method_resp: Some(METHOD_INFOBASE_INFO_RESP),
            requires_cluster_context: true,
            requires_infobase_context: true,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(32);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.infobase);
        Ok(out)
    }
}

impl Response for InfobaseInfoResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let mut cursor = RecordCursor::new(body, 0);
        let record = InfobaseFieldsRecord::decode(&mut cursor)?;
        Ok(Self {
            infobase: record.infobase,
            fields: record.fields,
        })
    }
}

pub fn infobase_info(
    client: &mut RacClient,
    cluster: Uuid16,
    infobase: Uuid16,
) -> Result<InfobaseInfoResp> {
    client.call_typed(InfobaseInfoRpc { cluster, infobase })
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
