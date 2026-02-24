use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request, Response};
use crate::rpc::decode_utils::rpc_body;
use crate::rac_wire::{
    METHOD_SESSION_INFO_REQ, METHOD_SESSION_INFO_RESP, METHOD_SESSION_LIST_REQ,
    METHOD_SESSION_LIST_RESP,
};
use crate::Uuid16;

use super::call_body;
use super::parse_list_u8;

mod generated {
    include!("session_generated.rs");
}

pub use generated::{SessionLicense, SessionRecord};

#[derive(Debug, Serialize)]
pub struct SessionListResp {
    pub sessions: Vec<Uuid16>,
    pub records: Vec<SessionRecord>,
}

#[derive(Debug, Serialize)]
pub struct SessionInfoResp {
    pub session: Uuid16,
    pub record: SessionRecord,
    pub fields: Vec<String>,
}

struct SessionListRpc {
    cluster: Uuid16,
}

impl Request for SessionListRpc {
    type Response = SessionListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_SESSION_LIST_REQ,
            method_resp: Some(METHOD_SESSION_LIST_RESP),
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

impl Response for SessionListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let records = parse_session_list_records(body)?;
        Ok(Self {
            sessions: records.iter().map(|r| r.session).collect(),
            records,
        })
    }
}

pub fn session_list(client: &mut RacClient, cluster: Uuid16) -> Result<SessionListResp> {
    client.call_typed(SessionListRpc { cluster })
}

struct SessionInfoRpc {
    cluster: Uuid16,
    session: Uuid16,
}

impl Request for SessionInfoRpc {
    type Response = Vec<u8>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_SESSION_INFO_REQ,
            method_resp: Some(METHOD_SESSION_INFO_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(32);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.session);
        Ok(out)
    }
}

pub fn session_info(
    client: &mut RacClient,
    cluster: Uuid16,
    session: Uuid16,
) -> Result<SessionInfoResp> {
    let body = call_body(client, SessionInfoRpc { cluster, session })?;
    let record = parse_session_record_for_info(&body, session)?;
    let fields = collect_session_fields(&record);
    Ok(SessionInfoResp {
        session: record.session,
        record,
        fields,
    })
}

fn parse_session_record_for_info(data: &[u8], requested_session: Uuid16) -> Result<SessionRecord> {
    let record = generated::parse_session_info_body(data)?;
    if record.session != requested_session {
        return Err(RacError::Decode(
            "session info record does not match requested session",
        ));
    }
    Ok(record)
}

fn parse_session_list_records(body: &[u8]) -> Result<Vec<SessionRecord>> {
    parse_list_u8(body, SessionRecord::decode)
}

fn parse_session_record_1cv8c(data: &[u8]) -> Result<SessionRecord> {
    let mut cursor = RecordCursor::new(data, 0);
    SessionRecord::decode(&mut cursor)
}

fn collect_session_fields(record: &SessionRecord) -> Vec<String> {
    let mut out = Vec::new();
    push_if_nonempty(&mut out, &record.app_id);
    push_if_nonempty(&mut out, &record.db_proc_info);
    push_if_nonempty(&mut out, &record.db_proc_took_at);
    push_if_nonempty(&mut out, &record.host);
    push_if_nonempty(&mut out, &record.locale);
    push_if_nonempty(&mut out, &record.user_name);
    push_if_nonempty(&mut out, &record.started_at);
    push_if_nonempty(&mut out, &record.last_active_at);
    push_if_nonempty(&mut out, &record.client_ip);
    push_if_nonempty(&mut out, &record.current_service_name);
    push_if_nonempty(&mut out, &record.data_separation);
    push_if_nonempty(&mut out, &record.license.file_name);
    push_if_nonempty(&mut out, &record.license.brief_presentation);
    push_if_nonempty(&mut out, &record.license.full_presentation);
    push_if_nonempty(&mut out, &record.license.server_address);
    push_if_nonempty(&mut out, &record.license.process_id);
    push_if_nonempty(&mut out, &record.license.key_series);
    out
}

