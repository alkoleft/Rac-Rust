use crate::{rac_wire::take_u64_le, Uuid16};

use super::{take_str8, take_u32_be, take_u64_be, take_uuid16, RacError, Result};

pub(in crate::commands) struct RecordCursor<'a> {
    data: &'a [u8],
    pub off: usize,
}

impl<'a> RecordCursor<'a> {
    pub(in crate::commands) fn new(data: &'a [u8], off: usize) -> Self {
        Self { data, off }
    }

    pub(in crate::commands) fn seek(&mut self, target: usize) -> Option<()> {
        if target < self.off || target > self.data.len() {
            return None;
        }
        self.skip(target - self.off).ok()
    }

    pub(in crate::commands) fn skip(&mut self, n: usize) -> Result<()> {
        let next = self
            .off
            .checked_add(n)
            .ok_or(RacError::Decode("session record: cursor overflow"))?;
        if next > self.data.len() {
            return Err(RacError::Decode("session record: truncated while skipping"));
        }
        self.off = next;
        Ok(())
    }

    pub(in crate::commands) fn take_uuid(&mut self) -> Result<Uuid16> {
        let (uuid, next) = take_uuid16(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated uuid field"))?;
        self.off = next;
        Ok(uuid)
    }

    pub(in crate::commands) fn take_uuid_opt(&mut self) -> Option<Uuid16> {
        let (uuid, next) = take_uuid16(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated uuid field"))
            .ok()?;
        self.off = next;
        Some(uuid)
    }

    pub(in crate::commands) fn take_str8(&mut self) -> Result<String> {
        let (value, next) = take_str8(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated str8 field"))?;
        self.off = next;
        Ok(value)
    }

    pub(in crate::commands) fn take_str8_opt(&mut self) -> Option<String> {
        let len = *self.data.get(self.off)? as usize;
        let mut start = self.off + 1;

        if (self.data[start] == 1u8) {
            // TODO
            start += 1;
        }
        let end = start + len;
        if end > self.data.len() {
            return None;
        }
        let value = std::str::from_utf8(&self.data[start..end])
            .ok()?
            .to_string();
        self.off = end;
        Some(value)
    }

    pub(in crate::commands) fn take_u32_be(&mut self) -> u32 {
        let (value, next) = take_u32_be(self.data, self.off).unwrap();
        self.off += 4;
        value
    }

    pub(in crate::commands) fn take_u32_be_opt(&mut self) -> Option<u32> {
        Some(self.take_u32_be())
    }

    pub(in crate::commands) fn take_u64_be_opt(&mut self) -> Option<u64> {
        let (value, next) = take_u64_be(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated u32 field"))
            .ok()?;
        self.off += 8;
        Some(value)
    }

    pub(in crate::commands) fn take_datetime_opt(&mut self) -> Option<String> {
        let (value, next) = take_u64_be(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated u32 field"))
            .ok()?;
        self.off = next;
        v8_datetime_to_iso(value)
    }

    pub(in crate::commands) fn take_u8(&mut self) -> u8 {
        let value = *self.data.get(self.off).unwrap();
        self.off += 1;
        value
    }

    pub(in crate::commands) fn take_u8_opt(&mut self) -> Option<u8> {
        Some(self.take_u8())
    }

    pub(in crate::commands) fn take_bool(&mut self) -> bool {
        let value = *self.data.get(self.off).unwrap() != 0;
        self.off += 1;
        value
    }
    pub(in crate::commands) fn take_bool_opt(&mut self) -> Option<bool> {
        Some(self.take_bool())
    }

    pub(in crate::commands) fn take_u16_le_opt(&mut self) -> Option<u16> {
        let bytes = self.data.get(self.off..self.off + 2)?;
        self.off += 2;
        Some(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub(in crate::commands) fn take_i32_be_opt(&mut self) -> Option<i32> {
        let bytes = self.data.get(self.off..self.off + 4)?;
        self.off += 4;
        Some(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

pub(in crate::commands) fn v8_datetime_to_iso(raw: u64) -> Option<String> {
    // 1C timestamp observed in captures: 1 unit = 1/10000 second,
    // epoch offset equals Unix epoch at 621355968000000.
    const UNIX_EPOCH_OFFSET: i128 = 621_355_968_000_000;
    let raw_i = i128::from(raw);
    if raw_i < UNIX_EPOCH_OFFSET {
        return None;
    }
    let unix_secs = (raw_i - UNIX_EPOCH_OFFSET) / 10_000;
    let unix_secs = i64::try_from(unix_secs).ok()?;

    let days = unix_secs.div_euclid(86_400);
    let sod = unix_secs.rem_euclid(86_400);
    let hour = sod / 3_600;
    let minute = (sod % 3_600) / 60;
    let second = sod % 60;

    let (year, month, day) = civil_from_days(days);
    Some(format!(
        "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}"
    ))
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i64, i64, i64) {
    // Howard Hinnant's civil_from_days algorithm.
    // See: https://howardhinnant.github.io/date_algorithms.html#civil_from_days
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096).div_euclid(365);
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let d = doy - (153 * mp + 2).div_euclid(5) + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year, i64::from(m), i64::from(d))
}
