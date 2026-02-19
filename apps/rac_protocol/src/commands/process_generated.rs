use crate::Uuid16;
use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Default, Clone)]
pub struct ProcessLicense {
    pub file_name: String,
    pub full_presentation: String,
    pub issued_by_server: bool,
    pub license_type: u32,
    pub max_users_all: u32,
    pub max_users_current: u32,
    pub network_key: bool,
    pub server_address: String,
    pub process_id: String,
    pub server_port: u32,
    pub key_series: String,
    pub brief_presentation: String,
}

impl ProcessLicense {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let __gap_license_0 = cursor.take_u8()?;
        let file_name = cursor.take_str8()?;
        let full_presentation = {
            let b0 = cursor.take_u8()? as usize;
            let b1 = cursor.take_u8()? as usize;
            let len = (b0 & 0x3f) | (b1 << 6);
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        let issued_by_server = cursor.take_u8()? != 0;
        let license_type = cursor.take_u32_be()?;
        let max_users_all = cursor.take_u32_be()?;
        let max_users_current = cursor.take_u32_be()?;
        let network_key = cursor.take_u8()? != 0;
        let server_address = cursor.take_str8()?;
        let process_id = cursor.take_str8()?;
        let server_port = cursor.take_u32_be()?;
        let key_series = cursor.take_str8()?;
        let brief_presentation = cursor.take_str8()?;
        Ok(Self {
            file_name,
            full_presentation,
            issued_by_server,
            license_type,
            max_users_all,
            max_users_current,
            network_key,
            server_address,
            process_id,
            server_port,
            key_series,
            brief_presentation,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ProcessRecord {
    pub process: Uuid16,
    pub avg_call_time: f64,
    pub avg_db_call_time: f64,
    pub avg_lock_call_time: f64,
    pub avg_server_call_time: f64,
    pub avg_threads: f64,
    pub capacity: u32,
    pub connections: u32,
    pub host: String,
    pub licenses: Vec<ProcessLicense>,
    pub port: u16,
    pub memory_excess_time: u32,
    pub memory_size: u32,
    pub pid: String,
    pub use_status: u32,
    pub selection_size: u32,
    pub started_at: String,
    pub running: bool,
    pub available_performance: u32,
    pub reserve: bool,
    pub turned_on: bool,
}

impl ProcessRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let process = cursor.take_uuid()?;
        let __gap_0 = cursor.take_bytes(8)?;
        let avg_call_time = cursor.take_f64_be()?;
        let avg_db_call_time = cursor.take_f64_be()?;
        let avg_lock_call_time = cursor.take_f64_be()?;
        let avg_server_call_time = cursor.take_f64_be()?;
        let avg_threads = cursor.take_f64_be()?;
        let capacity = cursor.take_u32_be()?;
        let connections = cursor.take_u32_be()?;
        let host = cursor.take_str8()?;
        let licenses = {
            let count = cursor.take_u8()? as usize;
            let mut out = Vec::with_capacity(count);
            for _ in 0..count {
                out.push(ProcessLicense::decode(cursor)?);
            }
            out
        };
        let port = cursor.take_u16_be()?;
        let memory_excess_time = cursor.take_u32_be()?;
        let memory_size = cursor.take_u32_be()?;
        let pid = cursor.take_str8()?;
        let use_status = cursor.take_u32_be()?;
        let selection_size = cursor.take_u32_be()?;
        let started_at = v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();
        let running = cursor.take_u32_be()? != 0;
        let available_performance = cursor.take_u32_be()?;
        let reserve = cursor.take_u8()? != 0;

        let turned_on = use_status != 0;
        Ok(Self {
            process,
            avg_call_time,
            avg_db_call_time,
            avg_lock_call_time,
            avg_server_call_time,
            avg_threads,
            capacity,
            connections,
            host,
            licenses,
            port,
            memory_excess_time,
            memory_size,
            pid,
            use_status,
            selection_size,
            started_at,
            running,
            available_performance,
            reserve,
            turned_on,
        })
    }
}
