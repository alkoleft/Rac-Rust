use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

const INIT_PACKET: &[u8] = &[
    0x1c, 0x53, 0x57, 0x50, 0x01, 0x00, 0x01, 0x00, 0x01, 0x16, 0x01, 0x0f, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x2e, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x04, 0x00, 0x00, 0x07, 0xd0,
];

const SERVICE_NEGOTIATION: &[u8] = &[
    0x18, 0x76, 0x38, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x41, 0x64, 0x6d, 0x69,
    0x6e, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x04, 0x31, 0x36, 0x2e, 0x30, 0x80,
];

#[derive(Debug)]
struct Frame {
    opcode: u8,
    payload: Vec<u8>,
}

fn encode_varuint(mut value: usize) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut b = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            b |= 0x80;
        }
        out.push(b);
        if value == 0 {
            break;
        }
    }
    out
}

fn decode_varuint(stream: &mut TcpStream) -> io::Result<usize> {
    let mut shift = 0usize;
    let mut value = 0usize;
    loop {
        let mut b = [0u8; 1];
        stream.read_exact(&mut b)?;
        value |= ((b[0] & 0x7f) as usize) << shift;
        if b[0] & 0x80 == 0 {
            return Ok(value);
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

fn send_frame(stream: &mut TcpStream, opcode: u8, payload: &[u8]) -> io::Result<()> {
    stream.write_all(&[opcode])?;
    stream.write_all(&encode_varuint(payload.len()))?;
    stream.write_all(payload)?;
    stream.flush()
}

fn recv_frame(stream: &mut TcpStream) -> io::Result<Frame> {
    let mut opcode = [0u8; 1];
    stream.read_exact(&mut opcode)?;
    let len = decode_varuint(stream)?;
    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload)?;
    Ok(Frame {
        opcode: opcode[0],
        payload,
    })
}

fn payload_method(payload: &[u8]) -> Option<u8> {
    if payload.len() >= 5 && payload[0..4] == [0x01, 0x00, 0x00, 0x01] {
        Some(payload[4])
    } else {
        None
    }
}

fn format_uuid(bytes: &[u8]) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}

fn extract_strings(payload: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < payload.len() {
        let len = payload[i] as usize;
        let start = i + 1;
        let end = start + len;
        if len >= 3 && end <= payload.len() {
            if let Ok(s) = std::str::from_utf8(&payload[start..end]) {
                if s.chars().all(|c| !c.is_control()) {
                    out.push(s.to_string());
                }
            }
        }
        i += 1;
    }
    out
}

fn negotiate(stream: &mut TcpStream) -> io::Result<()> {
    stream.write_all(INIT_PACKET)?;
    let ack = recv_frame(stream)?;
    if ack.opcode != 0x02 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected init ack opcode 0x{:02x}", ack.opcode),
        ));
    }

    send_frame(stream, 0x0b, SERVICE_NEGOTIATION)?;
    let svc = recv_frame(stream)?;
    if svc.opcode != 0x0c {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected service reply opcode 0x{:02x}", svc.opcode),
        ));
    }
    Ok(())
}

fn close_session(stream: &mut TcpStream) -> io::Result<()> {
    send_frame(stream, 0x0d, &[0x01])
}

fn cmd_agent_version(stream: &mut TcpStream) -> io::Result<()> {
    send_frame(stream, 0x0e, &[0x01, 0x00, 0x00, 0x01, 0x87])?;
    let reply = recv_frame(stream)?;
    if reply.opcode != 0x0e || payload_method(&reply.payload) != Some(0x88) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unexpected agent version reply",
        ));
    }

    let strings = extract_strings(&reply.payload);
    if let Some(version) = strings.first() {
        println!("version: {version}");
    } else {
        println!("version payload: {}", hex(&reply.payload));
    }
    Ok(())
}

fn cmd_cluster_list(stream: &mut TcpStream) -> io::Result<()> {
    send_frame(stream, 0x0e, &[0x01, 0x00, 0x00, 0x01, 0x0b])?;
    let reply = recv_frame(stream)?;
    if reply.opcode != 0x0e || payload_method(&reply.payload) != Some(0x0c) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unexpected cluster list reply",
        ));
    }

    if reply.payload.len() >= 22 {
        let uuid = format_uuid(&reply.payload[6..22]);
        println!("cluster: {uuid}");
    } else {
        println!("cluster payload: {}", hex(&reply.payload));
    }

    for s in extract_strings(&reply.payload) {
        if s != "v8.service.Admin.Cluster" && s != "16.0" {
            println!("text: {s}");
        }
    }
    Ok(())
}

fn hex(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len() * 2);
    for b in data {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn usage() {
    eprintln!("Usage:");
    eprintln!("  rac_lite cluster-list <host:port>");
    eprintln!("  rac_lite agent-version <host:port>");
}

fn run() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        usage();
        std::process::exit(2);
    }

    let cmd = &args[1];
    let addr = &args[2];
    let mut stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    negotiate(&mut stream)?;
    match cmd.as_str() {
        "cluster-list" => cmd_cluster_list(&mut stream)?,
        "agent-version" => cmd_agent_version(&mut stream)?,
        _ => {
            usage();
            std::process::exit(2);
        }
    }
    close_session(&mut stream)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
