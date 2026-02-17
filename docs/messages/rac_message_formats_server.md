# RAC Server Message Formats (Observed)

## Server List

Source capture:
- `logs/session_1771331867_3319466_127_0_0_1_34690/server_to_client.stream.bin`

Payload example:
- `artifacts/server_list_response.hex`

RAC output reference:
- `artifacts/server_list_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac server list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `server` | UUID | yes | 1 |
| `agent-host` | string | yes | 2 |
| `agent-port` | u16 | yes | 3 |
| `port-range` | u16+u16 | hypothesis | 4 |
| `name` | string | yes | 5 |
| `using` | enum (u32) | hypothesis | 6 |
| `dedicate-managers` | enum (u32) | hypothesis | 7 |
| `infobases-limit` | u32 | hypothesis | 8 |
| `memory-limit` | u64 | hypothesis | 9 |
| `connections-limit` | u32 | hypothesis | 10 |
| `safe-working-processes-memory-limit` | u64 | hypothesis | 11 |
| `safe-call-memory-limit` | u64 | hypothesis | 12 |
| `cluster-port` | u16 | yes | 13 |
| `critical-total-memory` | u64 | hypothesis | 14 |
| `temporary-allowed-total-memory` | u64 | hypothesis | 15 |
| `temporary-allowed-total-memory-time-limit` | u32 | hypothesis | 16 |
| `service-principal-name` | string | no | - |
| `restart-schedule` | string | no | - |

### RPC

Request method: `0x16` (`server list --cluster <id>`)
Response method: `0x17`

### Поля запроса (из `rac`)

Observed request parameters for `rac server list`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 |

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `server` | UUID | |
| `0x10` | `1` | `agent-host-len` | u8 | |
| `0x11` | `agent-host-len` | `agent-host` | string | UTF-8, observed `alko-home` |
| `0x11 + agent-host-len` | `2` | `agent-port` | u16_be | observed `0x0604` -> `1540` |
| `0x13 + agent-host-len` | `1` | `name-len` | u8 | |
| `0x14 + agent-host-len` | `name-len` | `name` | string | UTF-8, observed `Центральный сервер` |
| `base + 0x00` | `4` | `using` | u32_le? | observed bytes `01 00 00 00` (endianness uncertain) |
| `base + 0x04` | `4` | `dedicate-managers` | u32_le? | observed `0` |
| `base + 0x08` | `8` | `memory-limit` | u64? | observed `0` |
| `base + 0x10` | `4` | `gap_1` | gap | unknown (all zeros) |
| `base + 0x14` | `4` | `infobases-limit` | u32_le? | observed bytes `08 00 00 00` |
| `base + 0x18` | `4` | `gap_2` | gap | unknown (all zeros) |
| `base + 0x1c` | `4` | `gap_3` | gap | observed bytes `00 00 00 01` |
| `base + 0x21` | `2` | `cluster-port` | u16_be | observed `0x0605` -> `1541` |
| `base + 0x25` | `2` | `port-range-end` | u16_be | observed `0x0637` -> `1591` |
| `base + 0x27` | `2` | `port-range-start` | u16_be | observed `0x0618` -> `1560` |
| `base + 0x29` | `0x14` | `gap_4` | gap | unknown (all zeros) |
| `base + 0x3d` | `4` | `temporary-allowed-total-memory-time-limit` | u32_be? | observed bytes `00 00 01 2c` -> `300` |
| `base + 0x41` | `3` | `gap_5` | gap | unknown (all zeros) |

Notes:
- `base = 0x14 + agent-host-len + name-len` (observed `0x40` in this capture).
- Gaps include possible numeric fields listed in `rac` output but not yet mapped.

## Server Info

Source capture:
- `logs/session_1771331875_3319584_127_0_0_1_37116/server_to_client.stream.bin`

Payload example:
- `artifacts/server_info_response.hex`

RAC output reference:
- `artifacts/server_info_rac.out`

### Поля ответа (из `rac`)

Same field set as `server list` (see above).

### RPC

Request method: `0x18` (`server info --cluster <id> --server <id>`)
Response method: `0x19`

### Поля запроса (из `rac`)

Observed request parameters for `rac server info`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `server` | UUID | yes | 2 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 3 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 4 |

Payload structure (method body):
- single record in the same layout as `server list` (no leading count byte)

## Hypotheses

- `using` and `dedicate-managers` are u32 values but endianness is not confirmed.
- `infobases-limit` uses u32 (little-endian observed), but verify with non-zero changes.
- `port-range` order might be reversed on wire (observed end before start).
- `temporary-allowed-total-memory-time-limit` is likely `u32_be` at `base + 0x3d`.

## Open Questions

- Where exactly are `connections-limit`, `safe-working-processes-memory-limit`,
  `safe-call-memory-limit`, `critical-total-memory`, `temporary-allowed-total-memory`
  encoded in the numeric block after `name`?

## Gap Analysis (Required)

- `base + 0x10` (size `4`): candidate `connections-limit` (u32_be/u32_le).
  - Capture change: set `--connections-limit` to a non-zero, non-256 value.
- `base + 0x18` (size `4`): candidate `safe-working-processes-memory-limit` (u32 or part of u64).
  - Capture change: set `--safe-working-processes-memory-limit` to a non-zero value.
- `base + 0x1c` (size `4`, bytes `00 00 00 01`): candidate `connections-limit` (u32_be=1 or bitmask).
  - Capture change: set `--connections-limit` to 128/512 to disambiguate endianness.
- `base + 0x29` (size `0x14`): candidate block for `critical-total-memory`,
  `temporary-allowed-total-memory`, and `safe-call-memory-limit` (likely u64/u32).
  - Capture change: set `--critical-total-memory`, `--temporary-allowed-total-memory`,
    and `--safe-call-memory-limit` to distinct non-zero values.
- Missing `service-principal-name` / `restart-schedule` fields:
  - Capture change: set SPN and restart schedule to non-empty values.
