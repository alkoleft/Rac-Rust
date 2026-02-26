# RAC Limit Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11_help/limit_help.txt`
- `artifacts/rac/v11_help/limit_list.out`
- `artifacts/rac/v11_help/limit_info.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Limit List

Source capture:
- `artifacts/rac/v11_limit_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11_limit_list_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11_limit_list_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11_limit_list_ro_rac.out`
- `artifacts/rac/v11_help/limit_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac limit list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 11.0 |
| `counter` | string | yes | 2 | 11.0 |
| `action` | enum (u8) | yes | 3 | 11.0 |
| `duration` | u64_be | yes | 4 | 11.0 |
| `cpu-time` | u64_be | yes | 5 | 11.0 |
| `memory` | u64_be | yes | 6 | 11.0 |
| `read` | u64_be | yes | 7 | 11.0 |
| `write` | u64_be | yes | 8 | 11.0 |
| `duration-dbms` | u64_be | yes | 9 | 11.0 |
| `dbms-bytes` | u64_be | yes | 10 | 11.0 |
| `service` | u64_be | yes | 11 | 11.0 |
| `call` | u64_be | yes | 12 | 11.0 |
| `number-of-active-sessions` | u64_be | yes | 13 | 11.0 |
| `number-of-sessions` | u64_be | yes | 14 | 11.0 |
| `error-message` | string | yes | 15 | 11.0 |
| `descr` | string | yes | 16 | 11.0 |

### RPC

Request method: `0x7c` (`limit list --cluster <id>`)
Response method: `0x7d`

Payload structure (method body):
- offset `0x00`: `16 <cluster_uuid_16b>`
- offset `0x00` (response): `items_count:u8`

### Поля запроса (из `rac`)

