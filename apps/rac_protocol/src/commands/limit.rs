use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use super::{call_body, expect_ack};
use crate::Uuid16;

mod generated {
    include!("limit_generated.rs");
}

pub use generated::LimitRecord;

#[derive(Debug, Serialize)]
pub struct LimitListResp {
    pub limits: Vec<LimitRecord>,
}

#[derive(Debug, Serialize)]
pub struct LimitInfoResp {
    pub record: LimitRecord,
}

#[derive(Debug, Serialize, Clone)]
pub struct LimitUpdateReq {
    pub name: String,
    pub counter: String,
    pub action: u8,
    pub duration: u64,
    pub cpu_time: u64,
    pub memory: u64,
    pub read: u64,
    pub write: u64,
    pub duration_dbms: u64,
    pub dbms_bytes: u64,
    pub service: u64,
    pub call: u64,
    pub number_of_active_sessions: u64,
    pub number_of_sessions: u64,
    pub error_message: String,
    pub descr: String,
}

#[derive(Debug, Serialize)]
pub struct LimitUpdateResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct LimitRemoveResp {
    pub acknowledged: bool,
}

pub fn limit_list(client: &mut RacClient, cluster: Uuid16) -> Result<LimitListResp> {
    let body = call_body(client, RacRequest::LimitList { cluster })?;
    Ok(LimitListResp {
        limits: parse_limit_list_body(&body)?,
    })
}

pub fn limit_info(client: &mut RacClient, cluster: Uuid16, limit: &str) -> Result<LimitInfoResp> {
    let body = call_body(
        client,
        RacRequest::LimitInfo {
            cluster,
            limit: limit.to_string(),
        },
    )?;
    Ok(LimitInfoResp {
        record: parse_limit_info_body(&body)?,
    })
}

pub fn limit_update(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: LimitUpdateReq,
) -> Result<LimitUpdateResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::LimitUpdate {
        cluster,
        name: req.name,
        counter: req.counter,
        action: req.action,
        duration: req.duration,
        cpu_time: req.cpu_time,
        memory: req.memory,
        read: req.read,
        write: req.write,
        duration_dbms: req.duration_dbms,
        dbms_bytes: req.dbms_bytes,
        service: req.service,
        call: req.call,
        number_of_active_sessions: req.number_of_active_sessions,
        number_of_sessions: req.number_of_sessions,
        error_message: req.error_message,
        descr: req.descr,
    })?;
    expect_ack(&reply, "limit update expected ack")?;
    Ok(LimitUpdateResp {
        acknowledged: true,
    })
}

pub fn limit_remove(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    name: &str,
) -> Result<LimitRemoveResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::LimitRemove {
        cluster,
        name: name.to_string(),
    })?;
    expect_ack(&reply, "limit remove expected ack")?;
    Ok(LimitRemoveResp {
        acknowledged: true,
    })
}

fn parse_limit_list_body(body: &[u8]) -> Result<Vec<LimitRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut limits = Vec::with_capacity(count);
    for _ in 0..count {
        limits.push(parse_limit_record(&mut cursor)?);
    }
    Ok(limits)
}

fn parse_limit_info_body(body: &[u8]) -> Result<LimitRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("limit info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    parse_limit_record(&mut cursor)
}

fn parse_limit_record(cursor: &mut RecordCursor<'_>) -> Result<LimitRecord> {
    LimitRecord::decode(cursor).map_err(|_| RacError::Decode("limit record truncated"))
}

#[cfg(test)]
fn parse_limit_update_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("limit update ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

#[cfg(test)]
fn parse_limit_remove_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("limit remove ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::RacProtocolVersion;
    use crate::rac_wire::parse_frames;
    use crate::rac_wire::parse_uuid;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_limit_list_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_list_nonempty_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let limits = parse_limit_list_body(body).expect("limit list parse");

        assert_eq!(limits.len(), 2);
        assert_eq!(limits[0].name, "limit_codex_a");
        assert_eq!(limits[0].counter, "cpu");
        assert_eq!(limits[0].action, 2);
        assert_eq!(limits[0].cpu_time, 11);
        assert_eq!(limits[0].error_message, "limit_a");
        assert_eq!(limits[0].descr, "limit_a");

        assert_eq!(limits[1].name, "limit_codex_b");
        assert_eq!(limits[1].counter, "call");
        assert_eq!(limits[1].action, 3);
        assert_eq!(limits[1].call, 7);
        assert_eq!(limits[1].error_message, "limit_b");
        assert_eq!(limits[1].descr, "limit_b");
    }

    #[test]
    fn parse_limit_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_info_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert!(!frames.is_empty());
        let frame = frames.last().expect("frame");
        assert_eq!(frame.opcode, 0x0e);
        let body = rpc_body(&frame.payload).expect("rpc body");
        let record = parse_limit_info_body(body).expect("limit info parse");

        assert_eq!(record.name, "limit_codex_tmp");
        assert_eq!(record.counter, "cpu");
        assert_eq!(record.action, 2);
        assert_eq!(record.cpu_time, 10);
        assert_eq!(record.error_message, "limit_tmp");
        assert_eq!(record.descr, "limit_tmp");
    }

    #[test]
    fn parse_limit_update_ack_payload() {
        let payload = decode_hex_str("01000000");
        let acknowledged = parse_limit_update_ack(&payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_limit_update_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_update_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let acknowledged = parse_limit_update_ack(&frames[2].payload).expect("parse ack");
        assert!(acknowledged);
        let acknowledged = parse_limit_update_ack(&frames[3].payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_limit_remove_ack_payload() {
        let payload = decode_hex_str("01000000");
        let acknowledged = parse_limit_remove_ack(&payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_limit_remove_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_remove_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let ack_first = parse_limit_remove_ack(&frames[2].payload).expect("parse ack");
        assert!(ack_first);
        let ack_second = parse_limit_remove_ack(&frames[3].payload).expect("parse ack");
        assert!(ack_second);
    }

    #[test]
    fn encode_limit_update_request() {
        let expected = decode_hex_str(
            "01000001801619820ad36f4d8aa7161516b1dea0770f6c696d69745f636f6465785f746d7003637075020000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000096c696d69745f746d70096c696d69745f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let req = RacRequest::LimitUpdate {
            cluster,
            name: "limit_codex_tmp".to_string(),
            counter: "cpu".to_string(),
            action: 2,
            duration: 0,
            cpu_time: 10,
            memory: 0,
            read: 0,
            write: 0,
            duration_dbms: 0,
            dbms_bytes: 0,
            service: 0,
            call: 0,
            number_of_active_sessions: 0,
            number_of_sessions: 0,
            error_message: "limit_tmp".to_string(),
            descr: "limit_tmp".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_limit_remove_request() {
        let expected = decode_hex_str(
            "01000001811619820ad36f4d8aa7161516b1dea0770f6c696d69745f636f6465785f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let req = RacRequest::LimitRemove {
            cluster,
            name: "limit_codex_tmp".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }
}
