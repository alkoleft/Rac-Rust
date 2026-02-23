use crate::codec::RecordCursor;
use crate::rac_wire::types::WireError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwpValue {
    U32(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwpParam {
    pub key: String,
    pub value: SwpValue,
    pub value_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwpInit {
    pub version: u8,
    pub header_a: u16,
    pub header_b: u16,
    pub tag: u8,
    pub params: Vec<SwpParam>,
}

pub fn parse_swp_init(data: &[u8]) -> Result<(SwpInit, usize), WireError> {
    let mut cursor = RecordCursor::new(data, 0);
    let magic = cursor.take_u8()?;
    if magic != 0x1c {
        return Err(WireError::InvalidData("swp magic"));
    }
    let magic_tail = cursor.take_bytes(3)?;
    if magic_tail != b"SWP" {
        return Err(WireError::InvalidData("swp magic"));
    }

    let version = cursor.take_u8()?;
    let header_a = cursor.take_u16_be()?;
    let header_b = cursor.take_u16_be()?;
    let tag = cursor.take_u8()?;
    let count = cursor.take_u8()?;

    let mut params = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let key_len = cursor.take_u8()? as usize;
        let key_bytes = cursor.take_bytes(key_len)?;
        let key = std::str::from_utf8(&key_bytes)
            .map_err(|_| WireError::InvalidData("swp key utf-8"))?
            .to_string();
        let value_type = cursor.take_u8()?;
        let value = match value_type {
            0x04 => SwpValue::U32(cursor.take_u32_be()?),
            _ => return Err(WireError::InvalidData("swp value type")),
        };
        params.push(SwpParam {
            key,
            value,
            value_type,
        });
    }

    let consumed = cursor.off;
    Ok((
        SwpInit {
            version,
            header_a,
            header_b,
            tag,
            params,
        },
        consumed,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_swp_init_single_timeout() {
        let data = [
            0x1c, 0x53, 0x57, 0x50, 0x01, 0x00, 0x01, 0x00, 0x01, 0x16, 0x01, 0x0f,
            0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x74, 0x69, 0x6d, 0x65,
            0x6f, 0x75, 0x74, 0x04, 0x00, 0x00, 0x07, 0xd0,
        ];
        let (init, consumed) = parse_swp_init(&data).expect("swp init");
        assert_eq!(consumed, data.len());
        assert_eq!(init.version, 0x01);
        assert_eq!(init.header_a, 1);
        assert_eq!(init.header_b, 1);
        assert_eq!(init.tag, 0x16);
        assert_eq!(init.params.len(), 1);
        assert_eq!(init.params[0].key, "connect.timeout");
        assert_eq!(init.params[0].value_type, 0x04);
        assert_eq!(init.params[0].value, SwpValue::U32(2000));
    }
}
