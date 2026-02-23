use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;

#[derive(Debug, Serialize, Clone)]
pub struct ServerRecord {
    pub server: Uuid16,
    pub agent_host: String,
    pub agent_port: u16,
    pub name: String,
    pub using: u32,
    pub dedicate_managers: u32,
    pub gap_1: u32,
    pub safe_call_memory_limit: u32,
    pub gap_2: u32,
    pub infobases_limit: u32,
    pub gap_3: u32,
    pub gap_4: u32,
    pub gap_4_pad: u8,
    pub cluster_port: u16,
    pub connections_limit: u16,
    pub port_range_end: u16,
    pub port_range_start: u16,
    pub critical_total_memory: u64,
    pub gap_5: u32,
    pub temporary_allowed_total_memory: u32,
    pub gap_6: u32,
    pub temporary_allowed_total_memory_time_limit: u32,
    pub service_principal_name: String,
    pub restart_schedule: String,
    pub gap_7: u8,
}

impl ServerRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let server = cursor.take_uuid()?;
        let agent_host = cursor.take_str8()?;
        let agent_port = cursor.take_u16_be()?;
        let name = cursor.take_str8()?;
        let using = cursor.take_u32_le()?;
        let dedicate_managers = cursor.take_u32_le()?;
        let gap_1 = cursor.take_u32_le()?;
        let safe_call_memory_limit = cursor.take_u32_be()?;
        let gap_2 = cursor.take_u32_le()?;
        let infobases_limit = cursor.take_u32_le()?;
        let gap_3 = cursor.take_u32_le()?;
        let gap_4 = cursor.take_u32_le()?;
        let gap_4_pad = cursor.take_u8()?;
        let cluster_port = cursor.take_u16_be()?;
        let connections_limit = cursor.take_u16_le()?;
        let port_range_end = cursor.take_u16_be()?;
        let port_range_start = cursor.take_u16_be()?;
        let critical_total_memory = cursor.take_u64_be()?;
        let gap_5 = cursor.take_u32_be()?;
        let temporary_allowed_total_memory = cursor.take_u32_be()?;
        let gap_6 = cursor.take_u32_be()?;
        let temporary_allowed_total_memory_time_limit = cursor.take_u32_be()?;
        let service_principal_name = cursor.take_str8()?;
        let restart_schedule = cursor.take_str8()?;
        let gap_7 = cursor.take_u8()?;
        Ok(Self {
            server,
            agent_host,
            agent_port,
            name,
            using,
            dedicate_managers,
            gap_1,
            safe_call_memory_limit,
            gap_2,
            infobases_limit,
            gap_3,
            gap_4,
            gap_4_pad,
            cluster_port,
            connections_limit,
            port_range_end,
            port_range_start,
            critical_total_memory,
            gap_5,
            temporary_allowed_total_memory,
            gap_6,
            temporary_allowed_total_memory_time_limit,
            service_principal_name,
            restart_schedule,
            gap_7,
        })
    }
}

pub const RPC_SERVER_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVER_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SERVER_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVER_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVER_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SERVER_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_server_info_body(body: &[u8]) -> Result<ServerRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("server info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    ServerRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct ServerListRequest {
    pub cluster: Uuid16,
}

impl ServerListRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServerInfoRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
}

impl ServerInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        Ok(())
    }
}




