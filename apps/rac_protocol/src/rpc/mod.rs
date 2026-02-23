pub mod decode_utils;

use crate::error::Result;
use crate::protocol::{ProtocolCodec, SerializedRpc};
use crate::Uuid16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequiredContext {
    pub cluster: Option<Uuid16>,
    pub infobase_cluster: Option<Uuid16>,
}

impl Default for RequiredContext {
    fn default() -> Self {
        Self {
            cluster: None,
            infobase_cluster: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    pub method_req: u8,
    pub method_resp: Option<u8>,
    pub requires_cluster_context: bool,
    pub requires_infobase_context: bool,
}

impl Meta {
    pub fn required_context(self, cluster: Option<Uuid16>) -> RequiredContext {
        let infobase_cluster = if self.requires_infobase_context {
            cluster
        } else {
            None
        };
        RequiredContext {
            cluster: if self.requires_cluster_context { cluster } else { None },
            infobase_cluster,
        }
    }
}

pub trait Request {
    type Response: Response;

    fn meta(&self) -> Meta;
    fn cluster(&self) -> Option<Uuid16>;
    fn encode_body(&self, codec: &dyn ProtocolCodec) -> Result<Vec<u8>>;

    fn encode(&self, codec: &dyn ProtocolCodec) -> Result<SerializedRpc> {
        let meta = self.meta();
        let body = self.encode_body(codec)?;
        Ok(SerializedRpc {
            payload: codec.encode_rpc(meta.method_req, &body),
            expect_method: meta.method_resp,
        })
    }

    fn required_context(&self) -> RequiredContext {
        self.meta().required_context(self.cluster())
    }
}

pub trait Response: Sized {
    fn decode(body: &[u8], codec: &dyn ProtocolCodec) -> Result<Self>;
}
