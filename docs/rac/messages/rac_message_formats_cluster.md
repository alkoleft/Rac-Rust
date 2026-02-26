# RAC Cluster Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v16):
- `artifacts/rac/v16/help/cluster_help.txt`
- `artifacts/rac/v16/help/cluster_list.out`
- `artifacts/rac/v16/help/cluster_info.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Cluster List

Source capture:
- `artifacts/rac/v16/v16_20260226_053425_cluster_list_after_update_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_cluster_list_after_update_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v16/v16_20260226_053425_cluster_list_after_update_response.hex`

RAC output reference:
- `artifacts/rac/v16/v16_20260226_053425_cluster_list_after_update_rac.out`
- `artifacts/rac/v16/help/cluster_list.out`

## Fields From `rac` Output

Observed field names in `rac cluster list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |
| `host` | string | yes | 3 | 11.0 |
| `port` | u16 | yes | 5 | 11.0 |
| `name` | string | yes | 8 | 11.0 |
| `expiration-timeout` | u32 | yes | 2 | 11.0 |
| `lifetime-limit` | u32 | yes | 4 | 11.0 |
| `max-memory-size` | u32 | yes | 6 | 11.0 |
| `max-memory-time-limit` | u32 | yes | 7 | 11.0 |
| `security-level` | u32 | yes | 9 | 11.0 |
| `session-fault-tolerance-level` | u32 | yes | 10 | 11.0 |
| `load-balancing-mode` | u32 | yes | 11 | 11.0 |
| `errors-count-threshold` | u32 | yes | 12 | 11.0 |
| `kill-problem-processes` | u8 | yes | 13 | 11.0 |
| `kill-by-memory-with-dump` | u8 | yes | 14 | 11.0 |
| `allow-access-right-audit-events-recording` | unknown | no | 15 | 11.0 |
| `ping-period` | unknown | no | 16 | 16.0 |
| `ping-timeout` | unknown | no | 17 | 16.0 |
| `restart-schedule` | unknown | no | 18 | 11.0 |

## RPC Envelope

Request method: `0x0b` (`cluster list`)
Response method: `0x0c`

## Fields From `rac` Request

Observed request parameters for `rac cluster list`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| *(none)* | - | n/a | n/a | 11.0 |

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here
- subsequent records start at the next cluster UUID occurrence

## Record Layout (Observed, v11.0)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `cluster` | UUID | |
| `0x10` | `4` | `expiration-timeout` | u32_be | observed `0x0000003c` -> `60` |
| `0x14` | `1` | `host-len` | u8 | |
| `0x15` | `host-len` | `host` | string | UTF-8, observed `alko-home` |
| `0x15 + host-len` | `4` | `lifetime-limit` | u32_be | observed `0x00000457` -> `1111` |
| `0x19 + host-len` | `2` | `port` | u16_be | observed `0x0605` -> `1541` |
| `0x1b + host-len` | `4` | `max-memory-size` | u32_be | hypothesis (remained zero in captures) |
| `0x1f + host-len` | `4` | `max-memory-time-limit` | u32_be | hypothesis (remained zero in captures) |
| `0x23 + host-len` | `1` | `name-len` | u8 | |
| `0x24 + host-len` | `name-len` | `name` | string | UTF-8, observed `Локальный кластер` |
| `0x24 + host-len + name-len` | `4` | `security-level` | u32_be | observed `0x00000003` |
| `0x28 + host-len + name-len` | `4` | `session-fault-tolerance-level` | u32_be | observed `0x00000004` |
| `0x2c + host-len + name-len` | `4` | `load-balancing-mode` | u32_be | `0=performance`, `1=memory` |
| `0x30 + host-len + name-len` | `4` | `errors-count-threshold` | u32_be | observed `0x00000000` |
| `0x34 + host-len + name-len` | `1` | `kill-problem-processes` | u8 | `0/1` |
| `0x35 + host-len + name-len` | `1` | `kill-by-memory-with-dump` | u8 | `0/1` |

## Record Layout (Observed, v16.0)

Offsets are relative to the start of a record.

- `0x00` `cluster_uuid[16]`
- `0x10` `expiration_timeout:u32_be`
- `0x14` `host_len:u8`
- `0x15` `host[host_len]`
- `0x15 + host_len` `lifetime_limit:u32_be`
- `0x19 + host_len` `port:u16_be`
- `0x1b + host_len` `max_memory_size:u32_be`
- `0x1f + host_len` `max_memory_time_limit:u32_be`
- `0x23 + host_len` `name_len:u8`
- `0x24 + host_len` `name[name_len]`
- `0x24 + host_len + name_len` `tail[32]` (8 x `u32` unknown)

## Tail Example (Bytes, v11.0)

Baseline (all defaults):
- `000000000000000000000000000000000100`

