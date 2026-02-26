# RAC Agent Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v16):
- `artifacts/rac/v16/help/agent_help.txt`
- `artifacts/rac/v16/help/agent_admin_list.out`
- `artifacts/rac/v16/help/agent_version.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

Aligned with current decoder implementation in `apps/rac_protocol/src/commands/agent_generated.rs`.

## Agent Admin List

Source capture:
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_response.hex`

RAC output reference:
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_list_rac.out`
- `artifacts/rac/v16/help/agent_admin_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac agent admin list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 11.0 |
| `descr` | string | yes | 2 | 11.0 |
| `auth` | enum/flags | yes | 3 | 11.0 |
| `os-user` | string | yes | 4 | 11.0 |

### RPC

Request method: `0x00` (`agent admin list`)
Response method: `0x01`

### Поля запроса (из `rac`)

Observed request parameters for `rac agent admin list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `agent-user` | string | yes (auth call `0x08`) | 1 | 11.0 |
| `agent-pwd` | string | yes (auth call `0x08`) | 2 | 11.0 |

Payload structure (method body):
- empty body (observed only RPC header)

Auth RPC (`agent auth`):
- request method: `0x08`
- response: ACK (`01 00 00 00`)
- payload: `str8 agent-user` + `str8 agent-pwd`

### Record Layout (Observed, v16.0)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `name_len` | u8 | |
| `0x01` | `name_len` | `name` | string | UTF-8, e.g. `admin` |
| `0x01 + name_len` | `1` | `descr_len` | u8 | |
| `0x02 + name_len` | `descr_len` | `descr` | string | UTF-8 (empty for `admin`) |
| `0x02 + name_len + descr_len` | `4` | `unknown_flags` | u32_be | observed raw bytes `03 ef bf bd` |
| `0x06 + name_len + descr_len` | `1` | `auth_tag` | u8 | observed `0x01` |
| `0x07 + name_len + descr_len` | `1` | `auth_flags` | u8 | observed `0x00` (pwd) / `0x01` (pwd|os) |
| `0x08 + name_len + descr_len` | `1` | `os_user_len` | u8 | |
| `0x09 + name_len + descr_len` | `os_user_len` | `os_user` | string | UTF-8, empty when `os_user_len=0` |

Payload structure (response body):
- offset `0x00`: `count:u8` (observed `0x03`)
- offset `0x01`: first record starts here

## Agent Version

Sources:
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)
- `artifacts/rac/v16/v16_20260226_053425_agent_version_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_version_server_to_client.decode.txt`
- `artifacts/rac/v16/help/agent_version.out` (v11 output)

Notes:
- Capture confirms response body layout; request body remains empty.

### RPC

Request method: `0x87` (`agent version`)
Response method: `0x88`

Payload structure (method body):
- empty body (observed in capture)

### Поля ответа

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `version` | string | yes | 1 | 11.0 |

### Record Layout (Observed, v16.0)

Offsets are relative to the start of the response body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `version_len` | u8 | |
| `0x01` | `version_len` | `version` | string | UTF-8, observed `8.5.1.1150` |

## Hypotheses

- `unknown_flags` (`0x03efbfbd`) and `auth_tag` (`0x01`) are fixed markers for agent admin records in v16.0.

## Open Questions

- What is the semantic meaning of `unknown_flags` (`0x03efbfbd`), and does it vary across versions?
- Is `auth_tag` always `0x01` or does it carry additional meaning in other captures?

## Gap Analysis

- Need captures with different `auth` combinations (e.g., `os` only) to validate `auth_flags` encoding.

## Agent Admin Register

Sources:
- `artifacts/rac/v16/help/agent_help.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_register_pwd_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_register_pwd_os_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_register_pwd_server_to_client.decode.txt`

### RPC

Request method: `0x04` (`agent admin register`)
Response: ACK (`01 00 00 00`)

### Поля запроса (из `rac`)

Observed request parameters for `rac agent admin register` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `agent-user` | string | yes (auth call `0x08`) | 1 | 11.0 |
| `agent-pwd` | string | yes (auth call `0x08`) | 2 | 11.0 |
| `name` | string | yes | 1 | 11.0 |
| `descr` | string | yes | 2 | 11.0 |
| `pwd` | string | yes | 3 | 11.0 |
| `auth` | flags | yes | 4 | 11.0 |
| `os-user` | string | yes | 5 | 11.0 |

Payload structure (method body):
- offset `0x00`: `name_len:u8`
- offset `0x01`: `name[name_len]`
- offset `0x01 + name_len`: `descr_len:u8`
- offset `0x02 + name_len`: `descr[descr_len]`
- offset `0x02 + name_len + descr_len`: `pwd_len:u8`
- offset `0x03 + name_len + descr_len`: `pwd[pwd_len]`
- offset `0x03 + name_len + descr_len + pwd_len`: `auth_tag:u8` (observed `0x01`)
- offset `0x04 + name_len + descr_len + pwd_len`: `auth_flags:u8` (`0x00` pwd, `0x01` pwd|os)
- offset `0x05 + name_len + descr_len + pwd_len`: `os_user_len:u8`
- offset `0x06 + name_len + descr_len + pwd_len`: `os_user[os_user_len]`

### Поля ответа

ACK-only (empty body).

## Agent Admin Remove

Sources:
- `artifacts/rac/v16/help/agent_help.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_remove_pwd_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_remove_pwd_os_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_agent_admin_remove_pwd_server_to_client.decode.txt`

### RPC

Request method: `0x06` (`agent admin remove`)
Response: ACK (`01 00 00 00`)

### Поля запроса (из `rac`)

Observed request parameters for `rac agent admin remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `agent-user` | string | yes (auth call `0x08`) | 1 | 11.0 |
| `agent-pwd` | string | yes (auth call `0x08`) | 2 | 11.0 |
| `name` | string | yes | 1 | 11.0 |

Payload structure (method body):
- offset `0x00`: `name_len:u8`
- offset `0x01`: `name[name_len]`

### Поля ответа

ACK-only (empty body).