fn push_if_nonempty(out: &mut Vec<String>, value: &str) {
    if !value.is_empty() {
        out.push(value.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_session_list_sessions_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/session_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let records = parse_session_list_records(body).expect("session list parse");
        let sessions: Vec<_> = records.iter().map(|r| r.session).collect();

        assert_eq!(sessions.len(), 3);
        assert_eq!(
            sessions[0],
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").unwrap()
        );
        assert_eq!(
            sessions[1],
            crate::rac_wire::parse_uuid("56bde8c0-d008-4d33-a6b9-8db9b6f82de5").unwrap()
        );
        assert_eq!(
            sessions[2],
            crate::rac_wire::parse_uuid("eb61231d-7bee-4a06-8869-41f70e2289de").unwrap()
        );

        assert_eq!(records[0].app_id, "1CV8C");
        assert_eq!(records[1].app_id, "Designer");
        assert_eq!(records[2].app_id, "SystemBackgroundJob");
        assert_eq!(records[0].client_ip, "127.0.0.1");
        assert_eq!(records[1].locale, "ru_RU");
        assert_eq!(records[2].session_id, 5);
        assert_eq!(records[2].dbms_bytes_all, 3088);
    }

    #[test]
    fn parse_session_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/session_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = parse_session_record_1cv8c(body).expect("session info parse");

        assert_eq!(
            record.session,
            crate::rac_wire::parse_uuid("56bde8c0-d008-4d33-a6b9-8db9b6f82de5").unwrap()
        );
        assert_eq!(record.app_id, "Designer");
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.user_name, "DefUser");
        assert_eq!(record.client_ip, "127.0.0.1");
        assert_eq!(record.session_id, 1);
        assert_eq!(record.bytes_all, 253146);
        assert_eq!(record.dbms_bytes_all, 654414);
        assert_eq!(record.cpu_time_total, 1357);
    }

    #[test]
    fn parse_session_info_1cv8c_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/session_info_response_1cv8c.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let requested =
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").expect("uuid");
        let record =
            parse_session_record_for_info(body, requested).expect("session info 1cv8c parse");

        assert_eq!(
            record.session,
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").unwrap()
        );
        assert_eq!(record.app_id, "1CV8C");
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.locale, "ru");
        assert_eq!(record.user_name, "DefUser");
        assert_eq!(record.started_at, "2026-02-15T00:10:57");
        assert_eq!(record.client_ip, "127.0.0.1");
        assert_eq!(record.session_id, 3);
        assert_eq!(record.bytes_all, 7807077);
        assert_eq!(record.dbms_bytes_all, 10914466);
        let lic = &record.license;
        assert_eq!(
            lic.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(lic.key_series, "500000025347");
        assert_eq!(lic.max_users_all, 4);
        assert_eq!(lic.max_users_current, 4);
    }

    #[test]
    fn parse_session_info_1cv8c_dbproc_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/session_info_response_1cv8c_dbproc.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let requested =
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").expect("uuid");
        let record = parse_session_record_for_info(body, requested)
            .expect("session info 1cv8c dbproc parse");

        assert_eq!(
            record.connection,
            crate::rac_wire::parse_uuid("e41e750e-56d7-40fb-b2e0-5e71b8e8f508").unwrap()
        );
        assert_eq!(
            record.process,
            crate::rac_wire::parse_uuid("f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5").unwrap()
        );
        assert_eq!(record.db_proc_info, "5719");
        assert_eq!(record.blocked_by_ls, 6);
        assert_eq!(record.db_proc_took, 18982);
        assert_eq!(record.duration_current, 20172);
        assert_eq!(record.memory_current, 18446744073709504536);
        assert_eq!(record.read_current, 16176);
        assert_eq!(record.duration_current_service, 0);
        assert_eq!(record.cpu_time_current, 1051);
        assert_eq!(record.last_active_at, "2026-02-16T00:28:41");
        assert_eq!(record.data_separation, "''");
        assert_eq!(record.client_ip, "127.0.0.1");
        let lic = &record.license;
        assert_eq!(lic.process_id, "381094");
        assert_eq!(lic.brief_presentation, "Клиент, 500000025347 4 4");
    }
}
