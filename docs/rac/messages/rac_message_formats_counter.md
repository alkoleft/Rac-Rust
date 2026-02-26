# RAC Counter Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11/help/counter_help.txt`
- `artifacts/rac/v11/help/counter_list.out`
- `artifacts/rac/v11/help/counter_info.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Counter List

Source capture:
- `artifacts/rac/v11/v11_counter_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_counter_list_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11/v11_counter_list_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_counter_list_ro_rac.out`
- `artifacts/rac/v11/help/counter_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac counter list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 11.0 |
| `collection-time` | u64_be | yes | 2 | 11.0 |
| `group` | enum (u8) | yes | 3 | 11.0 |
| `filter-type` | enum (u8) | yes | 4 | 11.0 |
| `filter` | string | yes | 5 | 11.0 |
| `duration` | enum (u8) | yes | 6 | 11.0 |
| `cpu-time` | enum (u8) | yes | 7 | 11.0 |
| `duration-dbms` | enum (u8) | yes | 8 | 11.0 |
| `service` | enum (u8) | yes | 9 | 11.0 |
| `memory` | enum (u8) | yes | 10 | 11.0 |
| `read` | enum (u8) | yes | 11 | 11.0 |
| `write` | enum (u8) | yes | 12 | 11.0 |
| `dbms-bytes` | enum (u8) | yes | 13 | 11.0 |
| `call` | enum (u8) | yes | 14 | 11.0 |
| `number-of-active-sessions` | enum (u8) | yes | 15 | 11.0 |
| `number-of-sessions` | enum (u8) | yes | 16 | 11.0 |
| `descr` | string | yes | 17 | 11.0 |

### RPC

Request method: `0x76` (`counter list --cluster <id>`)
Response method: `0x77`

Payload structure (method body):
- offset `0x00`: `16 <cluster_uuid_16b>`

### Поля запроса (из `rac`)

Observed request parameters for `rac counter list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 11.0 |

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

## Counter Info

Source capture:
- `artifacts/rac/v11/v11_counter_info_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_counter_info_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11/v11_counter_info_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_counter_info_ro_rac.out`
- `artifacts/rac/v11/help/counter_info.out`

### Поля ответа (из `rac`)

Observed field names in `rac counter info` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 11.0 |
| `collection-time` | u64_be | yes | 2 | 11.0 |
| `group` | enum (u8) | yes | 3 | 11.0 |
| `filter-type` | enum (u8) | yes | 4 | 11.0 |
| `filter` | string | yes | 5 | 11.0 |
| `duration` | enum (u8) | yes | 6 | 11.0 |
| `cpu-time` | enum (u8) | yes | 7 | 11.0 |
| `duration-dbms` | enum (u8) | yes | 8 | 11.0 |
| `service` | enum (u8) | yes | 9 | 11.0 |
| `memory` | enum (u8) | yes | 10 | 11.0 |
| `read` | enum (u8) | yes | 11 | 11.0 |
| `write` | enum (u8) | yes | 12 | 11.0 |
| `dbms-bytes` | enum (u8) | yes | 13 | 11.0 |
| `call` | enum (u8) | yes | 14 | 11.0 |
| `number-of-active-sessions` | enum (u8) | yes | 15 | 11.0 |
| `number-of-sessions` | enum (u8) | yes | 16 | 11.0 |
| `descr` | string | yes | 17 | 11.0 |

### RPC

Request method: `0x78` (`counter info --cluster <id> --counter <name>`)
Response method: `0x79`

Payload structure (method body):
- offset `0x00`: counter record (same layout as `counter list` record)

### Поля запроса (из `rac`)

Observed request parameters for `rac counter info`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `counter` | string | yes | 2 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 4 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the counter record.

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

### Hypotheses

- Enum mappings reuse the `counter list` capture and need additional confirmation.

### Open Questions

- Is the `info` response always a single record with the same layout as `list` on all server builds?

### Gap Analysis (Required)

- No unknown byte regions remain in the record layout.

## Counter Update

Source capture:
- `logs/session_1771346554_3508050_127_0_0_1_60770/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/counter_update_codex_tmp_response.hex`

RAC output reference:
- `artifacts/rac/v16/counter_update_codex_tmp_rac.out`

### Поля ответа (из `rac`)

`rac counter update` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 16.0 |

### RPC

Request method: `0x7a` (`counter update --cluster <id> --name <name> ...`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00`: `cluster:uuid16`
- offset `0x10`: counter record starts here

### Поля запроса (из `rac`)

