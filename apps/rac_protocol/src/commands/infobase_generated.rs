use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_INFOBASE_SUMMARY_LIST_REQ: u8 = 0x2a;
pub const METHOD_INFOBASE_SUMMARY_LIST_RESP: u8 = 0x2b;
pub const METHOD_INFOBASE_SUMMARY_INFO_REQ: u8 = 0x2e;
pub const METHOD_INFOBASE_SUMMARY_INFO_RESP: u8 = 0x2f;
pub const METHOD_INFOBASE_INFO_REQ: u8 = 0x30;
pub const METHOD_INFOBASE_INFO_RESP: u8 = 0x31;
pub const METHOD_INFOBASE_SUMMARY_UPDATE_REQ: u8 = 0x27;

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
pub struct InfobaseInfoRecord {
    pub infobase: Uuid16,
    pub tag: u8,
    pub unknown_u32_0: u32,
    pub dbms: String,
    pub name: String,
    pub unknown_str_0: String,
    pub db_server: String,
    pub db_user: String,
    pub unknown_str_1: String,
    pub unknown_str_2: String,
    pub unknown_bytes_0: [u8; 4],
    pub denied_message: String,
    pub denied_parameter: String,
    pub unknown_str_3: String,
    pub unknown_str_4: String,
    pub unknown_u32_1: u32,
    pub descr: String,
    pub locale: String,
    pub db_name: String,
    pub permission_code: String,
    pub tail: [u8; 28],
}

impl InfobaseInfoRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let infobase = cursor.take_uuid()?;
        let tag = cursor.take_u8()?;
        let unknown_u32_0 = cursor.take_u32_be()?;
        let dbms = cursor.take_str8()?;
        let name = cursor.take_str8()?;
        let unknown_str_0 = cursor.take_str8()?;
        let db_server = cursor.take_str8()?;
        let db_user = cursor.take_str8()?;
        let unknown_str_1 = cursor.take_str8()?;
        let unknown_str_2 = cursor.take_str8()?;
        let unknown_bytes_0 = {
            let bytes = cursor.take_bytes(4)?;
            let value: [u8; 4] = bytes.as_slice().try_into().map_err(|_| RacError::Decode("bytes_fixed"))?;
            value
        };
        let denied_message = cursor.take_str8()?;
        let denied_parameter = cursor.take_str8()?;
        let unknown_str_3 = cursor.take_str8()?;
        let unknown_str_4 = cursor.take_str8()?;
        let unknown_u32_1 = cursor.take_u32_be()?;
        let descr = cursor.take_str8()?;
        let locale = cursor.take_str8()?;
        let db_name = cursor.take_str8()?;
        let permission_code = cursor.take_str8()?;
        let tail = {
            let bytes = cursor.take_bytes(28)?;
            let value: [u8; 28] = bytes.as_slice().try_into().map_err(|_| RacError::Decode("bytes_fixed"))?;
            value
        };
        Ok(Self {
            infobase,
            tag,
            unknown_u32_0,
            dbms,
            name,
            unknown_str_0,
            db_server,
            db_user,
            unknown_str_1,
            unknown_str_2,
            unknown_bytes_0,
            denied_message,
            denied_parameter,
            unknown_str_3,
            unknown_str_4,
            unknown_u32_1,
            descr,
            locale,
            db_name,
            permission_code,
            tail,
        })
    }
}

pub struct InfobaseSummaryListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for InfobaseSummaryListRpc {
    type Response = InfobaseSummaryListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_INFOBASE_SUMMARY_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16);
        out.extend_from_slice(&self.cluster);
        Ok(out)
    }
}

pub struct InfobaseSummaryInfoRpc {
    pub cluster: Uuid16,
    pub infobase: Uuid16,
}

impl crate::rpc::Request for InfobaseSummaryInfoRpc {
    type Response = InfobaseSummaryInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_INFOBASE_SUMMARY_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 16);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.infobase);
        Ok(out)
    }
}

pub struct InfobaseInfoRpc {
    pub cluster: Uuid16,
    pub infobase: Uuid16,
}

impl crate::rpc::Request for InfobaseInfoRpc {
    type Response = InfobaseInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_INFOBASE_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 16);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.infobase);
        Ok(out)
    }
}

pub struct InfobaseSummaryUpdateRpc {
    pub cluster: Uuid16,
    pub infobase: Uuid16,
    pub descr: String,
}

impl crate::rpc::Request for InfobaseSummaryUpdateRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_INFOBASE_SUMMARY_UPDATE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 16 + 1 + self.descr.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.infobase);
        out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct InfobaseSummaryListResp {
    pub summaries: Vec<InfobaseSummary>,
}

impl crate::rpc::Response for InfobaseSummaryListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        Ok(Self {
            summaries: crate::commands::parse_list_u8(body, InfobaseSummary::decode)?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct InfobaseSummaryInfoResp {
    pub summary: InfobaseSummary,
}

impl crate::rpc::Response for InfobaseSummaryInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let record = parse_infobase_summary_info_body(body)?;
        Ok(Self {
            summary: record,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct InfobaseInfoResp {
    pub info: InfobaseInfoRecord,
}

impl crate::rpc::Response for InfobaseInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let record = parse_infobase_info_body(body)?;
        Ok(Self {
            info: record,
        })
    }
}


pub fn parse_infobase_summary_info_body(body: &[u8]) -> Result<InfobaseSummary> {
    if body.is_empty() {
        return Err(RacError::Decode("infobase summary info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    InfobaseSummary::decode(&mut cursor)
}

pub fn parse_infobase_info_body(body: &[u8]) -> Result<InfobaseInfoRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("infobase info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    InfobaseInfoRecord::decode(&mut cursor)
}


pub const RPC_INFOBASE_SUMMARY_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_INFOBASE_SUMMARY_LIST_REQ,
    method_resp: Some(METHOD_INFOBASE_SUMMARY_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_INFOBASE_SUMMARY_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_INFOBASE_SUMMARY_INFO_REQ,
    method_resp: Some(METHOD_INFOBASE_SUMMARY_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: true,
};

pub const RPC_INFOBASE_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_INFOBASE_INFO_REQ,
    method_resp: Some(METHOD_INFOBASE_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: true,
};

pub const RPC_INFOBASE_SUMMARY_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_INFOBASE_SUMMARY_UPDATE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};


