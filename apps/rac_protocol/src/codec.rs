use crate::rac_wire::{uuid_from_slice, WireError};
use crate::Uuid16;

pub struct RecordCursor<'a> {
    data: &'a [u8],
    pub off: usize,
}

impl<'a> RecordCursor<'a> {
    pub fn new(data: &'a [u8], off: usize) -> Self {
        Self { data, off }
    }

    pub fn remaining_len(&self) -> usize {
        self.data.len().saturating_sub(self.off)
    }

    pub fn remaining_slice(&self) -> &'a [u8] {
        if self.off >= self.data.len() {
            return &[];
        }
        &self.data[self.off..]
    }

    pub fn take_uuid(&mut self) -> Result<Uuid16, WireError> {
        if self.off + 16 > self.data.len() {
            return Err(WireError::Truncated("uuid"));
        }
        let start = self.off;
        let end = start + 16;
        let uuid = uuid_from_slice(&self.data[start..end])?;
        self.off = end;
        Ok(uuid)
    }

    pub fn take_uuid_opt(&mut self) -> Result<Option<Uuid16>, WireError> {
        if self.off + 16 > self.data.len() {
            return Ok(None);
        }
        let start = self.off;
        let end = start + 16;
        let uuid = uuid_from_slice(&self.data[start..end])?;
        self.off = end;
        Ok(Some(uuid))
    }

    pub fn take_str8(&mut self) -> Result<String, WireError> {
        if self.off >= self.data.len() {
            return Err(WireError::Truncated("str8 len"));
        }
        let len = self.data[self.off] as usize;
        let start = self.off + 1;
        let end = start + len;
        if end > self.data.len() {
            return Err(WireError::Truncated("str8 data"));
        }
        let value = std::str::from_utf8(&self.data[start..end])
            .map_err(|_| WireError::InvalidData("invalid utf-8"))?
            .to_string();
        self.off = end;
        Ok(value)
    }

    pub fn take_str8_opt(&mut self) -> Result<Option<String>, WireError> {
        if self.off >= self.data.len() {
            return Ok(None);
        }
        let len = self.data[self.off] as usize;
        let mut start = self.off + 1;
        if start >= self.data.len() {
            return Ok(None);
        }
        if self.data[start] == 1u8 {
            start += 1;
            if start > self.data.len() {
                return Ok(None);
            }
        }
        let end = start + len;
        if end > self.data.len() {
            return Ok(None);
        }
        let value = match std::str::from_utf8(&self.data[start..end]) {
            Ok(v) => v.to_string(),
            Err(_) => return Ok(None),
        };
        self.off = end;
        Ok(Some(value))
    }

    pub fn take_u32_be(&mut self) -> Result<u32, WireError> {
        if self.off + 4 > self.data.len() {
            return Err(WireError::Truncated("u32"));
        }
        let bytes = &self.data[self.off..self.off + 4];
        self.off += 4;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn take_u16_be(&mut self) -> Result<u16, WireError> {
        if self.off + 2 > self.data.len() {
            return Err(WireError::Truncated("u16"));
        }
        let bytes = &self.data[self.off..self.off + 2];
        self.off += 2;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub fn take_u16_le(&mut self) -> Result<u16, WireError> {
        if self.off + 2 > self.data.len() {
            return Err(WireError::Truncated("u16"));
        }
        let bytes = &self.data[self.off..self.off + 2];
        self.off += 2;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn take_u32_le(&mut self) -> Result<u32, WireError> {
        if self.off + 4 > self.data.len() {
            return Err(WireError::Truncated("u32"));
        }
        let bytes = &self.data[self.off..self.off + 4];
        self.off += 4;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn take_u32_be_opt(&mut self) -> Result<Option<u32>, WireError> {
        if self.off + 4 > self.data.len() {
            return Ok(None);
        }
        let bytes = &self.data[self.off..self.off + 4];
        self.off += 4;
        Ok(Some(u32::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ])))
    }

    pub fn take_u64_be_opt(&mut self) -> Result<Option<u64>, WireError> {
        if self.off + 8 > self.data.len() {
            return Ok(None);
        }
        let bytes = &self.data[self.off..self.off + 8];
        self.off += 8;
        Ok(Some(u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])))
    }

    pub fn take_u64_be(&mut self) -> Result<u64, WireError> {
        if self.off + 8 > self.data.len() {
            return Err(WireError::Truncated("u64"));
        }
        let bytes = &self.data[self.off..self.off + 8];
        self.off += 8;
        Ok(u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    pub fn take_datetime_opt(&mut self) -> Result<Option<String>, WireError> {
        let value = match self.take_u64_be_opt()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(v8_datetime_to_iso(value))
    }

    pub fn take_u8(&mut self) -> Result<u8, WireError> {
        let value = *self
            .data
            .get(self.off)
            .ok_or(WireError::Truncated("u8"))?;
        self.off += 1;
        Ok(value)
    }

    pub fn take_bytes(&mut self, len: usize) -> Result<Vec<u8>, WireError> {
        let start = self.off;
        let end = start
            .checked_add(len)
            .ok_or(WireError::Truncated("bytes"))?;
        if end > self.data.len() {
            return Err(WireError::Truncated("bytes"));
        }
        let out = self.data[start..end].to_vec();
        self.off = end;
        Ok(out)
    }

    pub fn take_f64_be(&mut self) -> Result<f64, WireError> {
        let bytes = self.take_bytes(8)?;
        Ok(f64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    pub fn take_u8_opt(&mut self) -> Result<Option<u8>, WireError> {
        if self.off >= self.data.len() {
            return Ok(None);
        }
        let value = self.data[self.off];
        self.off += 1;
        Ok(Some(value))
    }

    pub fn take_bool(&mut self) -> Result<bool, WireError> {
        let value = *self
            .data
            .get(self.off)
            .ok_or(WireError::Truncated("bool"))?
            != 0;
        self.off += 1;
        Ok(value)
    }

    pub fn take_bool_opt(&mut self) -> Result<Option<bool>, WireError> {
        if self.off >= self.data.len() {
            return Ok(None);
        }
        let value = self.data[self.off] != 0;
        self.off += 1;
        Ok(Some(value))
    }

    pub fn take_u16_le_opt(&mut self) -> Result<Option<u16>, WireError> {
        if self.off + 2 > self.data.len() {
            return Ok(None);
        }
        let bytes = &self.data[self.off..self.off + 2];
        self.off += 2;
        Ok(Some(u16::from_le_bytes([bytes[0], bytes[1]])))
    }

    pub fn take_i32_be_opt(&mut self) -> Result<Option<i32>, WireError> {
        if self.off + 4 > self.data.len() {
            return Ok(None);
        }
        let bytes = &self.data[self.off..self.off + 4];
        self.off += 4;
        Ok(Some(i32::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ])))
    }
}

pub fn v8_datetime_to_iso(raw: u64) -> Option<String> {
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