Observed request parameters for `rac counter update`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `name` | string | yes | 4 | 11.0 |
| `collection-time` | u64_be | yes | 5 | 11.0 |
| `group` | enum (u8) | yes | 6 | 11.0 |
| `filter-type` | enum (u8) | yes | 7 | 11.0 |
| `filter` | string | yes | 8 | 11.0 |
| `duration` | enum (u8) | yes | 9 | 11.0 |
| `cpu-time` | enum (u8) | yes | 10 | 11.0 |
| `duration-dbms` | enum (u8) | yes | 11 | 11.0 |
| `service` | enum (u8) | yes | 12 | 11.0 |
| `memory` | enum (u8) | yes | 13 | 11.0 |
| `read` | enum (u8) | yes | 14 | 11.0 |
| `write` | enum (u8) | yes | 15 | 11.0 |
| `dbms-bytes` | enum (u8) | yes | 16 | 11.0 |
| `call` | enum (u8) | yes | 17 | 11.0 |
| `number-of-active-sessions` | enum (u8) | yes | 18 | 11.0 |
| `number-of-sessions` | enum (u8) | yes | 19 | 11.0 |
| `descr` | string | yes | 20 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the counter record (immediately after `cluster:uuid16`).

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

### Hypotheses

- The ACK marker `0x01000000` likely encodes a success status; meaning needs confirmation.

### Open Questions

- Is the ACK marker stable across error cases?

### Gap Analysis (Required)

- No unknown byte regions remain in the request record layout.

## Counter Values

Source capture:
- `artifacts/rac/v11/v11_counter_values_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_counter_values_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11/v11_counter_values_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_counter_values_ro_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac counter values` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `object` | string | yes | 1 | 11.0 |
| `collection-time` | u64_be | yes | 2 | 11.0 |
| `duration` | u64_be | yes | 3 | 11.0 |
| `cpu-time` | u64_be | yes | 4 | 11.0 |
| `memory` | u64_be | yes | 5 | 11.0 |
| `read` | u64_be | yes | 6 | 11.0 |
| `write` | u64_be | yes | 7 | 11.0 |
| `duration-dbms` | u64_be | yes | 8 | 11.0 |
| `dbms-bytes` | u64_be | yes | 9 | 11.0 |
| `service` | u64_be | yes | 10 | 11.0 |
| `call` | u64_be | yes | 11 | 11.0 |
| `number-of-active-sessions` | u64_be | yes | 12 | 11.0 |
| `number-of-sessions` | u64_be | yes | 13 | 11.0 |
| `time` | datetime | yes | 14 | 16.0 |

### RPC

Request method: `0x82` (`counter values --cluster <id> --counter <name>`)
Response method: `0x83`

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

### Поля запроса (из `rac`)

Observed request parameters for `rac counter values`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `counter` | string | yes | 2 | 11.0 |
| `object` | string | yes (empty, `len=0`) | 3 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 4 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 5 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `object-len` | u8 | |
| `0x01` | `object-len` | `object` | string | UTF-8 |
| `base + 0x00` | `8` | `collection-time` | u64_be | |
| `base + 0x08` | `8` | `duration` | u64_be | |
| `base + 0x10` | `8` | `cpu-time` | u64_be | |
| `base + 0x18` | `8` | `memory` | u64_be | |
| `base + 0x20` | `8` | `read` | u64_be | |
| `base + 0x28` | `8` | `write` | u64_be | |
| `base + 0x30` | `8` | `duration-dbms` | u64_be | |
| `base + 0x38` | `8` | `dbms-bytes` | u64_be | |
| `base + 0x40` | `8` | `service` | u64_be | |
| `base + 0x48` | `8` | `call` | u64_be | |
| `base + 0x50` | `8` | `number-of-active-sessions` | u64_be | |
| `base + 0x58` | `8` | `number-of-sessions` | u64_be | |
| `base + 0x60` | `8` | `time` | datetime (u64_be) | 1C timestamp |

Notes:
- `base = 0x01 + object-len`.

### Hypotheses

- Record count > 1 should appear when multiple objects are returned.

### Open Questions

- Are there additional object filter types that affect record ordering?

### Gap Analysis (Required)

- No unknown byte regions remain in the record layout.

## Counter Accumulated Values

