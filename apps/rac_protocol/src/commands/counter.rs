use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct CounterListResp {
    pub counters: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn counter_list(client: &mut RacClient, cluster: crate::Uuid16) -> Result<CounterListResp> {
    let reply = client.call(RacRequest::CounterList { cluster })?;
    let body = rpc_body(&reply)?;
    let counters = parse_counter_list(body)?;
    Ok(CounterListResp {
        counters,
        raw_payload: Some(reply),
    })
}

fn parse_counter_list(body: &[u8]) -> Result<Vec<String>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let mut counters = Vec::new();
    while cursor.remaining_len() > 0 {
        if cursor.remaining_len() < 1 {
            return Err(RacError::Decode("counter record truncated"));
        }
        let name = cursor.take_str8()?;
        let _collection_time = cursor.take_u64_be()?;
        let _group = cursor.take_u8()?;
        let _filter_type = cursor.take_u8()?;
        let _filter = cursor.take_str8()?;
        let _duration = cursor.take_u8()?;
        let _cpu_time = cursor.take_u8()?;
        let _duration_dbms = cursor.take_u8()?;
        let _service = cursor.take_u8()?;
        let _memory = cursor.take_u8()?;
        let _read = cursor.take_u8()?;
        let _write = cursor.take_u8()?;
        let _dbms_bytes = cursor.take_u8()?;
        let _call = cursor.take_u8()?;
        let _number_of_active_sessions = cursor.take_u8()?;
        let _number_of_sessions = cursor.take_u8()?;
        let _descr = cursor.take_str8()?;
        counters.push(name);
    }
    Ok(counters)
}
