use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

#[derive(Debug, Serialize, Clone)]
pub struct ClusterAdminRecord {
    pub name: String,
    pub unknown_tag: u8,
    pub unknown_flags: u32,
    pub unknown_tail: [u8; 3],
}

impl ClusterAdminRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let name = cursor.take_str8()?;
        let unknown_tag = cursor.take_u8()?;
        let unknown_flags = cursor.take_u32_be()?;
        let unknown_tail = {
            let bytes = cursor.take_bytes(3)?;
            let value: [u8; 3] = bytes.as_slice().try_into().map_err(|_| RacError::Decode("bytes_fixed"))?;
            value
        };
        Ok(Self {
            name,
            unknown_tag,
            unknown_flags,
            unknown_tail,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusterRecord {
    pub uuid: Uuid16,
    pub expiration_timeout: u32,
    pub host: String,
    pub lifetime_limit: u32,
    pub port: u16,
    pub max_memory_size: u32,
    pub max_memory_time_limit: u32,
    pub display_name: String,
    pub security_level: u32,
    pub session_fault_tolerance_level: u32,
    pub load_balancing_mode: u32,
    pub errors_count_threshold: u32,
    pub kill_problem_processes: u8,
    pub kill_by_memory_with_dump: u8,
}

impl ClusterRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let uuid = cursor.take_uuid()?;
        let expiration_timeout = cursor.take_u32_be()?;
        let host = cursor.take_str8()?;
        let lifetime_limit = cursor.take_u32_be()?;
        let port = cursor.take_u16_be()?;
        let max_memory_size = cursor.take_u32_be()?;
        let max_memory_time_limit = cursor.take_u32_be()?;
        let display_name = cursor.take_str8()?;
        let security_level = cursor.take_u32_be()?;
        let session_fault_tolerance_level = cursor.take_u32_be()?;
        let load_balancing_mode = cursor.take_u32_be()?;
        let errors_count_threshold = cursor.take_u32_be()?;
        let kill_problem_processes = cursor.take_u8()?;
        let kill_by_memory_with_dump = cursor.take_u8()?;
        Ok(Self {
            uuid,
            expiration_timeout,
            host,
            lifetime_limit,
            port,
            max_memory_size,
            max_memory_time_limit,
            display_name,
            security_level,
            session_fault_tolerance_level,
            load_balancing_mode,
            errors_count_threshold,
            kill_problem_processes,
            kill_by_memory_with_dump,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClusterIdRequest {
    pub cluster: Uuid16,
}

impl ClusterIdRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClusterAuthRequest {
    pub cluster: Uuid16,
    pub user: String,
    pub pwd: String,
}

impl ClusterAuthRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.user.len() + 1 + self.pwd.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.user.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClusterAdminRegisterRequest {
    pub cluster: Uuid16,
    pub name: String,
    pub descr: String,
    pub pwd: String,
    pub auth_flags: u8,
}

impl ClusterAdminRegisterRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.name.len() + 1 + self.descr.len() + 1 + self.pwd.len() + 1 + 2
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        out.push(self.auth_flags);
        out.extend_from_slice(&[0, 0]);
        Ok(())
    }
}
