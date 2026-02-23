mod debug;
mod handshake;
mod protocol;
mod transport;

use std::io;
use std::time::Duration;

use crate::client::debug::{format_payload_head, log_frame};
use crate::client::handshake::negotiate;
use crate::client::protocol::RacProtocol;
use crate::client::transport::RacTransport;
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};

pub use protocol::{RacProtocolVersion, RacRequest};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub debug_raw: bool,
    pub protocol: RacProtocolVersion,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(5),
            write_timeout: Duration::from_secs(5),
            debug_raw: false,
            protocol: RacProtocolVersion::default(),
        }
    }
}

pub struct RacClient {
    transport: RacTransport,
    protocol: Box<dyn RacProtocol>,
    protocol_version: RacProtocolVersion,
    current_cluster: Option<crate::Uuid16>,
    current_infobase: Option<crate::Uuid16>,
    debug_raw: bool,
}

impl RacClient {
    pub fn connect(addr: &str, cfg: ClientConfig) -> Result<Self> {
        let protocol = cfg.protocol.boxed();
        Self::connect_with_protocol(addr, cfg, protocol)
    }

    pub fn connect_with_protocol(
        addr: &str,
        cfg: ClientConfig,
        protocol: Box<dyn RacProtocol>,
    ) -> Result<Self> {
        let transport = RacTransport::connect(
            addr,
            cfg.connect_timeout,
            cfg.read_timeout,
            cfg.write_timeout,
        )?;

        let mut client = Self {
            transport,
            protocol,
            protocol_version: cfg.protocol,
            current_cluster: None,
            current_infobase: None,
            debug_raw: cfg.debug_raw,
        };

        negotiate(
            &mut client.transport,
            client.protocol.as_ref(),
            client.debug_raw,
        )?;
        Ok(client)
    }

    pub fn close(mut self) -> Result<()> {
        self.transport
            .write_frame(self.protocol.opcode_close(), self.protocol.close_payload())?;
        self.transport.flush()?;
        Ok(())
    }

    pub fn protocol_name(&self) -> &'static str {
        self.protocol.name()
    }

    pub fn protocol_version(&self) -> RacProtocolVersion {
        self.protocol_version
    }

    pub fn call(&mut self, request: RacRequest) -> Result<Vec<u8>> {
        let required = self.protocol.required_context(&request);
        if let Some(cluster) = required.cluster {
            self.ensure_cluster_context(cluster)?;
        }
        if let Some(cluster) = required.infobase_cluster {
            self.ensure_infobase_context(cluster)?;
        }

        let serialized = self.protocol.serialize(request)?;
        self.send_rpc_raw(&serialized.payload, serialized.expect_method)
    }

    fn ensure_cluster_context(&mut self, cluster: crate::Uuid16) -> Result<()> {
        if self.current_cluster == Some(cluster) {
            return Ok(());
        }

        let serialized = self.protocol.serialize_set_cluster_context(cluster)?;

        if let Err(err) = self.send_rpc_raw(&serialized.payload, serialized.expect_method) {
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

    fn ensure_infobase_context(&mut self, cluster: crate::Uuid16) -> Result<()> {
        if self.current_infobase == Some(cluster) {
            return Ok(());
        }

        let serialized = self.protocol.serialize_set_infobase_context(cluster)?;

        if let Err(err) = self.send_rpc_raw(&serialized.payload, serialized.expect_method) {
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

    fn send_rpc_raw(&mut self, payload: &[u8], expect_method: Option<u8>) -> Result<Vec<u8>> {
        self.transport
            .write_frame(self.protocol.opcode_rpc(), payload)?;
        self.transport.flush()?;

        for _ in 0..3 {
            let reply = self.transport.read_frame()?;
            if reply.opcode == 0x0f {
                continue;
            }
            if reply.opcode != self.protocol.opcode_rpc() {
                if self.debug_raw {
                    log_frame("rpc-unexpected-opcode", &reply);
                }
                let head = format_payload_head(&reply.payload, 24);
                return Err(RacError::ProtocolMessage(format!(
                    "unexpected opcode in rpc reply: got 0x{:02x}, expected 0x{:02x}, payload_len={}, payload_head={}",
                    reply.opcode,
                    self.protocol.opcode_rpc(),
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
                let got = match self.protocol.decode_rpc_method_id(&reply.payload) {
                    Some(method) => method,
                    None => {
                        if is_service_notice(&reply.payload) {
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
                    return Err(RacError::UnexpectedMethod {
                        got,
                        expected: expect,
                    });
                }
            }

            return Ok(reply.payload);
        }

        Err(RacError::Protocol("rpc reply not received"))
    }
}

fn is_service_notice(payload: &[u8]) -> bool {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 4 {
        return false;
    }
    let head = match cursor.take_bytes(4) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    head == [0x01, 0x00, 0x00, 0xff]
}
