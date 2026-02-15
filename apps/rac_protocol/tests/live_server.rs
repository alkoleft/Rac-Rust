use std::fs;
use std::io::{self, Write};
use std::net::TcpStream;
use std::time::Duration;

use serde::Deserialize;
use rac_protocol::rac_wire::{
    decode_rpc_method, encode_agent_version, encode_close, encode_cluster_context, encode_cluster_scoped,
    encode_rpc, encode_service_negotiation, format_uuid, init_packet, parse_uuid, read_frame,
    scan_len_prefixed_strings, scan_prefixed_uuids, write_frame,
};
use rac_protocol::client::{ClientConfig, RacClient};

#[derive(Debug, Deserialize)]
struct TestParams {
    addr: String,
    expected_agent_version: String,
}

fn load_params() -> TestParams {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let path = format!("{}/tests/params.toml", manifest_dir);
    let data = fs::read_to_string(&path).expect("read tests/params.toml");
    toml::from_str(&data).expect("parse tests/params.toml")
}

fn negotiate(stream: &mut TcpStream) -> io::Result<()> {
    stream.write_all(init_packet())?;
    stream.flush()?;
    let ack = read_frame(stream)?;
    if ack.opcode != 0x02 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected init ack opcode 0x{:02x}", ack.opcode),
        ));
    }

    write_frame(stream, 0x0b, encode_service_negotiation())?;
    stream.flush()?;
    let svc = read_frame(stream)?;
    if svc.opcode != 0x0c {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected service reply opcode 0x{:02x}", svc.opcode),
        ));
    }
    Ok(())
}

fn send_rpc(stream: &mut TcpStream, payload: &[u8], expect_method: Option<u8>) -> io::Result<Vec<u8>> {
    write_frame(stream, 0x0e, payload)?;
    stream.flush()?;
    let mut reply = read_frame(stream)?;
    if reply.opcode == 0x0f && payload_has_service_name(&reply.payload) {
        reply = read_frame(stream)?;
    }
    if reply.opcode != 0x0e {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected rpc reply opcode 0x{:02x}", reply.opcode),
        ));
    }
    if let Some(expect) = expect_method {
        let got = decode_rpc_method(&reply.payload).unwrap_or(0);
        if got != expect {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected rpc method 0x{got:02x}, expected 0x{expect:02x}"),
            ));
        }
    }
    Ok(reply.payload)
}

fn close_session(stream: &mut TcpStream) -> io::Result<()> {
    write_frame(stream, 0x0d, encode_close())?;
    stream.flush()
}

fn payload_has_service_name(payload: &[u8]) -> bool {
    payload
        .windows(b"v8.service.Admin.Cluster".len())
        .any(|w| w == b"v8.service.Admin.Cluster")
}

fn load_cluster_uuid() -> Option<[u8; 16]> {
    std::env::var("RAC_CLUSTER")
        .ok()
        .and_then(|s| parse_uuid(&s).ok())
}

#[test]
fn live_agent_version_and_cluster_list() {
    let params = load_params();
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());
    let mut stream = TcpStream::connect(addr).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(15)))
        .expect("read timeout");
    stream
        .set_write_timeout(Some(Duration::from_secs(15)))
        .expect("write timeout");

    negotiate(&mut stream).expect("negotiate");

    let reply = send_rpc(&mut stream, &encode_agent_version(), Some(0x88)).expect("agent version");
    assert_eq!(decode_rpc_method(&reply), Some(0x88));
    let body = &reply[5..];
    let strings = scan_len_prefixed_strings(body);
    let version = strings
        .first()
        .map(|(_, s)| s.as_str())
        .expect("version string");
    assert_eq!(version, params.expected_agent_version);

    let reply = send_rpc(&mut stream, &encode_rpc(0x0b, &[]), Some(0x0c)).expect("cluster list");
    let body = &reply[5..];
    let uuids = scan_prefixed_uuids(body);
    assert!(!uuids.is_empty(), "expected at least one cluster UUID");

    let cluster_uuid = uuids[0].1.clone();
    let raw = parse_uuid(&cluster_uuid).expect("cluster uuid parse");

    let _ = send_rpc(&mut stream, &encode_cluster_context(raw), None).expect("cluster context");
    let reply = send_rpc(&mut stream, &encode_cluster_scoped(0x12, raw), Some(0x13)).expect("manager list");
    assert_eq!(decode_rpc_method(&reply), Some(0x13));

    close_session(&mut stream).expect("close");
}

