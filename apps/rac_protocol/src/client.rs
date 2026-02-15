use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::error::{RacError, Result};
use crate::rac_wire::{
    decode_rpc_method, encode_close, encode_cluster_context, encode_infobase_context,
    encode_service_negotiation, init_packet, read_frame, write_frame, OPCODE_CLOSE, OPCODE_INIT_ACK,
    OPCODE_RPC, OPCODE_SERVICE_ACK, OPCODE_SERVICE_NEGOTIATION,
};
use crate::Uuid16;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub debug_raw: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(5),
            write_timeout: Duration::from_secs(5),
            debug_raw: false,
        }
    }
}

pub struct RacClient {
    stream: TcpStream,
    current_cluster: Option<Uuid16>,
    current_infobase: Option<Uuid16>,
    debug_raw: bool,
}

impl RacClient {
    pub fn connect(addr: &str, cfg: ClientConfig) -> Result<Self> {
        let mut addrs = addr.to_socket_addrs()?;
        let socket_addr = addrs
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid address"))?;
        let stream = TcpStream::connect_timeout(&socket_addr, cfg.connect_timeout)?;
        stream.set_read_timeout(Some(cfg.read_timeout))?;
        stream.set_write_timeout(Some(cfg.write_timeout))?;
        stream.set_nodelay(true)?;

        let mut client = Self {
            stream,
            current_cluster: None,
            current_infobase: None,
            debug_raw: cfg.debug_raw,
        };
        client.negotiate()?;
        Ok(client)
    }

    pub fn close(mut self) -> Result<()> {
        write_frame(&mut self.stream, OPCODE_CLOSE, encode_close())?;
        self.stream.flush()?;
        Ok(())
    }

    pub fn send_rpc(&mut self, payload: &[u8], expect_method: Option<u8>) -> Result<Vec<u8>> {
        write_frame(&mut self.stream, OPCODE_RPC, payload)?;
        self.stream.flush()?;

        for _ in 0..3 {
            let reply = read_frame(&mut self.stream)?;
            if reply.opcode != OPCODE_RPC {
                if reply.opcode == 0x0f && payload_has_service_name(&reply.payload) {
                    continue;
                }
                if self.debug_raw {
                    log_frame("rpc-unexpected-opcode", &reply);
                }
                let head = format_payload_head(&reply.payload, 24);
                return Err(RacError::ProtocolMessage(format!(
                    "unexpected opcode in rpc reply: got 0x{:02x}, expected 0x{:02x}, payload_len={}, payload_head={}",
                    reply.opcode,
                    OPCODE_RPC,
                    reply.payload.len(),
                    head
                )));
            }

            if reply.payload == [0x01, 0x00, 0x00, 0x00] {
                if expect_method.is_some() {
                    continue;
                }
                return Ok(reply.payload);
            }

            if let Some(expect) = expect_method {
                let got = match decode_rpc_method(&reply.payload) {
                    Some(method) => method,
                    None => {
                        if payload_has_service_name(&reply.payload) {
                            continue;
                        }
                        if self.debug_raw {
                            log_frame("rpc-missing-header", &reply);
                        }
                        let head = format_payload_head(&reply.payload, 24);
                        return Err(RacError::ProtocolMessage(format!(
                            "missing rpc header: payload_len={}, payload_head={}",
                            reply.payload.len(),
                            head
                        )));
                    }
                };
                if got != expect {
                    if self.debug_raw {
                        log_frame("rpc-unexpected-method", &reply);
                    }
                    return Err(RacError::UnexpectedMethod { got, expected: expect });
                }
            }

            return Ok(reply.payload);
        }

        Err(RacError::Protocol("rpc reply not received"))
    }

    pub fn set_cluster_context(&mut self, cluster: Uuid16) -> Result<()> {
        if self.current_cluster == Some(cluster) {
            return Ok(());
        }
        let payload = encode_cluster_context(cluster);
        if let Err(err) = self.send_rpc(&payload, None) {
            if let RacError::Io(io_err) = &err {
                if io_err.kind() == io::ErrorKind::WouldBlock {
                    self.current_cluster = Some(cluster);
                    self.current_infobase = None;
                    return Ok(());
                }
            }
            return Err(err);
        }
        self.current_cluster = Some(cluster);
        self.current_infobase = None;
        Ok(())
    }

    pub fn set_infobase_context(&mut self, cluster: Uuid16) -> Result<()> {
        if self.current_infobase == Some(cluster) {
            return Ok(());
        }
        let payload = encode_infobase_context(cluster);
        if let Err(err) = self.send_rpc(&payload, None) {
            if let RacError::Io(io_err) = &err {
                if io_err.kind() == io::ErrorKind::WouldBlock {
                    self.current_infobase = Some(cluster);
                    return Ok(());
                }
            }
            return Err(err);
        }
        self.current_infobase = Some(cluster);
        Ok(())
    }

    fn negotiate(&mut self) -> Result<()> {
        self.send_init()?;
        write_frame(&mut self.stream, OPCODE_SERVICE_NEGOTIATION, encode_service_negotiation())?;
        self.stream.flush()?;
        let svc = read_frame(&mut self.stream)?;
        if svc.opcode != OPCODE_SERVICE_ACK {
            if self.debug_raw {
                log_frame("service-ack-unexpected", &svc);
            }
            return Err(RacError::Protocol("unexpected service negotiation reply"));
        }
        Ok(())
    }

    fn send_init(&mut self) -> io::Result<()> {
        self.stream.write_all(init_packet())?;
        self.stream.flush()?;
        let ack = read_frame(&mut self.stream)?;
        if ack.opcode != OPCODE_INIT_ACK {
            if self.debug_raw {
                log_frame("init-ack-unexpected", &ack);
            }
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected init ack opcode 0x{:02x}", ack.opcode),
            ));
        }
        Ok(())
    }
}

fn format_payload_head(payload: &[u8], max_len: usize) -> String {
    let take = payload.len().min(max_len);
    if take == 0 {
        return "<empty>".to_string();
    }
    let mut out = String::new();
    for (idx, byte) in payload[..take].iter().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02x}"));
    }
    if payload.len() > take {
        out.push_str(" ...");
    }
    out
}

fn payload_has_service_name(payload: &[u8]) -> bool {
    payload
        .windows(b"v8.service.Admin.Cluster".len())
        .any(|w| w == b"v8.service.Admin.Cluster")
}

fn log_frame(label: &str, frame: &crate::rac_wire::Frame) {
    let mut hex = String::new();
    for (idx, b) in frame.payload.iter().enumerate() {
        if idx > 0 {
            hex.push(' ');
        }
        hex.push_str(&format!("{b:02x}"));
    }
    eprintln!(
        "rac_lite debug: {label}: opcode=0x{:02x} len={} payload_hex={}",
        frame.opcode,
        frame.payload.len(),
        hex
    );
}
