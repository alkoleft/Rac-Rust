# RAC Process Message Formats (Observed)

## Process List

Source capture:
- `logs/session_1771287567_2788242_127_0_0_1_42314/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/process_list_response.hex`

RAC output reference:
- `artifacts/rac/process_list_rac.out`

### Fields From `rac` Output

Observed field names in `rac process list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `process` | UUID | yes | 1 |
| `host` | string | yes | 2 |
| `port` | u16 | yes | 10 |
| `pid` | string (digits) | yes | 13 |
| `turned-on` | bool | hypothesis | - |
| `running` | bool | yes | 17 |
| `started-at` | datetime | yes | 16 |
| `use` | enum | yes | 14 |
| `available-performance` | u32 | yes | 18 |
| `capacity` | u32 | yes | 3 |
| `connections` | u32 | yes | 4 |
| `memory-size` | u32 | yes | 12 |
| `memory-excess-time` | u32 | hypothesis | 11 |
| `selection-size` | u32 | yes | 15 |
| `avg-call-time` | f64 | yes | 5 |
| `avg-db-call-time` | f64 | yes | 6 |
| `avg-lock-call-time` | f64 | yes | 7 |
| `avg-server-call-time` | f64 | yes | 8 |
| `avg-threads` | f64 | yes | 9 |
| `reserve` | bool | hypothesis | 19 |

### RPC Envelope

Request method: `0x1d` (`process list --cluster <id>`)
Response method: `0x1e`

### Fields From `rac` Request

Observed request parameters for `rac process list`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 |

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

### Message Description

- `count:u8` followed by `count` process records.
- Each record starts with `process:uuid16` and a fixed prefix up to `host`.
- `gap_0` is an unknown 8-byte field.
- `avg-*` metrics are consecutive `f64_be` values.
- After `host`, the record always carries a license block (even if `--licenses` is not used in CLI output).
- The tail after the license block contains `port`, `pid`, `memory-size`, `selection-size`, and `available-performance`, plus several still-unknown fields.

### Record Layout (Observed, Partial)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `process` | `uuid16` | record anchor |
| `0x10` | `0x08` | `gap_0` | `bytes` | unknown; not matching 1C datetime in this capture |
| `0x18` | `8` | `avg-call-time` | `f64_be` | observed ~`4.115` |
| `0x20` | `8` | `avg-db-call-time` | `f64_be` | observed non-zero with `--licenses` |
| `0x28` | `8` | `avg-lock-call-time` | `f64_be` | observed non-zero with `--licenses` |
| `0x30` | `8` | `avg-server-call-time` | `f64_be` | observed ~`4.115` |
| `0x38` | `8` | `avg-threads` | `f64_be` | observed ~`1.014` |
| `0x40` | `4` | `capacity` | `u32_be` | observed `1000` |
| `0x44` | `4` | `connections` | `u32_be` | observed `7` |
| `0x48` | `1` | `host_len` | `u8` | length of `host` |
| `0x49` | `host_len` | `host` | `str8` | UTF-8, observed `alko-home` |
| `0x49 + host_len` | `tail` | `license_block + tail_gap` | `bytes` | license block is always present; remaining tail is unknown |

## Process List (`--licenses`)

Source capture:
- `logs/session_1771288283_2797451_127_0_0_1_55460/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/process_list_licenses_response.hex`

RAC output reference:
- `artifacts/rac/process_list_licenses_rac.out`

### Fields From `rac` Output (`--licenses`)

Observed field names in `rac process list --licenses` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `full-name` | string | yes | - |
| `series` | string | yes | - |
| `issued-by-server` | bool | yes | - |
| `license-type` | enum | yes | - |
| `net` | bool | yes | - |
| `max-users-all` | u32 | yes | - |
| `max-users-cur` | u32 | yes | - |
| `rmngr-address` | string | yes | - |
| `rmngr-port` | u32 | yes | - |
| `rmngr-pid` | string (digits) | yes | - |
| `short-presentation` | string | yes | - |
| `full-presentation` | string | yes | - |

### Record Layout Notes (`--licenses`)

Record layout offsets above remain valid; in this capture:
- `avg-call-time` ~`0.134`
- `avg-db-call-time` ~`0.00108`
- `avg-lock-call-time` ~`0.00243`
- `avg-server-call-time` ~`0.122`
- `avg-threads` ~`1.399`
- `connections` = `9` (in this capture)

License block structure matches the session license block starting from `issued-by-server`.
Offsets are relative to the start of a record (recStart = record start, not payload start).

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x52` | `1` | `licenses-count` | `u8` | observed `0x01` |
| `0x53` | `1` | `gap_license_0` | `u8` | unknown flag before `file-name` |
| `0x54` | `1` | `file-name-len` | `u8` | observed `0x37` |
| `0x55` | `file-name-len` | `file-name` | `str8` | UTF-8 path to `.lic` |
| `0x8c` | `2` | `full-presentation-len` | `u14` | length = `(b0 & 0x3f) + (b1 << 6)` |
| `0x8e` | `full-presentation` | `str8` | UTF-8, length via `u14` above |
| `0x118` | `1` | `issued-by-server` | `u8` | observed `0x01` |
| `0x119` | `4` | `license-type` | `u32_be` | observed `0` (`soft`) |
| `0x11d` | `4` | `max-users-all` | `u32_be` | observed `4` |
| `0x121` | `4` | `max-users-cur` | `u32_be` | observed `4` |
| `0x125` | `1` | `network-key` | `u8` | observed `0x00` (`net=no`) |
| `0x126` | `1` | `server-address-len` | `u8` | observed `0x09` |
| `0x127` | `server-address-len` | `server-address` | `str8` | observed `alko-home` |
| `0x130` | `1` | `process-id-len` | `u8` | observed `0x06` |
| `0x131` | `process-id` | `str8` | observed `314150` |
| `0x137` | `4` | `server-port` | `u32_be` | observed `1560` |
| `0x13b` | `1` | `key-series-len` | `u8` | observed `0x0c` |
| `0x13c` | `key-series` | `str8` | observed `500000025347` |
| `0x148` | `1` | `brief-presentation-len` | `u8` | observed `0x1e` |
| `0x149` | `brief-presentation` | `str8` | observed `Сервер, 500000025347 4 4` |

