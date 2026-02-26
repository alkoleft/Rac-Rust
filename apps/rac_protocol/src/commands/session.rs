use crate::client::RacClient;
use crate::commands::cluster_auth;
use crate::error::Result;
use crate::rpc::AckResponse;
use crate::Uuid16;

mod generated {
    include!("session_generated.rs");
}

pub use generated::{
    SessionInfoResp,
    SessionInfoRpc,
    SessionInterruptCurrentServerCallRpc,
    SessionLicense,
    SessionListResp,
    SessionListRpc,
    SessionRecord,
    SessionTerminateRpc,
};

pub fn session_list(client: &mut RacClient, cluster: Uuid16) -> Result<SessionListResp> {
    client.call_typed(SessionListRpc { cluster })
}

pub fn session_info(
    client: &mut RacClient,
    cluster: Uuid16,
    session: Uuid16,
) -> Result<SessionInfoResp> {
    client.call_typed(SessionInfoRpc { cluster, session })
}

pub fn session_terminate(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    cluster: Uuid16,
    session: Uuid16,
    error_message: String,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(SessionTerminateRpc {
        cluster,
        session,
        error_message,
    })
}

pub fn session_interrupt_current_server_call(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    cluster: Uuid16,
    session: Uuid16,
    error_message: String,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(SessionInterruptCurrentServerCallRpc {
        cluster,
        session,
        error_message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::rpc_body;
    use crate::protocol::ProtocolVersion;
    use crate::rpc::Response;
    use crate::rpc::Request;
    use crate::rac_wire::parse_uuid;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_session_list_sessions_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/session_list_response.hex");
        let payload = decode_hex_str(hex);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = SessionListResp::decode(&payload, protocol.as_ref())
            .expect("session list parse");
        let records = resp.records;
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
        let record = generated::parse_session_info_body(body).expect("session info parse");

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
        let record = generated::parse_session_info_body(body).expect("session info 1cv8c parse");

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
        let record =
            generated::parse_session_info_body(body).expect("session info 1cv8c dbproc parse");

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

    #[test]
    fn encode_session_terminate_request() {
        let expected = decode_hex_str(
            "01000001471619820ad36f4d8aa7161516b1dea0770000000000000000000000000000000014636f646578207465726d696e6174652074657374",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").unwrap();
        let session = parse_uuid("00000000-0000-0000-0000-000000000000").unwrap();
        let req = SessionTerminateRpc {
            cluster,
            session,
            error_message: "codex terminate test".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_session_interrupt_current_server_call_request() {
        let expected = decode_hex_str(
            "01000001751619820ad36f4d8aa7161516b1dea0770000000000000000000000000000000014636f64657820696e746572727570742074657374",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").unwrap();
        let session = parse_uuid("00000000-0000-0000-0000-000000000000").unwrap();
        let req = SessionInterruptCurrentServerCallRpc {
            cluster,
            session,
            error_message: "codex interrupt test".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }
}