Custom values (security=3, fault-tolerance=4, load-balancing=memory, kill-by-memory=1):
- `000000030000000400000001000000000001`

Flag toggle (kill-problem-processes=1, kill-by-memory=0):
- `000000030000000400000001000000000100`

## Tail Example (Bytes, v16.0)

From the observed record tail:
- `00000000 00000000 00000000 00000000 01000000 00010000 00000000 00000000`

## Open Questions

- Confirm `errors-count-threshold` (`u32`) by setting a non-zero value in a list capture.
- Identify which `tail[32]` slots map to `security-level`, `session-fault-tolerance-level`, `load-balancing-mode`, `kill-*` flags, and `allow-access-right-audit-events-recording`.
- Locate `ping-period`, `ping-timeout`, and `restart-schedule` in the response body (not yet mapped to explicit offsets).

## Gap Analysis (Required)

- `errors-count-threshold` needs a non-zero list capture to confirm its exact slot within `tail[32]`.
- `allow-access-right-audit-events-recording`, `ping-period`, `ping-timeout`, and `restart-schedule` are present in `rac` output but not mapped to exact offsets in v16 response payloads.


## Cluster Info

Source capture:
- `artifacts/rac/v16/v16_20260226_053425_cluster_info_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_cluster_info_server_to_client.decode.txt`

RAC output reference:
- `rac cluster info --cluster <id>`
- `artifacts/rac/v16/v16_20260226_053425_cluster_info_rac.out`
- `artifacts/rac/v16/help/cluster_info.out`

## Fields From `rac` Output

Same field set as `cluster list` (see above).

## RPC Envelope

Request method: `0x0d` (`cluster info --cluster <id>`)
Response method: `0x0e`

## Fields From `rac` Request

Observed request parameters for `rac cluster info`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |

Payload structure (method body):
- single record in the same layout as `cluster list` (no leading count byte)

## Open Questions

- Confirm field semantics that are still unknown in `cluster list`.


## Cluster Admin List

Source capture:
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_response.hex`

RAC output reference:
- `rac cluster admin list --cluster <id> --cluster-user <user> --cluster-pwd <pwd>`
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_rac.out`
- `artifacts/rac/v16/help/cluster_help.txt`

## Fields From `rac` Output

Observed field names in `rac cluster admin list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `name` | string | yes | 1 | 11.0 |
| `descr` | string | yes | 2 | 11.0 |
| `auth` | flags | yes | 3 | 11.0 |
| `os-user` | string | yes | 4 | 11.0 |

## RPC Envelope

Request method: `0x02` (`cluster admin list --cluster <id>`)
Response method: `0x03`

## Fields From `rac` Request

Observed request parameters for `rac cluster admin list`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

## Record Layout (Observed, v16.0)

Offsets are relative to the start of a record.

- `0x00` `name_len:u8`
- `0x01` `name[name_len]` (UTF-8, observed `cadmin`)
- `0x01 + name_len` `descr_len:u8`
- `0x02 + name_len` `descr[descr_len]`
- `0x02 + name_len + descr_len` `unknown_flags:u32_be` (observed raw bytes `03 ef bf bd`)
- `0x06 + name_len + descr_len` `auth_tag:u8` (observed `0x01`)
- `0x07 + name_len + descr_len` `auth_flags:u8` (observed `0x00` (pwd) / `0x01` (pwd|os))
- `0x08 + name_len + descr_len` `os_user_len:u8`
- `0x09 + name_len + descr_len` `os_user[os_user_len]`

## Open Questions

- What is the semantic meaning of `unknown_flags` (`0x03efbfbd`) for cluster admin records?
- Is `auth_tag` always `0x01`, or does it vary by version?


## Cluster Admin Register

Source capture:
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_register_pwd_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_register_pwd_os_client_to_server.decode.txt`

## RPC Envelope

Request method: `0x05` (`cluster admin register --cluster <id> --name <name>`)
Response: `01 00 00 00` (ack only, no method id)

## Fields From `rac` Request

Observed request parameters for `rac cluster admin register`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 4 | 11.0 |
| `descr` | string | yes | 5 | 11.0 |
| `pwd` | string | yes | 6 | 11.0 |
| `auth` | enum | yes (as `auth_flags`) | 7 | 11.0 |
| `os-user` | string | yes | 8 | 11.0 |

## Request Layout (Observed)

Offsets are relative to the start of the method body.

- `0x00` `cluster_uuid[16]`
- `0x10` `name_len:u8`
- `0x11` `name[name_len]`
- `0x11 + name_len` `descr_len:u8`
- `0x12 + name_len` `descr[descr_len]`
- `0x12 + name_len + descr_len` `pwd_len:u8`
- `0x13 + name_len + descr_len` `pwd[pwd_len]`
- `0x13 + name_len + descr_len + pwd_len` `auth_tag:u8` (observed `0x01`)
- `0x14 + name_len + descr_len + pwd_len` `auth_flags:u8` (`0x00` pwd, `0x01` pwd|os)
- `0x15 + name_len + descr_len + pwd_len` `os_user_len:u8`
- `0x16 + name_len + descr_len + pwd_len` `os_user[os_user_len]`

