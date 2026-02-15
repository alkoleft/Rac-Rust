use serde::Serialize;

use crate::client::RacClient;
use crate::error::{RacError, Result};
use crate::rac_wire::{
    decode_rpc_method, encode_agent_version, encode_cluster_scoped, encode_cluster_scoped_object,
    encode_rpc, scan_len_prefixed_strings, take_str8, take_u16_be, take_u32_be, take_u64_be,
    take_uuid16, METHOD_AGENT_VERSION_RESP,
    METHOD_CLUSTER_INFO_REQ, METHOD_CLUSTER_INFO_RESP, METHOD_CLUSTER_LIST_REQ, METHOD_CLUSTER_LIST_RESP,
    METHOD_CONNECTION_INFO_REQ, METHOD_CONNECTION_INFO_RESP, METHOD_CONNECTION_LIST_REQ,
    METHOD_CONNECTION_LIST_RESP, METHOD_COUNTER_LIST_REQ, METHOD_COUNTER_LIST_RESP,
    METHOD_LIMIT_LIST_REQ, METHOD_LIMIT_LIST_RESP, METHOD_LOCK_LIST_REQ, METHOD_LOCK_LIST_RESP,
    METHOD_MANAGER_INFO_REQ, METHOD_MANAGER_INFO_RESP, METHOD_MANAGER_LIST_REQ, METHOD_MANAGER_LIST_RESP,
    METHOD_PROCESS_INFO_REQ, METHOD_PROCESS_INFO_RESP, METHOD_PROCESS_LIST_REQ, METHOD_PROCESS_LIST_RESP,
    METHOD_PROFILE_LIST_REQ, METHOD_PROFILE_LIST_RESP, METHOD_SERVER_INFO_REQ, METHOD_SERVER_INFO_RESP,
    METHOD_SERVER_LIST_REQ, METHOD_SERVER_LIST_RESP, METHOD_SESSION_INFO_REQ, METHOD_SESSION_INFO_RESP,
    METHOD_SESSION_LIST_REQ, METHOD_SESSION_LIST_RESP,
};
use crate::rac_wire::uuid_from_slice;
use crate::Uuid16;

pub mod infobase;

