# RAC Cluster Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11/help/cluster_help.txt`
- `artifacts/rac/v11/help/cluster_list.out`
- `artifacts/rac/v11/help/cluster_info.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Cluster List

Source capture:
- `artifacts/rac/v11/v11_cluster_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_cluster_list_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11/v11_cluster_list_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_cluster_list_ro_rac.out`
- `artifacts/rac/v11/help/cluster_list.out`

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
| `max-memory-size` | u32 | hypothesis | 6 | 11.0 |
| `max-memory-time-limit` | u32 | hypothesis | 7 | 11.0 |
| `security-level` | u32 | yes | 9 | 11.0 |
| `session-fault-tolerance-level` | u32 | yes | 10 | 11.0 |
| `load-balancing-mode` | u32 | yes | 11 | 11.0 |
| `errors-count-threshold` | u32 | hypothesis | 12 | 11.0 |
| `kill-problem-processes` | u8 | yes | 13 | 11.0 |
| `kill-by-memory-with-dump` | u8 | yes | 14 | 11.0 |
| `allow-access-right-audit-events-recording` | unknown | no | - | 11.0 |
| `ping-period` | unknown | no | - | 16.0 |
| `ping-timeout` | unknown | no | - | 16.0 |
| `restart-schedule` | unknown | no | - | 11.0 |

## RPC Envelope

Request method: `0x0b` (`cluster list`)
Response method: `0x0c`

## Fields From `rac` Request

Observed request parameters for `rac cluster list`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| *(none)* | - | n/a | - | 11.0 |

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
- `0x15 + host_len` `unknown_0:u32_be`
- `0x19 + host_len` `port:u16_be`
- `0x1b + host_len` `unknown_1:u64_be`
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

- Confirm `max-memory-size` and `max-memory-time-limit` values by setting non-zero and re-capturing.
- Confirm `errors-count-threshold` (`u32`) by setting non-zero value.
- Identify where `allow-access-right-audit-events-recording` and `restart-schedule` are encoded.
- Confirm `tail[32]` field semantics (8 x `u32`).

## Gap Analysis (Required)

- `max-memory-size` at `0x1b + host-len` (4 bytes) and `max-memory-time-limit` at `0x1f + host-len` (4 bytes): candidates `u32_be`. To confirm, set non-zero values and re-capture.
- `errors-count-threshold` at `0x30 + host-len + name-len` (4 bytes): candidate `u32_be` percentage. To confirm, set `--errors-count-threshold` to a non-zero value and re-capture.
- Missing fields `allow-access-right-audit-events-recording` and `restart-schedule`: capture with a non-zero audit flag and a non-empty schedule string.


## Cluster Info

Source capture:
- `artifacts/rac/v11/v11_cluster_info_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_cluster_info_ro_server_to_client.decode.txt`

RAC output reference:
- `rac cluster info --cluster <id>`
- `artifacts/rac/v11/v11_cluster_info_ro_rac.out`
- `artifacts/rac/v11/help/cluster_info.out`

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
- `logs/session_1771286338_2771856_127_0_0_1_59686`

Payload example:
- `artifacts/rac/cluster_admin_list_request.hex`
- `artifacts/rac/cluster_admin_list_response.hex`

RAC output reference:
- `rac cluster admin list --cluster <id> --cluster-user <user> --cluster-pwd <pwd>`
- `artifacts/rac/v11/help/cluster_help.txt`

## Fields From `rac` Output

Observed field names in `rac cluster admin list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `name` | string | yes | 1 | 11.0 |
| `auth` | unknown | no | - | 16.0 |
| `os-user` | unknown | no | - | 16.0 |
| `descr` | unknown | no | - | 16.0 |

## RPC Envelope

Request method: `0x02` (`cluster admin list --cluster <id>`)
Response method: `0x03`

## Fields From `rac` Request

Observed request parameters for `rac cluster admin list`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 11.0 |

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

## Record Layout (Observed, Hypothesis)

Offsets are relative to the start of a record.

- `0x00` `name_len:u8`
- `0x01` `name[name_len]` (UTF-8, observed `cadmin`)
- `0x01 + name_len` `unknown_0:u8` (observed `0x00`)
- `0x02 + name_len` `unknown_1:u32_be?` (observed raw bytes `03 ef bf bd`)
- `0x06 + name_len` `unknown_2[3]` (observed `01 00 00`)

