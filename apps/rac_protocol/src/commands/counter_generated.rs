use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CounterRecord {
    pub name: String,
    pub collection_time: u64,
    pub group: u8,
    pub filter_type: u8,
    pub filter: String,
    pub duration: u8,
    pub cpu_time: u8,
    pub duration_dbms: u8,
    pub service: u8,
    pub memory: u8,
    pub read: u8,
    pub write: u8,
    pub dbms_bytes: u8,
    pub call: u8,
    pub number_of_active_sessions: u8,
    pub number_of_sessions: u8,
    pub descr: String,
}

impl CounterRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let name = cursor.take_str8()?;
        let collection_time = cursor.take_u64_be()?;
        let group = cursor.take_u8()?;
        let filter_type = cursor.take_u8()?;
        let filter = cursor.take_str8()?;
        let duration = cursor.take_u8()?;
        let cpu_time = cursor.take_u8()?;
        let duration_dbms = cursor.take_u8()?;
        let service = cursor.take_u8()?;
        let memory = cursor.take_u8()?;
        let read = cursor.take_u8()?;
        let write = cursor.take_u8()?;
        let dbms_bytes = cursor.take_u8()?;
        let call = cursor.take_u8()?;
        let number_of_active_sessions = cursor.take_u8()?;
        let number_of_sessions = cursor.take_u8()?;
        let descr = cursor.take_str8()?;
        Ok(Self {
            name,
            collection_time,
            group,
            filter_type,
            filter,
            duration,
            cpu_time,
            duration_dbms,
            service,
            memory,
            read,
            write,
            dbms_bytes,
            call,
            number_of_active_sessions,
            number_of_sessions,
            descr,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CounterValuesRecord {
    pub object: String,
    pub collection_time: u64,
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
    pub time: String,
}

impl CounterValuesRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let object = cursor.take_str8()?;
        let collection_time = cursor.take_u64_be()?;
        let duration = cursor.take_u64_be()?;
        let cpu_time = cursor.take_u64_be()?;
        let memory = cursor.take_u64_be()?;
        let read = cursor.take_u64_be()?;
        let write = cursor.take_u64_be()?;
        let duration_dbms = cursor.take_u64_be()?;
        let dbms_bytes = cursor.take_u64_be()?;
        let service = cursor.take_u64_be()?;
        let call = cursor.take_u64_be()?;
        let number_of_active_sessions = cursor.take_u64_be()?;
        let number_of_sessions = cursor.take_u64_be()?;
        let time = v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();
        Ok(Self {
            object,
            collection_time,
            duration,
            cpu_time,
            memory,
            read,
            write,
            duration_dbms,
            dbms_bytes,
            service,
            call,
            number_of_active_sessions,
            number_of_sessions,
            time,
        })
    }
}
