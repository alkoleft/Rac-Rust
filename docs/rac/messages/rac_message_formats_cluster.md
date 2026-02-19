# RAC Cluster Message Formats (Observed)

## Cluster List

Source capture:
- `logs/session_1771110767_483969_127_0_0_1_48522`

RAC output reference:
- `rac cluster list`

## Fields From `rac` Output

Observed field names in `rac cluster list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `host` | string | yes | 3 |
| `port` | u16 | yes | 5 |
| `name` | string | yes | 7 |
| `expiration-timeout` | u32 | yes | 2 |
| `lifetime-limit` | unknown | no | - |
| `max-memory-size` | unknown | no | - |
| `max-memory-time-limit` | unknown | no | - |
| `security-level` | unknown | no | - |
| `session-fault-tolerance-level` | unknown | no | - |
| `load-balancing-mode` | unknown | no | - |
| `errors-count-threshold` | unknown | no | - |
| `kill-problem-processes` | unknown | no | - |
| `kill-by-memory-with-dump` | unknown | no | - |
| `allow-access-right-audit-events-recording` | unknown | no | - |
| `ping-period` | unknown | no | - |
| `ping-timeout` | unknown | no | - |
| `restart-schedule` | unknown | no | - |

## RPC Envelope

Request method: `0x0b` (`cluster list`)
Response method: `0x0c`

## Fields From `rac` Request

Observed request parameters for `rac cluster list`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| *(none)* | - | n/a | - |

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here
- subsequent records start at the next cluster UUID occurrence

## Record Layout (Observed)

Offsets are relative to the start of a record.

- `0x00` `cluster_uuid[16]`
- `0x10` `expiration_timeout:u32_be` (observed `0x0000003c` -> 60)
- `0x14` `host_len:u8`
- `0x15` `host[host_len]` (UTF-8, observed `alko-home`)
- `0x15 + host_len` `unknown_0:u32_be` (observed `0x00000000`)
- `0x19 + host_len` `port:u16_be` (observed `0x0605` -> 1541)
- `0x1b + host_len` `unknown_1:u64_be` (observed `0x0000000000000000`)
- `0x23 + host_len` `name_len:u8`
- `0x24 + host_len` `name[name_len]` (UTF-8, observed `Локальный кластер`)
- `0x24 + host_len + name_len` `tail[32]` (8 x `u32` unknown)

## Tail Example (Bytes)

From the observed record tail:
- `00000000 00000000 00000000 00000000 01000000 00010000 00000000 00000000`

## Open Questions

- Confirm `tail[32]` field semantics (8 x `u32`).


## Cluster Info

Source capture:
- `logs/session_1771110778_484133_127_0_0_1_39376`

RAC output reference:
- `rac cluster info --cluster <id>`

## Fields From `rac` Output

Same field set as `cluster list` (see above).

## RPC Envelope

Request method: `0x0d` (`cluster info --cluster <id>`)
Response method: `0x0e`

## Fields From `rac` Request

Observed request parameters for `rac cluster info`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |

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

## Fields From `rac` Output

Observed field names in `rac cluster admin list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `name` | string | yes | 1 |
| `auth` | unknown | no | - |
| `os-user` | unknown | no | - |
| `descr` | unknown | no | - |

## RPC Envelope

Request method: `0x02` (`cluster admin list --cluster <id>`)
Response method: `0x03`

## Fields From `rac` Request

Observed request parameters for `rac cluster admin list`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 |

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

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 |
| `name` | string | yes | 4 |
| `descr` | string | yes | 5 |
| `pwd` | string | yes | 6 |
| `auth` | enum | yes (as `auth_flags`) | 7 |

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
