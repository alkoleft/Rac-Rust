use crate::client::RacClient;
use crate::error::Result;
use crate::Uuid16;

mod generated {
    include!("process_generated.rs");
}

pub use generated::{
    ProcessInfoResp,
    ProcessInfoRpc,
    ProcessLicense,
    ProcessListResp,
    ProcessListRpc,
    ProcessRecord,
};

pub fn process_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProcessListResp> {
    client.call_typed(ProcessListRpc { cluster })
}

pub fn process_info(
    client: &mut RacClient,
    cluster: Uuid16,
    process: Uuid16,
) -> Result<ProcessInfoResp> {
    client.call_typed(ProcessInfoRpc { cluster, process })
}

#[cfg(all(test, feature = "artifacts"))]
mod tests {
    use super::*;
    use crate::commands::parse_list_u8;
    use crate::commands::rpc_body;
    use crate::protocol::ProtocolVersion;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        let s = input.trim();
        assert!(s.len() % 2 == 0, "hex length must be even");
        let mut out = Vec::with_capacity(s.len() / 2);
        let bytes = s.as_bytes();
        for i in (0..bytes.len()).step_by(2) {
            let hi = (bytes[i] as char).to_digit(16).expect("hex hi");
            let lo = (bytes[i + 1] as char).to_digit(16).expect("hex lo");
            out.push(((hi << 4) | lo) as u8);
        }
        out
    }

    #[test]
    fn process_list_response_decodes_fields() {
        let hex = include_str!("../../../../artifacts/rac/process_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let records = parse_list_u8(body, |cursor| {
            ProcessRecord::decode(cursor, ProtocolVersion::V16_0)
        })
        .expect("process list");
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.turned_on, true);
        assert_eq!(record.running, true);
        assert_eq!(record.started_at, "2026-02-14T22:47:19");
        assert_eq!(record.use_status, 1);
        assert_eq!(record.available_performance, 153);
        assert_eq!(record.capacity, 1000);
        assert_eq!(record.connections, 7);
        assert_eq!(record.memory_size, 682224);
        assert_eq!(record.memory_excess_time, 0);
        assert_eq!(record.selection_size, 21944);
        assert!((record.avg_call_time - 4.115422347794385).abs() < 1e-9);
        assert!((record.avg_db_call_time - 0.0).abs() < 1e-12);
        assert!((record.avg_lock_call_time - 0.0001695679912504557).abs() < 1e-12);
        assert!((record.avg_server_call_time - 4.115252779803135).abs() < 1e-9);
        assert!((record.avg_threads - 1.0139867691851958).abs() < 1e-9);
        assert_eq!(record.reserve, false);
        assert_eq!(record.licenses.len(), 1);
        let license = &record.licenses[0];
        assert_eq!(
            license.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(license.key_series, "500000025347");
        assert_eq!(license.issued_by_server, true);
        assert_eq!(license.license_type, 0);
        assert_eq!(license.network_key, false);
        assert_eq!(license.max_users_all, 4);
        assert_eq!(license.max_users_current, 4);
        assert_eq!(license.server_address, "alko-home");
        assert_eq!(license.server_port, 1560);
        assert_eq!(license.process_id, "314150");
        assert!(license
            .full_presentation
            .contains("file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"));
    }

    #[test]
    fn process_info_response_decodes_fields() {
        let hex = include_str!("../../../../artifacts/rac/process_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = generated::parse_process_info_body(body, ProtocolVersion::V16_0)
            .expect("process info");
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.running, true);
        assert_eq!(record.started_at, "2026-02-14T22:47:19");
    }

    #[test]
    fn process_list_licenses_response_decodes_license_fields() {
        let hex = include_str!("../../../../artifacts/rac/process_list_licenses_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let records = parse_list_u8(body, |cursor| {
            ProcessRecord::decode(cursor, ProtocolVersion::V16_0)
        })
        .expect("process list");
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.licenses.len(), 1);
        let license = &record.licenses[0];
        assert_eq!(
            license.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(license.key_series, "500000025347");
        assert_eq!(license.issued_by_server, true);
        assert_eq!(license.license_type, 0);
        assert_eq!(license.network_key, false);
        assert_eq!(license.max_users_all, 4);
        assert_eq!(license.max_users_current, 4);
        assert_eq!(license.server_address, "alko-home");
        assert_eq!(license.server_port, 1560);
        assert_eq!(license.process_id, "314150");
        assert!(license
            .brief_presentation
            .contains("500000025347 4 4"));
        assert!(license
            .full_presentation
            .contains("file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"));
    }

    #[test]
    fn process_info_licenses_response_decodes_license_fields() {
        let hex = include_str!("../../../../artifacts/rac/process_info_licenses_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = generated::parse_process_info_body(body, ProtocolVersion::V16_0)
            .expect("process info licenses");
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.licenses.len(), 1);
        let license = &record.licenses[0];
        assert_eq!(
            license.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(license.key_series, "500000025347");
        assert_eq!(license.issued_by_server, true);
        assert_eq!(license.license_type, 0);
        assert_eq!(license.network_key, false);
        assert_eq!(license.max_users_all, 4);
        assert_eq!(license.max_users_current, 4);
        assert_eq!(license.server_address, "alko-home");
        assert_eq!(license.server_port, 1560);
        assert_eq!(license.process_id, "314150");
        assert!(license
            .brief_presentation
            .contains("500000025347 4 4"));
        assert!(license
            .full_presentation
            .contains("file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"));
    }
}