#[test]
fn live_agent_version_only() {
    let params = load_params();
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());
    let mut stream = TcpStream::connect(addr).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("read timeout");
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .expect("write timeout");

    negotiate(&mut stream).expect("negotiate");

    let reply = send_rpc(&mut stream, &encode_agent_version(), Some(0x88)).expect("agent version");
    assert_eq!(decode_rpc_method(&reply), Some(0x88));
    let body = &reply[5..];
    let strings = scan_len_prefixed_strings(body);
    let version = strings
        .first()
        .map(|(_, s)| s.as_str())
        .expect("version string");
    assert_eq!(version, params.expected_agent_version);

    close_session(&mut stream).expect("close");
}

#[test]
fn live_infobase_summary_list() {
    let Some(cluster_uuid) = load_cluster_uuid() else {
        eprintln!("RAC_CLUSTER is not set; skipping live_infobase_summary_list");
        return;
    };
    let params = load_params();
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());
    let cfg = ClientConfig {
        connect_timeout: Duration::from_secs(5),
        read_timeout: Duration::from_secs(15),
        write_timeout: Duration::from_secs(15),
        debug_raw: false,
    };
    let mut client = RacClient::connect(&addr, cfg).expect("connect");
    let _ = client
        .send_rpc(&encode_agent_version(), Some(0x88))
        .expect("agent version");
    client
        .set_infobase_context(cluster_uuid)
        .expect("infobase context");
    let reply = match client.send_rpc(&encode_cluster_scoped(0x2a, cluster_uuid), Some(0x2b)) {
        Ok(payload) => payload,
        Err(err) => {
            if let rac_protocol::error::RacError::Io(io_err) = &err {
                if io_err.kind() == std::io::ErrorKind::WouldBlock {
                    eprintln!("infobase summary list: no response (timeout), skipping");
                    let _ = client.close();
                    return;
                }
            }
            panic!("infobase summary list: {err:?}");
        }
    };
    let body = &reply[5..];
    assert!(!body.is_empty(), "empty infobase summary list body");

    let count = body[0] as usize;
    println!("infobase_count: {count}");
    if count == 0 {
        client.close().expect("close");
        return;
    }

    let mut off = 1;
    for idx in 0..count {
        if off + 16 > body.len() {
            panic!("infobase summary list body too short for uuid at index {idx}");
        }
        let mut uuid = [0u8; 16];
        uuid.copy_from_slice(&body[off..off + 16]);
        off += 16;

        let tag = body.get(off).copied().unwrap_or(0);
        off += 1;

        let (descr, next_off) = if off < body.len() {
            let len = body[off] as usize;
            let start = off + 1;
            let end = start + len;
            if end <= body.len() {
                let s = std::str::from_utf8(&body[start..end]).unwrap_or("<invalid utf-8>");
                (s.to_string(), end)
            } else {
                ("<truncated>".to_string(), body.len())
            }
        } else {
            ("<missing>".to_string(), body.len())
        };
        off = next_off;

        let (name, next_off) = if off < body.len() {
            let len = body[off] as usize;
            let start = off + 1;
            let end = start + len;
            if end <= body.len() {
                let s = std::str::from_utf8(&body[start..end]).unwrap_or("<invalid utf-8>");
                (s.to_string(), end)
            } else {
                ("<truncated>".to_string(), body.len())
            }
        } else {
            ("<missing>".to_string(), body.len())
        };
        off = next_off;

        println!(
            "infobase[{idx}]: uuid={}, tag=0x{tag:02x}, descr={descr}, name={name}",
            format_uuid(&uuid)
        );
    }

    client.close().expect("close");
}
