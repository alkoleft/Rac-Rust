# RAC Agent Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11_help/agent_help.txt`
- `artifacts/rac/v11_help/agent_admin_list.out`
- `artifacts/rac/v11_help/agent_version.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

Aligned with current decoder implementation in `apps/rac_protocol/src/commands/agent_generated.rs`.

## Agent Admin List

Source capture:
- `artifacts/rac/v11_agent_admin_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11_agent_admin_list_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11_agent_admin_list_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11_agent_admin_list_ro_rac.out`
- `artifacts/rac/v11_help/agent_admin_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac agent admin list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 11.0 |
| `auth` | enum/string | no | 2 | 11.0 |
| `os-user` | string | no | 3 | 11.0 |
| `descr` | string | no | 4 | 11.0 |

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

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `name_len` | u8 | |
| `0x01` | `name_len` | `name` | string | UTF-8, observed `admin` |
| `0x01 + name_len` | `1` | `unknown_tag` | u8 | observed `0x00` |
| `0x02 + name_len` | `4` | `unknown_flags` | u32_be | observed raw bytes `03 ef bf bd` |
| `0x06 + name_len` | `3` | `unknown_tail` | bytes[3] | observed `01 00 00` |

Payload structure (response body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

## Agent Version

Sources:
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)
- `artifacts/rac/v11_help/agent_version.out` (v11 output)

Notes:
- No capture for this command yet; layout is aligned with decoder behavior.

### RPC

Request method: `0x87` (`agent version`)
Response method: `0x88`

Payload structure (method body):
- empty body (observed in decoder/test only)

### Поля ответа

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `version` | string | no | 1 | 11.0 |

### Record Layout (Decoder-Based)

Offsets are relative to the start of the response body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `version_len` | u8 | |
| `0x01` | `version_len` | `version` | string | UTF-8, e.g. `16.0.0.0` |

## Hypotheses

- `unknown_tag`, `unknown_flags`, and `unknown_tail` encode the `auth`/`os-user`/`descr` values shown by `rac`, but current capture has empty values so these fields collapse to short fixed markers.

## Open Questions

- Which bytes map to `auth`, `os-user`, and `descr` when those values are non-empty?
- Does the response include length-prefixed strings for `os-user`/`descr`, or are they omitted when empty?

## Gap Analysis

- Need captures where `auth` is changed (e.g., OS auth), and where `os-user`/`descr` are non-empty to identify string length markers.
- Capture multiple admin records to confirm whether any of `unknown_tag`, `unknown_flags`, or `unknown_tail` are per-record flags or response-level metadata.

## Agent Admin Register

Sources:
- `artifacts/rac/v11_help/agent_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac agent admin register` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `agent-user` | string | no | - | 11.0 |
| `agent-pwd` | string | no | - | 11.0 |
| `name` | string | no | - | 11.0 |
| `pwd` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `auth` | enum (`pwd`, `os`) | no | - | 11.0 |
| `os-user` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Agent Admin Remove

Sources:
- `artifacts/rac/v11_help/agent_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac agent admin remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `agent-user` | string | no | - | 11.0 |
| `agent-pwd` | string | no | - | 11.0 |
| `name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).
