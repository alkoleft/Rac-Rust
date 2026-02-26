# RAC Rule Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11/help/rule_help.txt`
- `artifacts/rac/v11/help/rule_list.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Rule Apply

Source capture:
- `logs/session_1771359945_3673197_127_0_0_1_36476/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/rule_apply_response.hex`

RAC output reference:
- `artifacts/rac/rule_apply_rac.out`

### Поля ответа (из `rac`)

`rac rule apply` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 11.0 |

### RPC

Request method: `0x51` (`rule apply --cluster <id> [--full|--partial]`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `apply_mode` (u32_be, `1` for `--full`)

### Поля запроса (из `rac`)

Observed request parameters for `rac rule apply`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x51`) | 1 | 11.0 |
| `full/partial` | enum (u32_be) | yes (request `0x51`) | 2 | 11.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 3 | 11.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 4 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the request body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `cluster` | UUID | |
| `0x10` | `4` | `apply_mode` | u32_be | `1` = `--full`, capture only |

### Hypotheses

- `apply_mode` is an enum where `0` might mean `--partial`.

### Open Questions

- Confirm `--partial` encoding by capturing `apply --partial`.

### Gap Analysis (Required)

- Capture `rule apply --partial` to verify the enum value.

## Rule List

Source capture:
- `artifacts/rac/v11/v11_rule_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_rule_list_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11/v11_rule_list_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_rule_list_ro_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac rule list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `rule` | UUID | yes | 1 | 11.0 |
| `object-type` | u32_be | yes | 2 | 11.0 |
| `infobase-name` | string | yes | 3 | 11.0 |
| `rule-type` | enum (u8) | yes | 4 | 11.0 |
| `application-ext` | string | yes | 5 | 11.0 |
| `priority` | u32_be | yes | 6 | 11.0 |

### RPC

Request method: `0x55` (`rule list --cluster <id> --server <id>`)
Response method: `0x56`

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x00` (response): `items_count:u8`

### Поля запроса (из `rac`)

Observed request parameters for `rac rule list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x55`) | 1 | 11.0 |
| `server` | UUID | yes (request `0x55`) | 2 | 11.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 3 | 11.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 4 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `rule` | UUID | |
| `0x10` | `4` | `object-type` | u32_be | |
| `0x14` | `1` | `infobase-name-len` | u8 | |
| `0x15` | `infobase-name-len` | `infobase-name` | string | UTF-8 |
| `0x15 + infobase-name-len` | `1` | `rule-type` | enum (u8) | `0x01` = `auto` (capture) |
| `0x16 + infobase-name-len` | `1` | `application-ext-len` | u8 | |
| `0x17 + infobase-name-len` | `application-ext-len` | `application-ext` | string | UTF-8 |
| `0x17 + infobase-name-len + application-ext-len` | `4` | `priority` | u32_be | |

### Hypotheses

- `rule-type` enum: `0x01` = `auto`; other values map to `always/never`.

### Open Questions

- Does `object-type` use numeric IDs beyond `0`? Which object types map to which values?

### Gap Analysis (Required)

- Capture rules with non-empty `infobase-name`, `application-ext`, and non-zero `priority` to confirm field offsets and lengths.

## Rule Info

Source capture:
- `logs/session_1771359933_3672979_127_0_0_1_35118/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/rule_info_response.hex`

RAC output reference:
- `artifacts/rac/rule_info_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac rule info` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `rule` | UUID | yes | 1 | 11.0 |
| `object-type` | u32_be | yes | 2 | 11.0 |
| `infobase-name` | string | yes | 3 | 11.0 |
| `rule-type` | enum (u8) | yes | 4 | 11.0 |
| `application-ext` | string | yes | 5 | 11.0 |
| `priority` | u32_be | yes | 6 | 11.0 |

### RPC

Request method: `0x57` (`rule info --cluster <id> --server <id> --rule <id>`)
Response method: `0x58`

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `rule_uuid` (16 bytes)

### Поля запроса (из `rac`)

