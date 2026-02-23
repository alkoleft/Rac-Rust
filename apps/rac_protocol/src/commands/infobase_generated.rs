use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct InfobaseSummary {
    pub infobase: Uuid16,
    pub descr: String,
    pub name: String,
}

impl InfobaseSummary {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let infobase = cursor.take_uuid()?;
        let descr = {
            let first = cursor.take_u8()? as usize;
            let len = if first == 0x2c { cursor.take_u8()? as usize } else { first };
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        let name = {
            let len = cursor.take_u8()? as usize;
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        Ok(Self {
            infobase,
            descr,
            name,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct InfobaseFieldsRecord {
    pub infobase: Uuid16,
    pub fields: Vec<String>,
}

impl InfobaseFieldsRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let infobase = cursor.take_uuid()?;
        let fields = {
            let mut out = Vec::new();
            while cursor.remaining_len() > 0 {
                out.push(cursor.take_str8()?);
            }
            out
        };
        Ok(Self {
            infobase,
            fields,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InfobaseSummaryListRequest {
    pub cluster: Uuid16,
}

impl InfobaseSummaryListRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct InfobaseSummaryInfoRequest {
    pub cluster: Uuid16,
    pub infobase: Uuid16,
}

impl InfobaseSummaryInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.infobase);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct InfobaseInfoRequest {
    pub cluster: Uuid16,
    pub infobase: Uuid16,
}

impl InfobaseInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.infobase);
        Ok(())
    }
}



pub fn parse_infobase_summary_info_body(body: &[u8]) -> Result<InfobaseFieldsRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("infobase summary info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    InfobaseFieldsRecord::decode(&mut cursor)
}

pub fn parse_infobase_info_body(body: &[u8]) -> Result<InfobaseFieldsRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("infobase info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    InfobaseFieldsRecord::decode(&mut cursor)
}


pub const RPC_INFOBASE_SUMMARY_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: crate::rac_wire::METHOD_INFOBASE_SUMMARY_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_INFOBASE_SUMMARY_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_INFOBASE_SUMMARY_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: crate::rac_wire::METHOD_INFOBASE_SUMMARY_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_INFOBASE_SUMMARY_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: true,
};

pub const RPC_INFOBASE_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: crate::rac_wire::METHOD_INFOBASE_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_INFOBASE_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: true,
};


