use crate::rpc::{Meta, Request};

pub type RpcMethodMeta = Meta;

pub fn rpc_meta<R: Request>(request: &R) -> Meta {
    request.meta()
}