Observed request parameters for `rac rule info`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x57`) | 1 | 11.0 |
| `server` | UUID | yes (request `0x57`) | 2 | 11.0 |
| `rule` | UUID | yes (request `0x57`) | 3 | 11.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 4 | 11.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 5 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `rule` | UUID | |
| `0x10` | `4` | `object-type` | u32_be | |
| `0x14` | `1` | `infobase-name-len` | u8 | |
| `0x15` | `infobase-name-len` | `infobase-name` | string | UTF-8 |
| `0x15 + infobase-name-len` | `1` | `rule-type` | enum (u8) | `0x01` = `auto` (capture) |
| `0x16 + infobase-name-len` | `1` | `application-ext-len` | u8 | |
| `0x17 + infobase-name-len` | `application-ext-len` | `application-ext` | string | UTF-8 |
| `0x17 + infobase-name-len + application-ext-len` | `4` | `priority` | u32_be | |

### Hypotheses

- `rule-type` enum mapping matches `rac` CLI options: `auto|always|never`.

### Open Questions

- Is `object-type` `0` used for "all objects"? Validate with a non-zero `--object-type`.

### Gap Analysis (Required)

- Capture info for a rule with `rule-type=always` and `priority>0` to confirm enum and numeric encodings.

## Rule Insert

Source capture:
- `logs/session_1771359718_3669811_127_0_0_1_50010/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/rule_insert_response.hex`

RAC output reference:
- `artifacts/rac/rule_insert_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac rule insert` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `rule` | UUID | yes | 1 | 11.0 |

### RPC

Request method: `0x52` (`rule insert --cluster <id> --server <id> ...`)
Response method: `0x53`

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `rule_uuid` (16 bytes, all-zero for insert)
- offset `0x30` (request): `position:u32_be`
- offset `0x34` (request): `object-type:u32_be`
- offset `0x38` (request): `infobase-name:str8`
- offset `0x38 + infobase-name-len` (request): `rule-type:u8`
- offset `0x39 + infobase-name-len` (request): `application-ext:str8`
- offset `...` (request): `priority:u32_be`
- offset `0x00` (response): `rule_uuid` (16 bytes)

### Поля запроса (из `rac`)

Observed request parameters for `rac rule insert`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x52`) | 1 | 11.0 |
| `server` | UUID | yes (request `0x52`) | 2 | 11.0 |
| `position` | u32_be | yes (request `0x52`) | 3 | 11.0 |
| `object-type` | u32_be | yes (request `0x52`) | 4 | 11.0 |
| `infobase-name` | string | yes (request `0x52`) | 5 | 11.0 |
| `rule-type` | enum (u8) | yes (request `0x52`) | 6 | 11.0 |
| `application-ext` | string | yes (request `0x52`) | 7 | 11.0 |
| `priority` | u32_be | yes (request `0x52`) | 8 | 11.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 9 | 11.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 10 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the response body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `rule` | UUID | |

### Hypotheses

- Insert and update share the same RPC method `0x52`; update supplies a non-zero `rule_uuid`.

### Open Questions

- Is `rule-type` encoded as `0x00` or `0x01` for `auto` in requests? Needs a non-default capture.

### Gap Analysis (Required)

- Capture an insert with non-zero `priority` and non-empty `infobase-name` to validate request offsets.

## Rule Update

Source capture:
- `logs/session_1771359941_3673129_127_0_0_1_36460/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/rule_update_response.hex`

RAC output reference:
- `artifacts/rac/rule_update_rac.out`

### Поля ответа (из `rac`)

`rac rule update` produces no output; response includes a rule UUID payload.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `rule` | UUID | yes | 1 | 11.0 |

### RPC

Request method: `0x52` (update) and an internal `info` fetch `0x57` (observed before update call)
Response method: `0x53` (update) and `0x58` (info response)

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `rule_uuid` (16 bytes, non-zero)
- offset `0x30` (request): `position:u32_be`
- offset `0x34` (request): `object-type:u32_be`
- offset `0x38` (request): `infobase-name:str8`
- offset `0x38 + infobase-name-len` (request): `rule-type:u8`
- offset `0x39 + infobase-name-len` (request): `application-ext:str8`
- offset `...` (request): `priority:u32_be`
- offset `0x00` (response): `rule_uuid` (16 bytes)

### Поля запроса (из `rac`)

Observed request parameters for `rac rule update`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x52`) | 1 | 11.0 |
| `server` | UUID | yes (request `0x52`) | 2 | 11.0 |
| `rule` | UUID | yes (request `0x52`) | 3 | 11.0 |
| `position` | u32_be | yes (request `0x52`) | 4 | 11.0 |
| `object-type` | u32_be | yes (request `0x52`) | 5 | 11.0 |
| `infobase-name` | string | yes (request `0x52`) | 6 | 11.0 |
| `rule-type` | enum (u8) | yes (request `0x52`) | 7 | 11.0 |
| `application-ext` | string | yes (request `0x52`) | 8 | 11.0 |
| `priority` | u32_be | yes (request `0x52`) | 9 | 11.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 10 | 11.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 11 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the response body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `rule` | UUID | |

### Hypotheses

- `rac` performs a preliminary `rule info` call (`0x57/0x58`) before issuing update `0x52`.

### Open Questions

- Is update always done via `0x52`, or does it switch for other rule types?

### Gap Analysis (Required)

- Capture update with non-default fields to validate request layout.

## Rule Remove

Source capture:
- `logs/session_1771359950_3673267_127_0_0_1_56154/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/rule_remove_response.hex`

RAC output reference:
- `artifacts/rac/rule_remove_rac.out`

### Поля ответа (из `rac`)

`rac rule remove` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 11.0 |

### RPC

Request method: `0x54` (`rule remove --cluster <id> --server <id> --rule <id>`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `rule_uuid` (16 bytes)

### Поля запроса (из `rac`)

Observed request parameters for `rac rule remove`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x54`) | 1 | 11.0 |
| `server` | UUID | yes (request `0x54`) | 2 | 11.0 |
| `rule` | UUID | yes (request `0x54`) | 3 | 11.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 4 | 11.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 5 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of the request body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `cluster` | UUID | |
| `0x10` | `16` | `server` | UUID | |
| `0x20` | `16` | `rule` | UUID | |

### Hypotheses

- ACK value `0x01000000` is a generic success marker for rule remove/apply.

### Open Questions

- Are there error responses for remove (non-ACK frames)?

### Gap Analysis (Required)

- Capture a failing `rule remove` to characterize error payloads.
