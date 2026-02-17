use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct AgentVersionResp {
    pub version: Option<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn agent_version(client: &mut RacClient) -> Result<AgentVersionResp> {
    let reply = client.call(RacRequest::AgentVersion)?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let version = if cursor.remaining_len() == 0 {
        None
    } else {
        Some(cursor.take_str8()?)
    };
    Ok(AgentVersionResp {
        version,
        raw_payload: Some(reply),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rac_wire::METHOD_AGENT_VERSION_RESP;

    fn rpc_with_body(method: u8, body: &[u8]) -> Vec<u8> {
        let mut out = vec![0x01, 0x00, 0x00, 0x01, method];
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn parse_agent_version() {
        let mut body = Vec::new();
        body.push(0x05);
        body.extend_from_slice(b"1.2.3");
        let payload = rpc_with_body(METHOD_AGENT_VERSION_RESP, &body);
        let body = rpc_body(&payload).unwrap();
        let mut cursor = RecordCursor::new(body, 0);
        let version = cursor.take_str8().unwrap();
        assert_eq!(version, "1.2.3");
    }
}
