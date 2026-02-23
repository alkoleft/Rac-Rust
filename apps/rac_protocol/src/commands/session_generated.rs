use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;

#[derive(Debug, Serialize, Default, Clone)]
pub struct SessionLicenseRecord {
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

impl SessionLicenseRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let file_name = cursor.take_str8_opt()?.unwrap_or_default();
        let full_presentation = cursor.take_str8_opt()?.unwrap_or_default();
        let issued_by_server = cursor.take_bool_opt()?.unwrap_or_default();
        let license_type = cursor.take_u32_be_opt()?.unwrap_or_default();
        let max_users_all = cursor.take_u32_be_opt()?.unwrap_or_default();
        let max_users_current = cursor.take_u32_be_opt()?.unwrap_or_default();
        let network_key = cursor.take_u8()? != 0;
        let server_address = cursor.take_str8_opt()?.unwrap_or_default();
        let process_id = cursor.take_str8_opt()?.unwrap_or_default();
        let server_port = cursor.take_u32_be_opt()?.unwrap_or_default();
        let key_series = cursor.take_str8_opt()?.unwrap_or_default();
        let brief_presentation = cursor.take_str8_opt()?.unwrap_or_default();
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
pub struct SessionRecordRaw {
    pub session: Uuid16,
    pub app_id: String,
    pub blocked_by_dbms: u32,
    pub blocked_by_ls: u32,
    pub bytes_all: u64,
    pub bytes_last_5min: u64,
    pub calls_all: u32,
    pub calls_last_5min: u64,
    pub connection: Uuid16,
    pub dbms_bytes_all: u64,
    pub dbms_bytes_last_5min: u64,
    pub db_proc_info: String,
    pub db_proc_took: u32,
    pub db_proc_took_at: String,
    pub duration_all: u32,
    pub duration_all_dbms: u32,
    pub duration_current: u32,
    pub duration_current_dbms: u32,
    pub duration_last_5min: u64,
    pub duration_last_5min_dbms: u64,
    pub host: String,
    pub infobase: Uuid16,
    pub last_active_at: String,
    pub hibernate: bool,
    pub passive_session_hibernate_time: u32,
    pub hibernate_session_terminate_time: u32,
    pub license: SessionLicenseRecord,
    pub locale: String,
    pub process: Uuid16,
    pub session_id: u32,
    pub started_at: String,
    pub user_name: String,
    pub memory_current: u64,
    pub memory_last_5min: u64,
    pub memory_total: u64,
    pub read_current: u64,
    pub read_last_5min: u64,
    pub read_total: u64,
    pub write_current: u64,
    pub write_last_5min: u64,
    pub write_total: u64,
    pub duration_current_service: u32,
    pub duration_last_5min_service: u64,
    pub duration_all_service: u32,
    pub current_service_name: String,
    pub cpu_time_current: u64,
    pub cpu_time_last_5min: u64,
    pub cpu_time_total: u64,
    pub data_separation: String,
    pub client_ip: String,
}

impl SessionRecordRaw {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let session = cursor.take_uuid()?;
        let app_id = cursor.take_str8()?;
        let blocked_by_dbms = cursor.take_u32_be_opt()?.unwrap_or_default();
        let blocked_by_ls = cursor.take_u32_be_opt()?.unwrap_or_default();
        let bytes_all = cursor.take_u64_be_opt()?.unwrap_or_default();
        let bytes_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let calls_all = cursor.take_u32_be_opt()?.unwrap_or_default();
        let calls_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let connection = cursor.take_uuid_opt()?.unwrap_or_default();
        let dbms_bytes_all = cursor.take_u64_be_opt()?.unwrap_or_default();
        let dbms_bytes_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let db_proc_info = cursor.take_str8_opt()?.unwrap_or_default();
        let db_proc_took = cursor.take_u32_be_opt()?.unwrap_or_default();
        let db_proc_took_at = cursor.take_datetime_opt()?.unwrap_or_default();
        let duration_all = cursor.take_u32_be_opt()?.unwrap_or_default();
        let duration_all_dbms = cursor.take_u32_be_opt()?.unwrap_or_default();
        let duration_current = cursor.take_u32_be_opt()?.unwrap_or_default();
        let duration_current_dbms = cursor.take_u32_be_opt()?.unwrap_or_default();
        let duration_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let duration_last_5min_dbms = cursor.take_u64_be_opt()?.unwrap_or_default();
        let host = cursor.take_str8_opt()?.unwrap_or_default();
        let infobase = cursor.take_uuid_opt()?.unwrap_or_default();
        let last_active_at = cursor.take_datetime_opt()?.unwrap_or_default();
        let hibernate = cursor.take_bool_opt()?.unwrap_or_default();
        let passive_session_hibernate_time = cursor.take_u32_be_opt()?.unwrap_or_default();
        let hibernate_session_terminate_time = cursor.take_u32_be_opt()?.unwrap_or_default();
        let license = {
            let count = cursor.take_u8()? as usize;
            if count == 0 { SessionLicenseRecord::default() } else { SessionLicenseRecord::decode(cursor)? }
        };
        let locale = cursor.take_str8_opt()?.unwrap_or_default();
        let process = cursor.take_uuid_opt()?.unwrap_or_default();
        let session_id = cursor.take_u32_be_opt()?.unwrap_or_default();
        let started_at = cursor.take_datetime_opt()?.unwrap_or_default();
        let user_name = cursor.take_str8_opt()?.unwrap_or_default();
        let memory_current = cursor.take_u64_be_opt()?.unwrap_or_default();
        let memory_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let memory_total = cursor.take_u64_be_opt()?.unwrap_or_default();
        let read_current = cursor.take_u64_be_opt()?.unwrap_or_default();
        let read_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let read_total = cursor.take_u64_be_opt()?.unwrap_or_default();
        let write_current = cursor.take_u64_be_opt()?.unwrap_or_default();
        let write_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let write_total = cursor.take_u64_be_opt()?.unwrap_or_default();
        let duration_current_service = cursor.take_u32_be_opt()?.unwrap_or_default();
        let duration_last_5min_service = cursor.take_u64_be_opt()?.unwrap_or_default();
        let duration_all_service = cursor.take_u32_be_opt()?.unwrap_or_default();
        let current_service_name = cursor.take_str8_opt()?.unwrap_or_default();
        let cpu_time_current = cursor.take_u64_be_opt()?.unwrap_or_default();
        let cpu_time_last_5min = cursor.take_u64_be_opt()?.unwrap_or_default();
        let cpu_time_total = cursor.take_u64_be_opt()?.unwrap_or_default();
        let data_separation = cursor.take_str8_opt()?.unwrap_or_default();
        let client_ip = cursor.take_str8_opt()?.unwrap_or_default();
        Ok(Self {
            session,
            app_id,
            blocked_by_dbms,
            blocked_by_ls,
            bytes_all,
            bytes_last_5min,
            calls_all,
            calls_last_5min,
            connection,
            dbms_bytes_all,
            dbms_bytes_last_5min,
            db_proc_info,
            db_proc_took,
            db_proc_took_at,
            duration_all,
            duration_all_dbms,
            duration_current,
            duration_current_dbms,
            duration_last_5min,
            duration_last_5min_dbms,
            host,
            infobase,
            last_active_at,
            hibernate,
            passive_session_hibernate_time,
            hibernate_session_terminate_time,
            license,
            locale,
            process,
            session_id,
            started_at,
            user_name,
            memory_current,
            memory_last_5min,
            memory_total,
            read_current,
            read_last_5min,
            read_total,
            write_current,
            write_last_5min,
            write_total,
            duration_current_service,
            duration_last_5min_service,
            duration_all_service,
            current_service_name,
            cpu_time_current,
            cpu_time_last_5min,
            cpu_time_total,
            data_separation,
            client_ip,
        })
    }
}

pub const RPC_SESSION_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SESSION_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SESSION_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SESSION_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SESSION_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SESSION_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_session_info_body(body: &[u8]) -> Result<SessionRecordRaw> {
    if body.is_empty() {
        return Err(RacError::Decode("session info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    SessionRecordRaw::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct SessionListRequest {
    pub cluster: Uuid16,
}

impl SessionListRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SessionInfoRequest {
    pub cluster: Uuid16,
    pub session: Uuid16,
}

impl SessionInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.session);
        Ok(())
    }
}




