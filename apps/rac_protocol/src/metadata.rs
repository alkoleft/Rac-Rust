use crate::client::RacRequest;
use crate::commands::agent::rpc_metadata as agent_rpc_metadata;
use crate::commands::cluster::rpc_metadata as cluster_rpc_metadata;

#[derive(Debug, Clone, Copy)]
pub struct RpcMethodMeta {
    pub method_req: u8,
    pub method_resp: Option<u8>,
    pub requires_cluster_context: bool,
    pub requires_infobase_context: bool,
}

pub fn cluster_rpc_meta(request: &RacRequest) -> Option<RpcMethodMeta> {
    cluster_rpc_metadata(request)
}

pub fn agent_rpc_meta(request: &RacRequest) -> Option<RpcMethodMeta> {
    agent_rpc_metadata(request)
}

pub fn rpc_meta(request: &RacRequest) -> Option<RpcMethodMeta> {
    agent_rpc_metadata(request).or_else(|| cluster_rpc_metadata(request))
}