### Tail Layout After License Block (Observed, Partial)

Define `t0` as the byte immediately after `brief-presentation` (end of string). Offsets below are relative to `t0`.

| Offset (t0 +) | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `2` | `port` | `u16_be` | observed `1560` |
| `0x02` | `4` | `memory-excess-time` | `u32_be` | hypothesis, observed `0` |
| `0x06` | `4` | `memory-size` | `u32_be` | observed `682224` |
| `0x0a` | `1` | `pid_len` | `u8` | observed `0x06` |
| `0x0b` | `pid_len` | `pid` | `str8` | observed `314150` |
| `0x0b + pid_len` | `4` | `use` | `u32_be` | observed `1` when `use=used` |
| `0x0f + pid_len` | `4` | `selection-size` | `u32_be` | observed `21944` / `3625` |
| `0x13 + pid_len` | `8` | `started-at` | `datetime` | 1C timestamp (u64_be) |
| `0x1b + pid_len` | `4` | `running` | `u32_be` | observed `1` when `running=yes` |
| `0x1f + pid_len` | `4` | `available-performance` | `u32_be` | observed `153` / `192` |
| `0x23 + pid_len` | `1` | `reserve` | `u8` | hypothesis, observed `0` when `reserve=no` |

## Process Info

Source capture:
- `logs/session_1771287578_2788405_127_0_0_1_40088/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/process_info_response.hex`

RAC output reference:
- `artifacts/rac/process_info_rac.out`

### Fields From `rac` Output

Same field set as `process list` (see above).

### RPC Envelope

Request method: `0x1f` (`process info --cluster <id> --process <id>`)
Response method: `0x20`

### Fields From `rac` Request

Observed request parameters for `rac process info`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `process` | UUID | yes | 2 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 3 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 4 |

Payload structure (method body):
- single record in the same layout as `process list` (no leading count byte)

### Message Description

- Single process record with the same layout as `process list` records.
- `gap_0` and tail are present in the same positions and currently only partially decoded.

### Record Layout (Observed, Partial)

Offsets are relative to the start of the record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `process` | `uuid16` | record anchor |
| `0x10` | `0x08` | `gap_0` | `bytes` | unknown; not matching 1C datetime in this capture |
| `0x18` | `8` | `avg-call-time` | `f64_be` | observed ~`4.107` |
| `0x20` | `8` | `avg-db-call-time` | `f64_be` | observed non-zero with `--licenses` |
| `0x28` | `8` | `avg-lock-call-time` | `f64_be` | observed non-zero with `--licenses` |
| `0x30` | `8` | `avg-server-call-time` | `f64_be` | observed ~`4.107` |
| `0x38` | `8` | `avg-threads` | `f64_be` | observed ~`1.014` |
| `0x40` | `4` | `capacity` | `u32_be` | observed `1000` |
| `0x44` | `4` | `connections` | `u32_be` | observed `7` |
| `0x48` | `1` | `host_len` | `u8` | length of `host` |
| `0x49` | `host_len` | `host` | `str8` | UTF-8, observed `alko-home` |
| `0x49 + host_len` | `tail` | `license_block + tail_gap` | `bytes` | license block is always present; remaining tail is unknown |

## Process Info (`--licenses`)

Source capture:
- `logs/session_1771288300_2797644_127_0_0_1_33814/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/process_info_licenses_response.hex`

RAC output reference:
- `artifacts/rac/process_info_licenses_rac.out`

### Fields From `rac` Output (`--licenses`)

Same field set as `process list --licenses` (see above).

## Open Questions

- `gap_0` is still unknown; does not match 1C datetime ticks in this capture.
- Confirm `use` mapping (`u32_be` values observed as `1` when `use=used`).
- Confirm `reserve` mapping (`u8` values observed as `0` when `reserve=no`).
