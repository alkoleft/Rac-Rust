use crate::error::Result;
use crate::rac_wire::{
    decode_rpc_method, encode_rpc, OPCODE_CLOSE, OPCODE_INIT_ACK, OPCODE_RPC, OPCODE_SERVICE_ACK,
    OPCODE_SERVICE_NEGOTIATION,
};
use crate::Uuid16;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct SerializedRpc {
    pub payload: Vec<u8>,
    pub expect_method: Option<u8>,
}

pub trait ProtocolCodec: Send + Sync {
    fn name(&self) -> &'static str;
    fn protocol_version(&self) -> ProtocolVersion;

    fn init_packet(&self) -> &'static [u8];
    fn service_negotiation_payload(&self) -> &'static [u8];
    fn close_payload(&self) -> &'static [u8];

    fn opcode_init_ack(&self) -> u8;
    fn opcode_service_negotiation(&self) -> u8;
    fn opcode_service_ack(&self) -> u8;
    fn opcode_rpc(&self) -> u8;
    fn opcode_close(&self) -> u8;

    fn encode_rpc(&self, method_id: u8, body: &[u8]) -> Vec<u8>;
    fn decode_rpc_method_id(&self, payload: &[u8]) -> Option<u8>;

    fn serialize_set_cluster_context(&self, cluster: Uuid16) -> Result<SerializedRpc>;
    fn serialize_set_infobase_context(&self, cluster: Uuid16) -> Result<SerializedRpc>;
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    #[serde(rename = "v11.0")]
    V11_0,
    #[serde(rename = "v16.0")]
    V16_0,
}

impl ProtocolVersion {
    pub fn boxed(self) -> Box<dyn ProtocolCodec> {
        Box::new(RacProtocolImpl::new(self))
    }
}

#[derive(Debug)]
pub(crate) struct RacProtocolImpl {
    version: ProtocolVersion,
}

impl RacProtocolImpl {
    const INIT_PACKET: &'static [u8] = &[
        0x1c, 0x53, 0x57, 0x50, 0x01, 0x00, 0x01, 0x00, 0x01, 0x16, 0x01, 0x0f, 0x63, 0x6f,
        0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x04,
        0x00, 0x00, 0x07, 0xd0,
    ];

    const SERVICE_NEGOTIATION_V11: &'static [u8] = &[
        0x18, 0x76, 0x38, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x41, 0x64, 0x6d,
        0x69, 0x6e, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x04, 0x31, 0x31, 0x2e, 0x30,
        0x80,
    ];

    const SERVICE_NEGOTIATION_V16: &'static [u8] = &[
        0x18, 0x76, 0x38, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x41, 0x64, 0x6d,
        0x69, 0x6e, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x04, 0x31, 0x36, 0x2e, 0x30,
        0x80,
    ];

    const CLOSE: &'static [u8] = &[0x01];

    pub(crate) fn service_negotiation_payload(version: ProtocolVersion) -> &'static [u8] {
        match version {
            ProtocolVersion::V11_0 => Self::SERVICE_NEGOTIATION_V11,
            ProtocolVersion::V16_0 => Self::SERVICE_NEGOTIATION_V16,
        }
    }

    fn new(version: ProtocolVersion) -> Self {
        Self { version }
    }
}

impl ProtocolCodec for RacProtocolImpl {
    fn name(&self) -> &'static str {
        match self.version {
            ProtocolVersion::V11_0 => "v11.0",
            ProtocolVersion::V16_0 => "v16.0",
        }
    }

    fn protocol_version(&self) -> ProtocolVersion {
        self.version
    }

    fn init_packet(&self) -> &'static [u8] {
        Self::INIT_PACKET
    }

    fn service_negotiation_payload(&self) -> &'static [u8] {
        Self::service_negotiation_payload(self.version)
    }

    fn close_payload(&self) -> &'static [u8] {
        Self::CLOSE
    }

    fn opcode_init_ack(&self) -> u8 {
        OPCODE_INIT_ACK
    }

    fn opcode_service_negotiation(&self) -> u8 {
        OPCODE_SERVICE_NEGOTIATION
    }

    fn opcode_service_ack(&self) -> u8 {
        OPCODE_SERVICE_ACK
    }

    fn opcode_rpc(&self) -> u8 {
        OPCODE_RPC
    }

    fn opcode_close(&self) -> u8 {
        OPCODE_CLOSE
    }

    fn encode_rpc(&self, method_id: u8, body: &[u8]) -> Vec<u8> {
        encode_rpc(method_id, body)
    }

    fn decode_rpc_method_id(&self, payload: &[u8]) -> Option<u8> {
        decode_rpc_method(payload)
    }

    fn serialize_set_cluster_context(&self, cluster: Uuid16) -> Result<SerializedRpc> {
        let mut body = Vec::with_capacity(16 + 2);
        body.extend_from_slice(&cluster);
        body.extend_from_slice(&[0x00, 0x00]);
        Ok(SerializedRpc {
            payload: encode_rpc(crate::rac_wire::METHOD_CLUSTER_AUTH, &body),
            expect_method: None,
        })
    }

    fn serialize_set_infobase_context(&self, cluster: Uuid16) -> Result<SerializedRpc> {
        let mut body = Vec::with_capacity(16 + 2);
        body.extend_from_slice(&cluster);
        body.extend_from_slice(&[0x00, 0x00]);
        Ok(SerializedRpc {
            payload: encode_rpc(crate::rac_wire::METHOD_INFOBASE_CONTEXT, &body),
            expect_method: None,
        })
    }
}
