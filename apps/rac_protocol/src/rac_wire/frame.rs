use std::io::{self, Read, Write};

use crate::rac_wire::format::encode_varuint;
use crate::rac_wire::types::WireError;

#[derive(Debug, Clone)]
pub struct Frame {
    pub offset: usize,
    pub opcode: u8,
    pub len_field_size: usize,
    pub payload: Vec<u8>,
}

pub fn read_frame<R: Read>(reader: &mut R) -> io::Result<Frame> {
    let mut opcode = [0u8; 1];
    reader.read_exact(&mut opcode)?;
    let (len, len_field_size) = decode_varuint_from_reader(reader)?;
    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload)?;
    Ok(Frame {
        offset: 0,
        opcode: opcode[0],
        len_field_size,
        payload,
    })
}

pub fn write_frame<W: Write>(writer: &mut W, opcode: u8, payload: &[u8]) -> io::Result<()> {
    writer.write_all(&[opcode])?;
    writer.write_all(&encode_varuint(payload.len()))?;
    writer.write_all(payload)
}

pub fn parse_frames(data: &[u8], start_offset: usize) -> Result<Vec<Frame>, WireError> {
    let mut frames = Vec::new();
    let mut offset = start_offset;
    while offset + 2 <= data.len() {
        let opcode = data[offset];
        let (len, len_field_size) = decode_varuint(&data[offset + 1..])?;
        let start = offset + 1 + len_field_size;
        let end = start + len;
        if end > data.len() {
            return Err(WireError::Truncated("frame payload"));
        }
        frames.push(Frame {
            offset,
            opcode,
            len_field_size,
            payload: data[start..end].to_vec(),
        });
        offset = end;
    }
    Ok(frames)
}

pub fn detect_swp_init_len(data: &[u8]) -> Option<usize> {
    if data.len() < 4 || &data[..4] != b"\x1cSWP" {
        return None;
    }
    let needle = b"connect.timeout";
    let pos = data.windows(needle.len()).position(|w| w == needle)?;
    let after_key = pos + needle.len();
    if after_key + 1 > data.len() {
        return None;
    }
    let value_len = data[after_key] as usize;
    let init_len = after_key + 1 + value_len;
    if init_len <= data.len() {
        Some(init_len)
    } else {
        None
    }
}

fn decode_varuint(data: &[u8]) -> Result<(usize, usize), WireError> {
    let mut value: usize = 0;
    let mut shift = 0usize;
    for (idx, &b) in data.iter().enumerate() {
        let part = (b & 0x7f) as usize;
        value |= part << shift;
        if b & 0x80 == 0 {
            return Ok((value, idx + 1));
        }
        shift += 7;
        if shift >= usize::BITS as usize {
            return Err(WireError::InvalidData("varuint too large"));
        }
    }
    Err(WireError::Truncated("varuint"))
}

fn decode_varuint_from_reader<R: Read>(reader: &mut R) -> io::Result<(usize, usize)> {
    let mut shift = 0usize;
    let mut value = 0usize;
    let mut count = 0usize;
    loop {
        let mut b = [0u8; 1];
        reader.read_exact(&mut b)?;
        count += 1;
        value |= ((b[0] & 0x7f) as usize) << shift;
        if b[0] & 0x80 == 0 {
            return Ok((value, count));
        }
        shift += 7;
        if shift > 63 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "varuint length is too large",
            ));
        }
    }
}

