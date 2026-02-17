# RAC Agent Message Formats (Observed)

## Agent Admin List

Source capture:
- `logs/session_1771343284_3463051_127_0_0_1_40136/client_to_server.stream.bin`
- `logs/session_1771343284_3463051_127_0_0_1_40136/server_to_client.stream.bin`

Payload example:
- `artifacts/agent_admin_list_response.hex`

RAC output reference:
- `artifacts/agent_admin_list_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac agent admin list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `name` | string | yes | 1 |
| `auth` | enum/string | no | 2 |
| `os-user` | string | no | 3 |
| `descr` | string | no | 4 |

### RPC

Request method: `0x00` (`agent admin list`)
Response method: `0x01`

### Поля запроса (из `rac`)

Observed request parameters for `rac agent admin list`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `agent-user` | string | yes (in auth/context `0x08`) | 1 |
| `agent-pwd` | string | yes (in auth/context `0x08`) | 2 |

Payload structure (method body):
- empty body (observed only rpc header)

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `name_len` | u8 | |
| `0x01` | `name_len` | `name` | string | UTF-8, observed `admin` |
| `0x01 + name_len` | `1` | `auth_tag` | u8 | observed `0x00` |
| `0x02 + name_len` | `4` | `auth_flags_or_hash` | u32_be? | observed raw bytes `03 ef bf bd` |
| `0x06 + name_len` | `3` | `tail_flags` | bytes[3] | observed `01 00 00` |

Payload structure (response body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

## Hypotheses

- `auth_tag` + `auth_flags_or_hash` + `tail_flags` encode `auth`/`os-user`/`descr` fields, but current capture has empty values so these fields collapse to short fixed markers.

## Open Questions

- Which bytes map to `auth`, `os-user`, and `descr` when those values are non-empty?
- Does the response include length-prefixed strings for `os-user`/`descr`, or are they omitted when empty?

## Gap Analysis

- Need captures where `auth` is changed (e.g., OS auth), and where `os-user`/`descr` are non-empty to identify string length markers.
- Capture multiple admin records to confirm whether any of `auth_tag`, `auth_flags_or_hash`, or `tail_flags` are per-record flags or response-level metadata.