## Open Questions

- Confirm `auth_flags` for `--auth=os` only.
- Confirm whether `auth_tag` is always `0x01`.

## Cluster Admin Remove

Sources:
- `artifacts/rac/v16/help/cluster_help.txt`
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_remove_pwd_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_remove_pwd_os_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_20260226_053425_cluster_admin_remove_pwd_server_to_client.decode.txt`

### RPC Envelope

Request method: `0x07` (`cluster admin remove`)
Response: ACK (`01 00 00 00`)

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster admin remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 4 | 11.0 |

Payload structure (method body):
- offset `0x00`: `cluster_uuid[16]`
- offset `0x10`: `name_len:u8`
- offset `0x11`: `name[name_len]`

### Поля ответа

ACK-only (empty body).

## Cluster Insert

Sources:
- `artifacts/rac/v16/help/cluster_help.txt`

### RPC Envelope

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster insert` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `host` | string | no | unknown | 11.0 |
| `port` | u16 | no | unknown | 11.0 |
| `name` | string | no | unknown | 11.0 |
| `expiration-timeout` | u32 | no | unknown | 11.0 |
| `lifetime-limit` | u32 | no | unknown | 11.0 |
| `max-memory-size` | u32 | no | unknown | 11.0 |
| `max-memory-time-limit` | u32 | no | unknown | 11.0 |
| `security-level` | u32 | no | unknown | 11.0 |
| `session-fault-tolerance-level` | u32 | no | unknown | 11.0 |
| `load-balancing-mode` | enum (`performance`, `memory`) | no | unknown | 11.0 |
| `errors-count-threshold` | u32 | no | unknown | 11.0 |
| `kill-problem-processes` | bool (`yes/no`) | no | unknown | 11.0 |
| `kill-by-memory-with-dump` | bool (`yes/no`) | no | unknown | 11.0 |
| `agent-user` | string | no | unknown | 11.0 |
| `agent-pwd` | string | no | unknown | 11.0 |

### Поля ответа

Not captured yet (likely created `cluster` UUID or ACK-only).

## Cluster Update

Sources:
- `artifacts/rac/v16/help/cluster_help.txt`
- `artifacts/rac/v16/v16_cluster_update_nonzero_retry_client_to_server.decode.txt`
- `artifacts/rac/v16/v16_cluster_update_nonzero_retry_server_to_client.decode.txt`

### RPC Envelope

Request method: `0x0f` (`cluster update`)
Response method: `0x10` (cluster UUID)

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |
| `expiration-timeout` | u32 | yes | 2 | 11.0 |
| `host` | string | yes | 3 | 11.0 |
| `lifetime-limit` | u32 | yes | 4 | 11.0 |
| `port` | u16 | yes | 5 | 11.0 |
| `max-memory-size` | u32 | yes | 6 | 11.0 |
| `max-memory-time-limit` | u32 | yes | 7 | 11.0 |
| `name` | string | yes | 8 | 11.0 |
| `security-level` | u32 | yes | 9 | 11.0 |
| `session-fault-tolerance-level` | u32 | yes | 10 | 11.0 |
| `load-balancing-mode` | enum (`performance`, `memory`) | yes | 11 | 11.0 |
| `errors-count-threshold` | u32 | yes | 12 | 11.0 |
| `kill-problem-processes` | bool (`yes/no`) | yes | 13 | 11.0 |
| `kill-by-memory-with-dump` | bool (`yes/no`) | yes | 14 | 11.0 |
| `agent-user` | string | yes (auth call `0x08`) | 15 | 11.0 |
| `agent-pwd` | string | yes (auth call `0x08`) | 16 | 11.0 |

Payload structure (method body):
- offset `0x00`: `cluster_uuid[16]`
- offset `0x10`: `expiration_timeout:u32_be`
- offset `0x14`: `host_len:u8`
- offset `0x15`: `host[host_len]`
- offset `0x15 + host_len`: `lifetime_limit:u32_be`
- offset `0x19 + host_len`: `port:u16_be`
- offset `0x1b + host_len`: `max_memory_size:u32_be`
- offset `0x1f + host_len`: `max_memory_time_limit:u32_be`
- offset `0x23 + host_len`: `name_len:u8`
- offset `0x24 + host_len`: `name[name_len]`
- offset `0x24 + host_len + name_len`: `tail[32]` (8 x `u32` unknown)

### Поля ответа

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |

## Cluster Remove

Sources:
- `artifacts/rac/v16/help/cluster_help.txt`

### RPC Envelope

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | no | unknown | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).