#[derive(Debug, Serialize)]
pub struct AgentVersionResp {
    pub version: Option<String>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ClusterSummary {
    pub uuid: Uuid16,
    pub host: Option<String>,
    pub display_name: Option<String>,
    pub port: Option<u16>,
    pub expiration_timeout: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ClusterListResp {
    pub clusters: Vec<ClusterSummary>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ClusterInfoResp {
    pub cluster: ClusterSummary,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ManagerListResp {
    pub managers: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ManagerInfoResp {
    pub manager: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ServerListResp {
    pub servers: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ServerInfoResp {
    pub server: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ProcessListResp {
    pub processes: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ProcessInfoResp {
    pub process: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub use self::infobase::{
    infobase_info, infobase_summary_info, infobase_summary_list, InfobaseInfoResp, InfobaseSummary,
    InfobaseSummaryInfoResp, InfobaseSummaryListResp,
};

#[derive(Debug, Serialize)]
pub struct ConnectionListResp {
    pub connections: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionInfoResp {
    pub connection: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct SessionListResp {
    pub sessions: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct SessionInfoResp {
    pub session: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct LockListResp {
    pub locks: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ProfileListResp {
    pub profiles: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct CounterListResp {
    pub counters: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct LimitListResp {
    pub limits: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn agent_version(client: &mut RacClient) -> Result<AgentVersionResp> {
    let reply = client.send_rpc(&encode_agent_version(), Some(METHOD_AGENT_VERSION_RESP))?;
    let body = rpc_body(&reply)?;
    let strings = scan_len_prefixed_strings(body);
    let version = strings.first().map(|(_, s)| s.clone());
    Ok(AgentVersionResp {
        version,
        raw_payload: Some(reply),
    })
}

pub fn cluster_list(client: &mut RacClient) -> Result<ClusterListResp> {
    let reply = client.send_rpc(&encode_rpc(METHOD_CLUSTER_LIST_REQ, &[]), Some(METHOD_CLUSTER_LIST_RESP))?;
    let body = rpc_body(&reply)?;
    let mut clusters = parse_cluster_list_body(body).unwrap_or_default();
    if clusters.is_empty() {
        let uuids = scan_uuid_bytes(body)?;
        let strings = scan_len_prefixed_strings(body);
        for (idx, uuid) in uuids.into_iter().enumerate() {
            let mut summary = ClusterSummary {
                uuid,
                host: None,
                display_name: None,
                port: None,
                expiration_timeout: None,
            };
            if idx == 0 && strings.len() >= 2 {
                summary.host = Some(strings[0].1.clone());
                summary.display_name = Some(strings[1].1.clone());
            }
            clusters.push(summary);
        }
    }
    Ok(ClusterListResp {
        clusters,
        raw_payload: Some(reply),
    })
}

pub fn cluster_info(client: &mut RacClient, cluster: Uuid16) -> Result<ClusterInfoResp> {
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_CLUSTER_INFO_REQ, cluster),
        Some(METHOD_CLUSTER_INFO_RESP),
    )?;
    let body = rpc_body(&reply)?;
    let summary = if let Some(summary) = parse_cluster_info_body(body) {
        summary
    } else {
        let uuids = scan_uuid_bytes(body)?;
        let strings = scan_len_prefixed_strings(body);
        let uuid = *uuids.first().ok_or(RacError::Decode("missing cluster uuid"))?;
        ClusterSummary {
            uuid,
            host: strings.get(0).map(|(_, s)| s.clone()),
            display_name: strings.get(1).map(|(_, s)| s.clone()),
            port: None,
            expiration_timeout: None,
        }
    };
    Ok(ClusterInfoResp {
        cluster: summary,
        raw_payload: Some(reply),
    })
}

pub fn manager_list(client: &mut RacClient, cluster: Uuid16) -> Result<ManagerListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_MANAGER_LIST_REQ, cluster),
        Some(METHOD_MANAGER_LIST_RESP),
    )?;
    Ok(ManagerListResp {
        managers: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn manager_info(client: &mut RacClient, cluster: Uuid16, manager: Uuid16) -> Result<ManagerInfoResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped_object(METHOD_MANAGER_INFO_REQ, cluster, manager),
        Some(METHOD_MANAGER_INFO_RESP),
    )?;
    let body = rpc_body(&reply)?;
    Ok(ManagerInfoResp {
        manager: first_uuid(body)?,
        fields: scan_len_prefixed_strings(body).into_iter().map(|(_, s)| s).collect(),
        raw_payload: Some(reply),
    })
}

pub fn server_list(client: &mut RacClient, cluster: Uuid16) -> Result<ServerListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_SERVER_LIST_REQ, cluster),
        Some(METHOD_SERVER_LIST_RESP),
    )?;
    Ok(ServerListResp {
        servers: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn server_info(client: &mut RacClient, cluster: Uuid16, server: Uuid16) -> Result<ServerInfoResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped_object(METHOD_SERVER_INFO_REQ, cluster, server),
        Some(METHOD_SERVER_INFO_RESP),
    )?;
    let body = rpc_body(&reply)?;
    Ok(ServerInfoResp {
        server: first_uuid(body)?,
        fields: scan_len_prefixed_strings(body).into_iter().map(|(_, s)| s).collect(),
        raw_payload: Some(reply),
    })
}

pub fn process_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProcessListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_PROCESS_LIST_REQ, cluster),
        Some(METHOD_PROCESS_LIST_RESP),
    )?;
    Ok(ProcessListResp {
        processes: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn process_info(client: &mut RacClient, cluster: Uuid16, process: Uuid16) -> Result<ProcessInfoResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped_object(METHOD_PROCESS_INFO_REQ, cluster, process),
        Some(METHOD_PROCESS_INFO_RESP),
    )?;
    let body = rpc_body(&reply)?;
    Ok(ProcessInfoResp {
        process: first_uuid(body)?,
        fields: scan_len_prefixed_strings(body).into_iter().map(|(_, s)| s).collect(),
        raw_payload: Some(reply),
    })
}

pub fn connection_list(client: &mut RacClient, cluster: Uuid16) -> Result<ConnectionListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_CONNECTION_LIST_REQ, cluster),
        Some(METHOD_CONNECTION_LIST_RESP),
    )?;
    Ok(ConnectionListResp {
        connections: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn connection_info(
    client: &mut RacClient,
    cluster: Uuid16,
    connection: Uuid16,
) -> Result<ConnectionInfoResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped_object(METHOD_CONNECTION_INFO_REQ, cluster, connection),
        Some(METHOD_CONNECTION_INFO_RESP),
    )?;
    let body = rpc_body(&reply)?;
    Ok(ConnectionInfoResp {
        connection: first_uuid(body)?,
        fields: scan_len_prefixed_strings(body).into_iter().map(|(_, s)| s).collect(),
        raw_payload: Some(reply),
    })
}

pub fn session_list(client: &mut RacClient, cluster: Uuid16) -> Result<SessionListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_SESSION_LIST_REQ, cluster),
        Some(METHOD_SESSION_LIST_RESP),
    )?;
    Ok(SessionListResp {
        sessions: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn session_info(client: &mut RacClient, cluster: Uuid16, session: Uuid16) -> Result<SessionInfoResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped_object(METHOD_SESSION_INFO_REQ, cluster, session),
        Some(METHOD_SESSION_INFO_RESP),
    )?;
    let body = rpc_body(&reply)?;
    Ok(SessionInfoResp {
        session: first_uuid(body)?,
        fields: scan_len_prefixed_strings(body).into_iter().map(|(_, s)| s).collect(),
        raw_payload: Some(reply),
    })
}

pub fn lock_list(client: &mut RacClient, cluster: Uuid16) -> Result<LockListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_LOCK_LIST_REQ, cluster),
        Some(METHOD_LOCK_LIST_RESP),
    )?;
    Ok(LockListResp {
        locks: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn profile_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProfileListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_PROFILE_LIST_REQ, cluster),
        Some(METHOD_PROFILE_LIST_RESP),
    )?;
    Ok(ProfileListResp {
        profiles: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn counter_list(client: &mut RacClient, cluster: Uuid16) -> Result<CounterListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_COUNTER_LIST_REQ, cluster),
        Some(METHOD_COUNTER_LIST_RESP),
    )?;
    Ok(CounterListResp {
        counters: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn limit_list(client: &mut RacClient, cluster: Uuid16) -> Result<LimitListResp> {
    client.set_cluster_context(cluster)?;
    let reply = client.send_rpc(
        &encode_cluster_scoped(METHOD_LIMIT_LIST_REQ, cluster),
        Some(METHOD_LIMIT_LIST_RESP),
    )?;
    Ok(LimitListResp {
        limits: scan_uuid_bytes(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub(crate) fn rpc_body(payload: &[u8]) -> Result<&[u8]> {
    if payload.len() >= 5 && payload[0..4] == [0x01, 0x00, 0x00, 0x01] {
        return Ok(&payload[5..]);
    }
    if decode_rpc_method(payload).is_none() {
        return Err(RacError::Decode("missing rpc header"));
    }
    Err(RacError::Decode("unexpected rpc header"))
}

fn scan_uuid_bytes(data: &[u8]) -> Result<Vec<Uuid16>> {
    let mut out = Vec::new();
    for i in 0..data.len() {
        let marker = data[i];
        if marker != 0x16 && marker != 0x19 {
            continue;
        }
        let start = i + 1;
        let end = start + 16;
        if end <= data.len() {
            let uuid = uuid_from_slice(&data[start..end])?;
            out.push(uuid);
        }
    }
    if !out.is_empty() {
        return Ok(out);
    }
    if data.len() >= 16 {
        let count = data[0] as usize;
        if 1 + count * 16 <= data.len() {
            let mut off = 1;
            for _ in 0..count {
                let uuid = uuid_from_slice(&data[off..off + 16])?;
                out.push(uuid);
                off += 16;
            }
            return Ok(out);
        }
        out.push(uuid_from_slice(&data[0..16])?);
    }
    Ok(out)
}

const CLUSTER_TAIL_SIZE: usize = 32;

fn parse_cluster_list_body(body: &[u8]) -> Option<Vec<ClusterSummary>> {
    if body.is_empty() {
        return Some(Vec::new());
    }
    if body[0] == 0x01 {
        if let Ok((clusters, _)) = parse_cluster_records_at(body, 1, Some(body[0] as usize)) {
            return Some(clusters);
        }
    }
    parse_cluster_records_at(body, 0, None).ok().map(|(clusters, _)| clusters)
}

fn parse_cluster_info_body(body: &[u8]) -> Option<ClusterSummary> {
    if body.is_empty() {
        return None;
    }
    if body[0] == 0x01 {
        if let Ok((clusters, _)) = parse_cluster_records_at(body, 1, Some(1)) {
            return clusters.into_iter().next();
        }
    }
    parse_cluster_records_at(body, 0, Some(1))
        .ok()
        .and_then(|(clusters, _)| clusters.into_iter().next())
}

fn parse_cluster_records_at(
    data: &[u8],
    offset: usize,
    count: Option<usize>,
) -> Result<(Vec<ClusterSummary>, usize)> {
    let mut out = Vec::new();
    let mut off = offset;
    let mut remaining = count.unwrap_or(1);
    while remaining > 0 {
        let (summary, next_off) = parse_cluster_record(data, off)?;
        out.push(summary);
        off = next_off;
        if count.is_some() {
            remaining -= 1;
        } else {
            break;
        }
        if off >= data.len() {
            break;
        }
    }
    Ok((out, off))
}

fn parse_cluster_record(data: &[u8], offset: usize) -> Result<(ClusterSummary, usize)> {
    let (uuid, mut off) = take_uuid16(data, offset)?;
    let (expiration_timeout, off2) = take_u32_be(data, off)?;
    off = off2;
    let (host, off2) = take_str8(data, off)?;
    off = off2;
    let (_unknown_u32, off2) = take_u32_be(data, off)?;
    off = off2;
    let (port, off2) = take_u16_be(data, off)?;
    off = off2;
    let (_unknown_u64, off2) = take_u64_be(data, off)?;
    off = off2;
    let (display_name, off2) = take_str8(data, off)?;
    off = off2;
    if off + CLUSTER_TAIL_SIZE <= data.len() {
        off += CLUSTER_TAIL_SIZE;
    }
    Ok((
        ClusterSummary {
            uuid,
            host: Some(host),
            display_name: Some(display_name),
            port: Some(port),
            expiration_timeout: Some(expiration_timeout),
        },
        off,
    ))
}

pub(crate) fn first_uuid(data: &[u8]) -> Result<Uuid16> {
    let list = scan_uuid_bytes(data)?;
    list.first().copied().ok_or(RacError::Decode("missing uuid"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_uuid(byte: u8) -> Uuid16 {
        [byte; 16]
    }

    fn rpc_with_body(method: u8, body: &[u8]) -> Vec<u8> {
        let mut out = vec![0x01, 0x00, 0x00, 0x01, method];
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn parse_uuid_list_from_body() {
        let mut body = Vec::new();
        body.push(0x16);
        body.extend_from_slice(&make_uuid(0x11));
        body.push(0x16);
        body.extend_from_slice(&make_uuid(0x22));
        let list = scan_uuid_bytes(&body).expect("uuid scan");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], make_uuid(0x11));
        assert_eq!(list[1], make_uuid(0x22));
    }

    #[test]
    fn parse_agent_version() {
        let mut body = Vec::new();
        body.push(0x05);
        body.extend_from_slice(b"1.2.3");
        let payload = rpc_with_body(METHOD_AGENT_VERSION_RESP, &body);
        let strings = scan_len_prefixed_strings(rpc_body(&payload).unwrap());
        assert_eq!(strings[0].1, "1.2.3");
    }
}
