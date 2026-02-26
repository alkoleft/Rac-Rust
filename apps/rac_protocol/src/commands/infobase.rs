use crate::client::RacClient;
use crate::error::Result;
use crate::Uuid16;

mod generated {
    include!("infobase_generated.rs");
}

pub use generated::{
    InfobaseInfoRecord,
    InfobaseInfoResp,
    InfobaseInfoRpc,
    InfobaseSummary,
    InfobaseSummaryInfoResp,
    InfobaseSummaryInfoRpc,
    InfobaseSummaryListResp,
    InfobaseSummaryListRpc,
    InfobaseSummaryUpdateRpc,
};

pub fn infobase_summary_list(
    client: &mut RacClient,
    cluster: Uuid16,
) -> Result<InfobaseSummaryListResp> {
    client.call_typed(InfobaseSummaryListRpc { cluster })
}

pub fn infobase_summary_info(
    client: &mut RacClient,
    cluster: Uuid16,
    infobase: Uuid16,
) -> Result<InfobaseSummaryInfoResp> {
    client.call_typed(InfobaseSummaryInfoRpc { cluster, infobase })
}

pub fn infobase_info(
    client: &mut RacClient,
    cluster: Uuid16,
    infobase: Uuid16,
) -> Result<InfobaseInfoResp> {
    client.call_typed(InfobaseInfoRpc { cluster, infobase })
}
