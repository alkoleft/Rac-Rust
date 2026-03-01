# Agent message analysis (captures)

Scope: RAC Agent mode using v16 captures with `--agent-user=admin --agent-pwd=pass`.

## Evidence
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_server_to_client.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_register_pwd_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_register_pwd_os_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_remove_pwd_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_version_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_version_server_to_client.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_response.hex`

## Confirmed facts
- Service negotiation opcode `0x0b` with payload `v8.service.Admin.Cluster` + version `16.0` occurs before each command.
- `agent auth` RPC method id is `0x08` and payload is `str8 agent-user` + `str8 agent-pwd`.
  - Example payload (auth): `0100000108 05 61646d696e 04 70617373` ("admin"/"pass").
- `agent admin list` RPC method id `0x00` request has empty body and response method id `0x01`.
- `agent version` request method id `0x87`, response method id `0x88`.
- `agent admin register` request method id `0x04` (ACK-only response).
- `agent admin remove` request method id `0x06` (ACK-only response).

## Record layout (AgentAdminRecord, v16.0)
Offsets from record start, derived from server response capture.

| Offset | Size | Field | Type | Notes |
| --- | --- | --- | --- | --- |
| 0x00 | 1 | name_len | u8 | |
| 0x01 | name_len | name | str8 | UTF-8 |
| 0x01+name_len | 1 | descr_len | u8 | |
| 0x02+name_len | descr_len | descr | str8 | UTF-8 |
| 0x02+name_len+descr_len | 4 | unknown_flags | u32_be | observed `03 ef bf bd` |
| 0x06+name_len+descr_len | 1 | auth_tag | u8 | observed `0x01` |
| 0x07+name_len+descr_len | 1 | auth_flags | u8 | `0x00` (pwd) / `0x01` (pwd|os) |
| 0x08+name_len+descr_len | 1 | os_user_len | u8 | |
| 0x09+name_len+descr_len | os_user_len | os_user | str8 | UTF-8 |

Payload structure for response body:
- `count:u8` followed by `count` records.

## Request layouts (v16.0)

### AgentAuth (0x08)
- `str8 agent-user`
- `str8 agent-pwd`

### AgentAdminList (0x00)
- empty body

### AgentAdminRegister (0x04)
Offsets from method body start:
- `name_len:u8`, `name[name_len]`
- `descr_len:u8`, `descr[descr_len]`
- `pwd_len:u8`, `pwd[pwd_len]`
- `auth_tag:u8` (observed `0x01`)
- `auth_flags:u8` (`0x00` pwd / `0x01` pwd|os)
- `os_user_len:u8`, `os_user[os_user_len]`

### AgentAdminRemove (0x06)
- `name_len:u8`, `name[name_len]`

### AgentVersion (0x87)
- empty body

## Hypotheses
- `unknown_flags` (`0x03efbfbd`) is a fixed marker for admin records in v16.0.
- `auth_tag` is a fixed marker (`0x01`) for auth fields in v16.0.

## Open questions
- Does `unknown_flags` vary across versions or auth types?
- Is `auth_tag` always `0x01` across v11/v16?
- Does `auth_flags` encode additional modes beyond pwd / pwd|os?

## Gap analysis (captures needed)
- Capture `agent admin list` with `--auth=os` only and with non-empty `os-user`/`descr` to confirm `auth_flags` encoding.
- Capture in v11 to confirm `unknown_flags` and `auth_tag` stability.
