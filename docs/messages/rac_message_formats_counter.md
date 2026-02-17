# RAC Counter Message Formats (Observed)

## Counter List

Source capture:
- `logs/session_1771345707_3496864_127_0_0_1_36988/server_to_client.stream.bin`

Payload example:
- `artifacts/counter_list_response.hex`

RAC output reference:
- `artifacts/counter_list_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac counter list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `name` | string | yes | 1 |
| `collection-time` | u64_be | yes | 2 |
| `group` | enum (u8) | yes | 3 |
| `filter-type` | enum (u8) | yes | 4 |
| `filter` | string | yes | 5 |
| `duration` | enum (u8) | yes | 6 |
| `cpu-time` | enum (u8) | yes | 7 |
| `duration-dbms` | enum (u8) | yes | 8 |
| `service` | enum (u8) | yes | 9 |
| `memory` | enum (u8) | yes | 10 |
| `read` | enum (u8) | yes | 11 |
| `write` | enum (u8) | yes | 12 |
| `dbms-bytes` | enum (u8) | yes | 13 |
| `call` | enum (u8) | yes | 14 |
| `number-of-active-sessions` | enum (u8) | yes | 15 |
| `number-of-sessions` | enum (u8) | yes | 16 |
| `descr` | string | yes | 17 |

### RPC

Request method: `0x76` (`counter list --cluster <id>`)
Response method: `0x77`

Payload structure (method body):
- offset `0x00`: `16 <cluster_uuid_16b>`

### Поля запроса (из `rac`)

Observed request parameters for `rac counter list`.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `cluster` | UUID | yes | 1 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `name-len` | u8 | |
| `0x01` | `name-len` | `name` | string | UTF-8 |
| `0x01 + name-len` | `8` | `collection-time` | u64_be | `0` => `current-call` |
| `0x09 + name-len` | `1` | `group` | enum (u8) | `0` -> `users`, `1` -> `data-separation` |
| `0x0a + name-len` | `1` | `filter-type` | enum (u8) | `0` -> `all-selected`, `1` -> `all-but-selected`, `2` -> `all` |
| `0x0b + name-len` | `1` | `filter-len` | u8 | |
| `0x0c + name-len` | `filter-len` | `filter` | string | UTF-8 digits, empty when `filter-len=0` |
| `base + 0x00` | `1` | `duration` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x01` | `1` | `cpu-time` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x02` | `1` | `duration-dbms` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x03` | `1` | `service` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x04` | `1` | `memory` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x05` | `1` | `read` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x06` | `1` | `write` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x07` | `1` | `dbms-bytes` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x08` | `1` | `call` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x09` | `1` | `number-of-active-sessions` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x0a` | `1` | `number-of-sessions` | enum (u8) | `1` -> `analyze`, `0` -> `not-analyze` |
| `base + 0x0b` | `1` | `descr-len` | u8 | |
| `base + 0x0c` | `descr-len` | `descr` | string | UTF-8 |

Notes:
- `base = 0x0c + name-len + filter-len`.
- All records share the same fixed ordering for the 11 analyze flags.

### Hypotheses

- Enum mappings for `group` and `filter-type` are inferred from a single capture and need confirmation with alternative values.

### Open Questions

- Does `collection-time=0` always mean `current-call` for all server builds?

### Gap Analysis (Required)

- No unknown byte regions remain in the record layout.
- To confirm enum mappings, capture a `counter list` with different `group`/`filter-type` combinations and verify the encoded values.