Observed request parameters for `rac limit list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes | 2 | 11.0 |
| `cluster-pwd` | string | yes | 3 | 11.0 |

Notes:
- `cluster-user`/`cluster-pwd` are sent via the context setter (`rpc_method_id=0x09`) before the `limit list` request. Order in that context payload: `cluster`, `cluster-user`, `cluster-pwd`.

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `name-len` | u8 | |
| `0x01` | `name-len` | `name` | string | UTF-8 |
| `0x01 + name-len` | `1` | `counter-len` | u8 | |
| `0x02 + name-len` | `counter-len` | `counter` | string | UTF-8 |
| `0x02 + name-len + counter-len` | `1` | `action` | enum (u8) | |
| `base + 0x00` | `8` | `duration` | u64_be | |
| `base + 0x08` | `8` | `cpu-time` | u64_be | |
| `base + 0x10` | `8` | `memory` | u64_be | |
| `base + 0x18` | `8` | `read` | u64_be | |
| `base + 0x20` | `8` | `write` | u64_be | |
| `base + 0x28` | `8` | `duration-dbms` | u64_be | |
| `base + 0x30` | `8` | `dbms-bytes` | u64_be | |
| `base + 0x38` | `8` | `service` | u64_be | |
| `base + 0x40` | `8` | `call` | u64_be | |
| `base + 0x48` | `8` | `number-of-active-sessions` | u64_be | |
| `base + 0x50` | `8` | `number-of-sessions` | u64_be | |
| `base + 0x58` | `1` | `error-message-len` | u8 | |
| `base + 0x59` | `error-message-len` | `error-message` | string | UTF-8 |
| `base + 0x59 + error-message-len` | `1` | `descr-len` | u8 | |
| `base + 0x5a + error-message-len` | `descr-len` | `descr` | string | UTF-8 |

Notes:
- `base = 0x03 + name-len + counter-len`.

### Hypotheses

- Action enum values are inferred from two captures: `interrupt-current-call` => `0x02`, `interrupt-session` => `0x03`.

### Open Questions

- Is `items_count` encoded as `u8` or varuint for larger lists?
- Are any of the u64 fields optional/nullable when the limit is not configured?

### Gap Analysis (Required)

- No unknown byte regions remain in the list record layout. Confirm whether `items_count` switches to varuint for larger lists.

## Limit Info

Source capture:
- `artifacts/rac/v11_limit_info_ro_client_to_server.decode.txt`
- `artifacts/rac/v11_limit_info_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11_limit_info_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11_limit_info_ro_rac.out`
- `artifacts/rac/v11_help/limit_info.out`

### Поля ответа (из `rac`)

Observed field names in `rac limit info` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 11.0 |
| `counter` | string | yes | 2 | 11.0 |
| `action` | enum (u8) | yes | 3 | 11.0 |
| `duration` | u64_be | yes | 4 | 11.0 |
| `cpu-time` | u64_be | yes | 5 | 11.0 |
| `memory` | u64_be | yes | 6 | 11.0 |
| `read` | u64_be | yes | 7 | 11.0 |
| `write` | u64_be | yes | 8 | 11.0 |
| `duration-dbms` | u64_be | yes | 9 | 11.0 |
| `dbms-bytes` | u64_be | yes | 10 | 11.0 |
| `service` | u64_be | yes | 11 | 11.0 |
| `call` | u64_be | yes | 12 | 11.0 |
| `number-of-active-sessions` | u64_be | yes | 13 | 11.0 |
| `number-of-sessions` | u64_be | yes | 14 | 11.0 |
| `error-message` | string | yes | 15 | 11.0 |
| `descr` | string | yes | 16 | 11.0 |

### RPC

Request method: `0x7e` (`limit info --cluster <id> --limit <name>`)
Response method: `0x7f`

Payload structure (method body):
- offset `0x00`: `16 <cluster_uuid_16b>`
- offset `0x10`: `limit_name:str8`

### Поля запроса (из `rac`)

Observed request parameters for `rac limit info`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `limit` | string | yes | 2 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 4 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `name-len` | u8 | |
| `0x01` | `name-len` | `name` | string | UTF-8 |
| `0x01 + name-len` | `1` | `counter-len` | u8 | |
| `0x02 + name-len` | `counter-len` | `counter` | string | UTF-8 |
| `0x02 + name-len + counter-len` | `1` | `action` | enum (u8) | |
| `base + 0x00` | `8` | `duration` | u64_be | |
| `base + 0x08` | `8` | `cpu-time` | u64_be | |
| `base + 0x10` | `8` | `memory` | u64_be | |
| `base + 0x18` | `8` | `read` | u64_be | |
| `base + 0x20` | `8` | `write` | u64_be | |
| `base + 0x28` | `8` | `duration-dbms` | u64_be | |
| `base + 0x30` | `8` | `dbms-bytes` | u64_be | |
| `base + 0x38` | `8` | `service` | u64_be | |
| `base + 0x40` | `8` | `call` | u64_be | |
| `base + 0x48` | `8` | `number-of-active-sessions` | u64_be | |
| `base + 0x50` | `8` | `number-of-sessions` | u64_be | |
| `base + 0x58` | `1` | `error-message-len` | u8 | |
| `base + 0x59` | `error-message-len` | `error-message` | string | UTF-8 |
| `base + 0x59 + error-message-len` | `1` | `descr-len` | u8 | |
| `base + 0x5a + error-message-len` | `descr-len` | `descr` | string | UTF-8 |

Notes:
- `base = 0x03 + name-len + counter-len`.

### Hypotheses

- Action enum values are inferred from a single capture: `interrupt-current-call` => `0x02`.

### Open Questions

- Are all numeric thresholds always present as `u64_be`, or can any be omitted/shortened when unset?

### Gap Analysis (Required)

- No unknown byte regions remain in the response record layout. Confirm action enum mapping by capturing other `--action` values.

## Limit Update

Source capture:
- `logs/session_1771357031_3632890_127_0_0_1_54092/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/limit_update_response.hex`

RAC output reference:
- `artifacts/rac/limit_update_rac.out`

### Поля ответа (из `rac`)

`rac limit update` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 16.0 |

### RPC

Request method: `0x80` (`limit update --cluster <id> --name <name> ...`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00`: `16 <cluster_uuid_16b>`
- offset `0x10`: `limit record` (same layout as `limit info` response)