Source capture:
- `artifacts/rac/v11/v11_counter_accumulated_values_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_counter_accumulated_values_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11/v11_counter_accumulated_values_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_counter_accumulated_values_ro_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac counter accumulated-values` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `object` | string | yes | 1 | 11.0 |
| `collection-time` | u64_be | yes | 2 | 11.0 |
| `duration` | u64_be | yes | 3 | 11.0 |
| `cpu-time` | u64_be | yes | 4 | 11.0 |
| `memory` | u64_be | yes | 5 | 11.0 |
| `read` | u64_be | yes | 6 | 11.0 |
| `write` | u64_be | yes | 7 | 11.0 |
| `duration-dbms` | u64_be | yes | 8 | 11.0 |
| `dbms-bytes` | u64_be | yes | 9 | 11.0 |
| `service` | u64_be | yes | 10 | 11.0 |
| `call` | u64_be | yes | 11 | 11.0 |
| `number-of-active-sessions` | u64_be | yes | 12 | 11.0 |
| `number-of-sessions` | u64_be | yes | 13 | 11.0 |
| `time` | datetime | yes | 14 | 16.0 |

### RPC

Request method: `0x85` (`counter accumulated-values --cluster <id> --counter <name>`)
Response method: `0x86`

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

### Поля запроса (из `rac`)

Observed request parameters for `rac counter accumulated-values`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `counter` | string | yes | 2 | 11.0 |
| `object` | string | yes (empty, `len=0`) | 3 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 4 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 5 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `object-len` | u8 | |
| `0x01` | `object-len` | `object` | string | UTF-8 |
| `base + 0x00` | `8` | `collection-time` | u64_be | |
| `base + 0x08` | `8` | `duration` | u64_be | |
| `base + 0x10` | `8` | `cpu-time` | u64_be | |
| `base + 0x18` | `8` | `memory` | u64_be | |
| `base + 0x20` | `8` | `read` | u64_be | |
| `base + 0x28` | `8` | `write` | u64_be | |
| `base + 0x30` | `8` | `duration-dbms` | u64_be | |
| `base + 0x38` | `8` | `dbms-bytes` | u64_be | |
| `base + 0x40` | `8` | `service` | u64_be | |
| `base + 0x48` | `8` | `call` | u64_be | |
| `base + 0x50` | `8` | `number-of-active-sessions` | u64_be | |
| `base + 0x58` | `8` | `number-of-sessions` | u64_be | |
| `base + 0x60` | `8` | `time` | datetime (u64_be) | 1C timestamp |

Notes:
- `base = 0x01 + object-len`.

### Hypotheses

- Record count > 1 should appear when multiple objects are returned.

### Open Questions

- Are accumulated values aggregated across time windows or only since last clear?

### Gap Analysis (Required)

- No unknown byte regions remain in the record layout.

## Counter Clear

Source capture:
- `logs/session_1771346568_3508493_127_0_0_1_33842/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/counter_clear_codex_tmp_response.hex`

RAC output reference:
- `artifacts/rac/v16/counter_clear_codex_tmp_rac.out`

### Поля ответа (из `rac`)

`rac counter clear` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 16.0 |

### RPC

Request method: `0x84` (`counter clear --cluster <id> --counter <name>`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00`: `cluster:uuid16`
- offset `0x10`: `counter:str8`
- offset `0x10 + 1 + counter_len`: `object:str8` (empty in this capture)

### Поля запроса (из `rac`)

Observed request parameters for `rac counter clear`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `counter` | string | yes | 2 | 11.0 |
| `object` | string | yes (empty, `len=0`) | 3 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 4 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 5 | 11.0 |

### Hypotheses

- The ACK marker `0x01000000` likely encodes a success status; meaning needs confirmation.

### Open Questions

- Does `clear` ever return additional diagnostic fields for failed filters?

### Gap Analysis (Required)

- No unknown byte regions remain in the request payload layout.

## Counter Remove

Source capture:
- `logs/session_1771346572_3508551_127_0_0_1_33850/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/counter_remove_codex_tmp_response.hex`

RAC output reference:
- `artifacts/rac/v16/counter_remove_codex_tmp_rac.out`

### Поля ответа (из `rac`)

`rac counter remove` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 16.0 |

### RPC

Request method: `0x7b` (`counter remove --cluster <id> --name <name>`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00`: `cluster:uuid16`
- offset `0x10`: `name:str8`

### Поля запроса (из `rac`)

Observed request parameters for `rac counter remove`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 2 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 4 | 11.0 |

### Hypotheses

- The ACK marker `0x01000000` likely encodes a success status; meaning needs confirmation.

### Open Questions

- Is the ACK marker stable across remove of non-existent counters?

### Gap Analysis (Required)

- No unknown byte regions remain in the request payload layout.
