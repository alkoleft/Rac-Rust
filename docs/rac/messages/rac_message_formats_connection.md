# RAC Connection List Record Format (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11/help/connection_help.txt`
- `artifacts/rac/v11/help/connection_list.out`
- `artifacts/rac/v11/help/connection_info.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

Source capture:
- `artifacts/rac/v11/v11_connection_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_connection_list_ro_server_to_client.decode.txt`

RAC output reference:
- `artifacts/rac/v11/v11_connection_list_ro_rac.out`

## Поля ответа (из `rac`)

Observed field names in `rac connection list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `connection` | UUID | yes | 1 | 11.0 |
| `application` | string | yes | 2 | 11.0 |
| `connected-at` | datetime (u64 ticks, 100us since 0001-01-01) | yes | 3 (inside `blocked_by_ls + connected_at`) | 11.0 |
| `conn-id` | u32 | yes | 3 | 11.0 |
| `host` | string | yes | 4 | 11.0 |
| `infobase` | UUID (nullable) | yes | 5 | 11.0 |
| `process` | UUID | yes | 6 | 11.0 |
| `session-number` | u32 | yes | 7 (tail) | 11.0 |
| `blocked-by-ls` | u32 | hypothesis (maps to `blocked_by_ls`) | 3 (inside `blocked_by_ls`) | 11.0 |

## RPC

Request method: `0x32` (after context `0x09`, `connection list --cluster <id>`)

Response method: `0x33` (`connection list`)

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x06`)
- offset `0x01`: first record starts here
- subsequent records start at the next connection UUID occurrence (variable length records)

## Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `connection` | UUID | |
| `0x10` | `1` | `app_len` | u8 | |
| `0x11` | `app_len` | `application` | string | UTF-8, observed `RAS`, `AgentStandardCall`, `1CV8C`, `SystemBackgroundJob` |
| `0x11 + app_len` | `4` | `blocked_by_ls` | u32_be | hypothesis, observed `0x00000000` |
| `0x15 + app_len` | `8` | `connected_at` | datetime (u64_be) | 100 microsecond ticks since `0001-01-01 00:00:00` |
| `0x1d + app_len` | `4` | `conn_id` | u32_be | example: `0x0000092b` => `2347` |
| `0x21 + app_len` | `1` | `host_len` | u8 | |
| `0x22 + app_len` | `host_len` | `host` | string | UTF-8, observed `alko-home` |
| `0x22 + app_len + host_len` | `16` | `infobase` | UUID (nullable) | can be all zeros when absent |
| `0x32 + app_len + host_len` | `16` | `process` | UUID | |
| `0x42 + app_len + host_len` | `4` | `session_number` | u32_be | observed `0x00000000`, `0x00000005` |

Notes:
- Gaps: none observed in this record layout.

## Record 1 Example (Offsets)

From the first record in the capture (connection `c030e65d-680a-41ed-a15a-6b859025f0b7`):

- `0x00..0x0f` connection UUID
- `0x10` app_len = `0x03`
- `0x11..0x13` app = `RAS`
- `0x14..0x17` blocked_by_ls (hypothesis) = `00 00 00 00`
- `0x18..0x1f` connected_at_ticks = `00 02 45 3a 9e af 75 60`
- `0x20..0x23` conn_id = `00 00 09 2b`
- `0x24` host_len = `0x09`
- `0x25..0x2d` host = `alko-home`
- `0x2e..0x3d` infobase UUID = `717bdda7-2f60-4577-b262-f1fc8c0e472c`
- `0x3e..0x4d` process UUID = `f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5`
- `0x4e..0x51` tail_u32 = `00 00 00 00`

## Hypotheses

- `blocked_by_ls` maps to `blocked-by-ls` in `rac` output (needs non-zero capture).

## Open Questions

- Confirm `blocked_by_ls` by finding a non-zero value in a capture.

## Поля запроса (из `rac`)

Observed request parameters for `rac connection list` (v11 help).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `process` | UUID | no | - | 11.0 |
| `infobase` | UUID | no | - | 11.0 |
| `infobase-user` | string | no | - | 11.0 |
| `infobase-pwd` | string | no | - | 11.0 |

## Connection Info

Source (v11 output):
- `artifacts/rac/v11/help/connection_info.out`

### RPC

Request method: `0x36` (after context `0x09`, `connection info --cluster <id> --connection <id>`)
Response method: `0x37`

### Поля ответа (из `rac`)

Same field set as `connection list` (see above).

### Поля запроса (из `rac`)

Observed request parameters for `rac connection info` (v11 help).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `connection` | UUID | no | - | 11.0 |

## Connection Disconnect

Sources:
- `artifacts/rac/v11/help/connection_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac connection disconnect` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `process` | UUID | no | - | 11.0 |
| `connection` | UUID | no | - | 11.0 |
| `infobase-user` | string | no | - | 11.0 |
| `infobase-pwd` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).