### Поля запроса (из `rac`)

Observed request parameters for `rac limit update`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 2 | 11.0 |
| `counter` | string | yes | 3 | 11.0 |
| `action` | enum (u8) | yes | 4 | 11.0 |
| `duration` | u64_be | yes | 5 | 11.0 |
| `cpu-time` | u64_be | yes | 6 | 11.0 |
| `memory` | u64_be | yes | 7 | 11.0 |
| `read` | u64_be | yes | 8 | 11.0 |
| `write` | u64_be | yes | 9 | 11.0 |
| `duration-dbms` | u64_be | yes | 10 | 11.0 |
| `dbms-bytes` | u64_be | yes | 11 | 11.0 |
| `service` | u64_be | yes | 12 | 11.0 |
| `call` | u64_be | yes | 13 | 11.0 |
| `number-of-active-sessions` | u64_be | yes | 14 | 11.0 |
| `number-of-sessions` | u64_be | yes | 15 | 11.0 |
| `error-message` | string | yes | 16 | 11.0 |
| `descr` | string | yes | 17 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 18 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 19 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the request body record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `name-len` | u8 | |
| `0x01` | `name-len` | `name` | string | UTF-8 |
| `0x01 + name-len` | `1` | `counter-len` | u8 | |
| `0x02 + name-len` | `counter-len` | `counter` | string | UTF-8 |
| `0x02 + name-len + counter-len` | `1` | `action` | enum (u8) | |
| `base + 0x00` | `8` | `duration` | u64_be | |
| `base + 0x08` | `8` | `cpu-time` | u64_be | |
| `base + 0x10` | `8` | `memory` | u64_be | |
| `base + 0x18` | `8` | `read` | u64_be | |
| `base + 0x20` | `8` | `write` | u64_be | |
| `base + 0x28` | `8` | `duration-dbms` | u64_be | |
| `base + 0x30` | `8` | `dbms-bytes` | u64_be | |
| `base + 0x38` | `8` | `service` | u64_be | |
| `base + 0x40` | `8` | `call` | u64_be | |
| `base + 0x48` | `8` | `number-of-active-sessions` | u64_be | |
| `base + 0x50` | `8` | `number-of-sessions` | u64_be | |
| `base + 0x58` | `1` | `error-message-len` | u8 | |
| `base + 0x59` | `error-message-len` | `error-message` | string | UTF-8 |
| `base + 0x59 + error-message-len` | `1` | `descr-len` | u8 | |
| `base + 0x5a + error-message-len` | `descr-len` | `descr` | string | UTF-8 |

Notes:
- `base = 0x03 + name-len + counter-len`.

### Hypotheses

- Action enum values are inferred from a single capture: `interrupt-current-call` => `0x02`.

### Open Questions

- Do servers accept update payloads that omit some threshold fields, or must all u64 slots be present?

### Gap Analysis (Required)

- No unknown byte regions remain in the request payload layout.

## Limit Remove

Source capture:
- `logs/session_1771357081_3633977_127_0_0_1_49364/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/limit_remove_response.hex`

RAC output reference:
- `artifacts/rac/limit_remove_rac.out`

### Поля ответа (из `rac`)

`rac limit remove` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 16.0 |

### RPC

Request method: `0x81` (`limit remove --cluster <id> --name <name>`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00`: `16 <cluster_uuid_16b>`
- offset `0x10`: `name:str8`

### Поля запроса (из `rac`)

Observed request parameters for `rac limit remove`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 2 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 4 | 11.0 |

### Hypotheses

- The ACK marker `0x01000000` likely encodes success status; meaning needs confirmation.

### Open Questions

- Is the ACK marker stable for remove of non-existent limits?

### Gap Analysis (Required)

- No unknown byte regions remain in the request payload layout.