## Open Questions

- Identify `unknown_1` and `unknown_2` by capturing multiple admin records.


## Cluster Admin Register

Source capture:
- `logs/session_1771286512_2774186_127_0_0_1_37332`

Payload example:
- `artifacts/rac/cluster_admin_register_request.hex`

## RPC Envelope

Request method: `0x05` (`cluster admin register --cluster <id> --name <name>`)
Response: `01 00 00 00` (ack only, no method id)

## Fields From `rac` Request

Observed request parameters for `rac cluster admin register`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `name` | string | yes | 4 | 11.0 |
| `descr` | string | yes | 5 | 11.0 |
| `pwd` | string | yes | 6 | 11.0 |
| `auth` | enum | yes (as `auth_flags`) | 7 | 11.0 |

## Request Layout (Observed)

Offsets are relative to the start of the method body.

- `0x00` `cluster_uuid[16]`
- `0x10` `name_len:u8`
- `0x11` `name[name_len]`
- `0x11 + name_len` `descr_len:u8`
- `0x12 + name_len` `descr[descr_len]`
- `0x12 + name_len + descr_len` `pwd_len:u8`
- `0x13 + name_len + descr_len` `pwd[pwd_len]`
- `0x13 + name_len + descr_len + pwd_len` `auth_flags:u8` (observed `0x01` for `--auth=pwd`)
- `0x14 + name_len + descr_len + pwd_len` `unknown_0:u16_be` (observed `0x0000`)

## Open Questions

- Confirm `auth_flags` for `--auth=pwd,os` and OS-only cases.
- Identify the meaning of trailing `unknown_0`.

## Cluster Admin Remove

Sources:
- `artifacts/rac/v11/help/cluster_help.txt`

### RPC Envelope

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster admin remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Cluster Insert

Sources:
- `artifacts/rac/v11/help/cluster_help.txt`

### RPC Envelope

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster insert` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `host` | string | no | - | 11.0 |
| `port` | u16 | no | - | 11.0 |
| `name` | string | no | - | 11.0 |
| `expiration-timeout` | u32 | no | - | 11.0 |
| `lifetime-limit` | u32 | no | - | 11.0 |
| `max-memory-size` | u32 | no | - | 11.0 |
| `max-memory-time-limit` | u32 | no | - | 11.0 |
| `security-level` | u32 | no | - | 11.0 |
| `session-fault-tolerance-level` | u32 | no | - | 11.0 |
| `load-balancing-mode` | enum (`performance`, `memory`) | no | - | 11.0 |
| `errors-count-threshold` | u32 | no | - | 11.0 |
| `kill-problem-processes` | bool (`yes/no`) | no | - | 11.0 |
| `kill-by-memory-with-dump` | bool (`yes/no`) | no | - | 11.0 |
| `agent-user` | string | no | - | 11.0 |
| `agent-pwd` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely created `cluster` UUID or ACK-only).

## Cluster Update

Sources:
- `artifacts/rac/v11/help/cluster_help.txt`

### RPC Envelope

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | no | - | 11.0 |
| `name` | string | no | - | 11.0 |
| `expiration-timeout` | u32 | no | - | 11.0 |
| `lifetime-limit` | u32 | no | - | 11.0 |
| `max-memory-size` | u32 | no | - | 11.0 |
| `max-memory-time-limit` | u32 | no | - | 11.0 |
| `security-level` | u32 | no | - | 11.0 |
| `session-fault-tolerance-level` | u32 | no | - | 11.0 |
| `load-balancing-mode` | enum (`performance`, `memory`) | no | - | 11.0 |
| `errors-count-threshold` | u32 | no | - | 11.0 |
| `kill-problem-processes` | bool (`yes/no`) | no | - | 11.0 |
| `kill-by-memory-with-dump` | bool (`yes/no`) | no | - | 11.0 |
| `agent-user` | string | no | - | 11.0 |
| `agent-pwd` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Cluster Remove

Sources:
- `artifacts/rac/v11/help/cluster_help.txt`

### RPC Envelope

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac cluster remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).
