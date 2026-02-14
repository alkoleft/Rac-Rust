use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Frame<'a> {
    offset: usize,
    opcode: u8,
    len_field_size: usize,
    len: usize,
    payload: &'a [u8],
}

fn parse_frames(data: &[u8], start_offset: usize) -> Vec<Frame<'_>> {
    let mut frames = Vec::new();
    let mut i = start_offset;
    while i + 2 <= data.len() {
        let opcode = data[i];
        let Some((len, len_field_size)) = decode_varuint(&data[i + 1..]) else {
            break;
        };
        let start = i + 1 + len_field_size;
        let end = start + len;
        if end > data.len() {
            break;
        }
        frames.push(Frame {
            offset: i,
            opcode,
            len_field_size,
            len,
            payload: &data[start..end],
        });
        i = end;
    }
    frames
}

fn decode_varuint(data: &[u8]) -> Option<(usize, usize)> {
    let mut value: usize = 0;
    let mut shift = 0usize;
    for (idx, &b) in data.iter().enumerate() {
        let part = (b & 0x7f) as usize;
        value |= part << shift;
        if b & 0x80 == 0 {
            return Some((value, idx + 1));
        }
        shift += 7;
        if shift >= usize::BITS as usize {
            return None;
        }
    }
    None
}

fn rpc_method_id(payload: &[u8]) -> Option<(usize, usize)> {
    if payload.len() < 5 {
        return None;
    }
    if payload[0..4] != [0x01, 0x00, 0x00, 0x01] {
        return None;
    }
    Some((payload[4] as usize, 5))
}

fn find_swp_init_len(data: &[u8]) -> Option<usize> {
    if data.len() < 4 || &data[..4] != b"\x1cSWP" {
        return None;
    }

    let needle = b"connect.timeout";
    let pos = data.windows(needle.len()).position(|w| w == needle)?;
    let after_key = pos + needle.len();
    if after_key + 5 > data.len() {
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

fn hex(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len() * 2);
    for b in data {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn looks_text(s: &str) -> bool {
    let mut printable = 0usize;
    let mut total = 0usize;
    for c in s.chars() {
        total += 1;
        if !c.is_control() {
            printable += 1;
        }
    }
    total >= 3 && printable * 100 / total >= 90
}

fn payload_strings(payload: &[u8]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < payload.len() {
        let len = payload[i] as usize;
        let start = i + 1;
        let end = start + len;
        if len > 0 && end <= payload.len() {
            if let Ok(s) = std::str::from_utf8(&payload[start..end]) {
                if looks_text(s) {
                    out.push((i, s.to_string()));
                }
            }
        }
        i += 1;
    }
    out
}

fn uuid_candidates(payload: &[u8]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for i in 0..payload.len() {
        if payload[i] != 0x19 {
            continue;
        }
        if i + 17 <= payload.len() {
            let raw = &payload[i + 1..i + 17];
            let canonical = format!(
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                raw[0],
                raw[1],
                raw[2],
                raw[3],
                raw[4],
                raw[5],
                raw[6],
                raw[7],
                raw[8],
                raw[9],
                raw[10],
                raw[11],
                raw[12],
                raw[13],
                raw[14],
                raw[15]
            );
            out.push((i, canonical));
        }
    }
    out
}

fn usage(bin: &str) {
    eprintln!("Usage: {bin} <stream.bin>");
}

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap_or_else(|| "rac_decode".to_string());
    let Some(path) = args.next() else {
        usage(&bin);
        std::process::exit(2);
    };
    if args.next().is_some() {
        usage(&bin);
        std::process::exit(2);
    }

    let file = Path::new(&path);
    let data = match fs::read(file) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("read error: {e}");
            std::process::exit(1);
        }
    };

    println!("file={}", file.display());
    println!("size={}", data.len());
    let mut start = 0usize;
    if let Some(init_len) = find_swp_init_len(&data) {
        println!(
            "init_packet offset=0x0 len={} hex={}",
            init_len,
            hex(&data[..init_len])
        );
        start = init_len;
    }

    let frames = parse_frames(&data, start);
    println!("frames={} start_offset=0x{:x}", frames.len(), start);

    for (idx, frame) in frames.iter().enumerate() {
        println!(
            "frame={} offset=0x{:x} opcode=0x{:02x} len={} len_field_size={}",
            idx + 1,
            frame.offset,
            frame.opcode,
            frame.len,
            frame.len_field_size
        );
        println!("  payload_hex={}", hex(frame.payload));
        if let Some((method, head_size)) = rpc_method_id(frame.payload) {
            println!("  rpc_method_id=0x{method:x} ({method}) rpc_head_size={head_size}");
        }

        let strings = payload_strings(frame.payload);
        if !strings.is_empty() {
            for (off, s) in strings {
                println!("  text_at=0x{:x} value={}", off, s);
            }
        }

        let uuids = uuid_candidates(frame.payload);
        if !uuids.is_empty() {
            for (off, u) in uuids {
                println!("  uuid_at=0x{:x} value={}", off, u);
            }
        }
    }
}
