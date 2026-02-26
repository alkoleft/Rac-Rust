# RAC Service Setting Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Note:
- The `service-setting` mode does not appear in `rac` v11 help; commands below are **not available in v11** and are documented from v16.0 captures.

## Service Setting List

Source capture:
- `logs/session_1771358420_3653062_127_0_0_1_55942/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/service_setting_list_nonempty_response.hex`

RAC output reference:
- `artifacts/rac/v16/service_setting_list_nonempty_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac service-setting list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `setting` | UUID | yes | 1 | 16.0 |
| `service-name` | string | yes | 2 | 16.0 |
| `infobase-name` | string | yes (empty string, len=0) | 3 | 16.0 |
| `service-data-dir` | string | yes | 4 | 16.0 |
| `active` | bool (u16?) | yes | 5 | 16.0 |

### RPC

Request method: `0x8b` (`service-setting list --cluster <id> --server <id>`)
Response method: `0x8c`

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x00` (response): `items_count:u8`

### Поля запроса (из `rac`)

Observed request parameters for `rac service-setting list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x8b`) | 1 | 16.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 2 | 16.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 3 | 16.0 |
| `server` | UUID | yes (request `0x8b`) | 4 | 16.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `setting` | UUID | |
| `0x10` | `1` | `service-name-len` | u8 | |
| `0x11` | `service-name-len` | `service-name` | string | UTF-8, observed `EventLogService` |
| `0x11 + service-name-len` | `1` | `infobase-name-len` | u8 | |
| `0x12 + service-name-len` | `infobase-name-len` | `infobase-name` | string | empty if len=0 |
| `0x12 + service-name-len + infobase-name-len` | `1` | `service-data-dir-len` | u8 | |
| `0x13 + service-name-len + infobase-name-len` | `service-data-dir-len` | `service-data-dir` | string | observed `/tmp/codex_service_setting/` |
| `0x13 + service-name-len + infobase-name-len + service-data-dir-len` | `2` | `active` | u16 | observed `0x0000` -> `no` |

## Service Setting Info

Source capture:
- `logs/session_1771358415_3652970_127_0_0_1_41106/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/service_setting_info_response.hex`

RAC output reference:
- `artifacts/rac/v16/service_setting_info_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac service-setting info` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `setting` | UUID | yes | 1 | 16.0 |
| `service-name` | string | yes | 2 | 16.0 |
| `infobase-name` | string | yes (empty string, len=0) | 3 | 16.0 |
| `service-data-dir` | string | yes | 4 | 16.0 |
| `active` | bool (u16?) | yes | 5 | 16.0 |

### RPC

Request method: `0x89` (`service-setting info --cluster <id> --server <id> --setting <id>`)
Response method: `0x8a`

Payload structure (method body):
- single record in the same layout as `service-setting list` (no leading count byte)

### Поля запроса (из `rac`)

Observed request parameters for `rac service-setting info`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x89`) | 1 | 16.0 |
| `server` | UUID | yes (request `0x89`) | 2 | 16.0 |
| `setting` | UUID | yes (request `0x89`) | 3 | 16.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 4 | 16.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 5 | 16.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `setting` | UUID | |
| `0x10` | `1` | `service-name-len` | u8 | |
| `0x11` | `service-name-len` | `service-name` | string | UTF-8, observed `EventLogService` |
| `0x11 + service-name-len` | `1` | `infobase-name-len` | u8 | |
| `0x12 + service-name-len` | `infobase-name-len` | `infobase-name` | string | empty if len=0 |
| `0x12 + service-name-len + infobase-name-len` | `1` | `service-data-dir-len` | u8 | |
| `0x13 + service-name-len + infobase-name-len` | `service-data-dir-len` | `service-data-dir` | string | observed `/tmp/codex_service_setting/` |
| `0x13 + service-name-len + infobase-name-len + service-data-dir-len` | `2` | `active` | u16 | observed `0x0000` -> `no` |

## Service Setting Insert

Source capture:
- `logs/session_1771358318_3651273_127_0_0_1_50978/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/service_setting_insert_response.hex`

RAC output reference:
- `artifacts/rac/v16/service_setting_insert_rac.out`

### Поля ответа (из `rac`)

`rac service-setting insert` returns the created setting UUID.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `setting` | UUID | yes | 1 | 16.0 |

### RPC

Request method: `0x8d` (`service-setting insert --cluster <id> --server <id> --service-name <name> [--infobase-name <name>] [--service-data-dir <dir>]`)
Response method: `0x8e`

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `setting record` (see Record Layout)
- offset `0x00` (response): `setting_uuid` (16 bytes)

### Поля запроса (из `rac`)

Observed request parameters for `rac service-setting insert`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 16.0 |
| `server` | UUID | yes | 2 | 16.0 |
| `setting` | UUID | yes (zeroed for insert) | 3 | 16.0 |
| `service-name` | string | yes | 4 | 16.0 |
| `infobase-name` | string | yes (empty string, len=0) | 5 | 16.0 |
| `service-data-dir` | string | yes | 6 | 16.0 |
| `active` | bool (u16?) | yes (default `0x0000`) | 7 | 16.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 8 | 16.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 9 | 16.0 |

### Record Layout (Observed)

Offsets are relative to the start of the request body record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `setting` | UUID | all zeros for insert |
| `0x10` | `1` | `service-name-len` | u8 | |
| `0x11` | `service-name-len` | `service-name` | string | UTF-8, observed `EventLogService` |
| `0x11 + service-name-len` | `1` | `infobase-name-len` | u8 | |
| `0x12 + service-name-len` | `infobase-name-len` | `infobase-name` | string | empty if len=0 |
| `0x12 + service-name-len + infobase-name-len` | `1` | `service-data-dir-len` | u8 | |
| `0x13 + service-name-len + infobase-name-len` | `service-data-dir-len` | `service-data-dir` | string | observed `/tmp/codex_service_setting` |
| `0x13 + service-name-len + infobase-name-len + service-data-dir-len` | `2` | `active` | u16 | observed `0x0000` |

## Service Setting Update

Source capture:
- `logs/session_1771358432_3653295_127_0_0_1_58326/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/service_setting_update_response.hex`

RAC output reference:
- `artifacts/rac/v16/service_setting_update_rac.out`

### Поля ответа (из `rac`)

`rac service-setting update` produces no output; the response payload contains the setting UUID.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `setting` | UUID | yes | 1 | 16.0 |

### RPC

Request method: `0x8d` (`service-setting update --cluster <id> --server <id> --setting <id> [--service-data-dir <dir>]`)
Response method: `0x8e`

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `setting record` (see Record Layout)
- offset `0x00` (response): `setting_uuid` (16 bytes)

Notes:
- `rac` issues a preliminary `service-setting info` request (`0x89`/`0x8a`) before sending the update payload.

### Поля запроса (из `rac`)

Observed request parameters for `rac service-setting update`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 16.0 |
| `server` | UUID | yes | 2 | 16.0 |
| `setting` | UUID | yes (non-zero) | 3 | 16.0 |
| `service-name` | string | yes | 4 | 16.0 |
| `infobase-name` | string | yes (empty string, len=0) | 5 | 16.0 |
| `service-data-dir` | string | yes | 6 | 16.0 |
| `active` | bool (u16?) | yes (default `0x0000`) | 7 | 16.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 8 | 16.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 9 | 16.0 |

### Record Layout (Observed)

Offsets are relative to the start of the request body record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `setting` | UUID | non-zero for update |
| `0x10` | `1` | `service-name-len` | u8 | |
| `0x11` | `service-name-len` | `service-name` | string | UTF-8, observed `EventLogService` |
| `0x11 + service-name-len` | `1` | `infobase-name-len` | u8 | |
| `0x12 + service-name-len` | `infobase-name-len` | `infobase-name` | string | empty if len=0 |
| `0x12 + service-name-len + infobase-name-len` | `1` | `service-data-dir-len` | u8 | |
| `0x13 + service-name-len + infobase-name-len` | `service-data-dir-len` | `service-data-dir` | string | observed `/tmp/codex_service_setting_updated` |
| `0x13 + service-name-len + infobase-name-len + service-data-dir-len` | `2` | `active` | u16 | observed `0x0000` |

## Service Setting Get Service Data Dirs For Transfer

Source capture:
- `logs/session_1771358425_3653158_127_0_0_1_55954/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/service_setting_get_data_dirs_response.hex`

RAC output reference:
- `artifacts/rac/v16/service_setting_get_data_dirs_rac.out`

### Поля ответа (из `rac`)

Observed field names in `rac service-setting get-service-data-dirs-for-transfer` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `service-name` | string | yes | 1 | 16.0 |
| `user` | string | yes | 2 | 16.0 |
| `source-dir` | string | yes | 3 | 16.0 |
| `target-dir` | string | yes | 4 | 16.0 |

### RPC

Request method: `0x91` (`service-setting get-service-data-dirs-for-transfer --cluster <id> --server <id> [--service-name <name>]`)
Response method: `0x92`

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `service-name-len + service-name` (optional)
- offset `0x00` (response): `items_count:u8`

### Поля запроса (из `rac`)

Observed request parameters for `rac service-setting get-service-data-dirs-for-transfer`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x91`) | 1 | 16.0 |
| `server` | UUID | yes (request `0x91`) | 2 | 16.0 |
| `service-name` | string | yes | 3 | 16.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 4 | 16.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 5 | 16.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `1` | `service-name-len` | u8 | |
| `0x01` | `service-name-len` | `service-name` | string | UTF-8, observed `EventLogService` |
| `0x01 + service-name-len` | `1` | `user-len` | u8 | |
| `0x02 + service-name-len` | `user-len` | `user` | string | observed `yaxunit` |
| `0x02 + service-name-len + user-len` | `1` | `source-dir-len` | u8 | |
| `0x03 + service-name-len + user-len` | `1` | `source-dir-flag` | u8 | observed `0x01` |
| `0x04 + service-name-len + user-len` | `source-dir-len` | `source-dir` | string | observed `/tmp/1cv8-agent/.../1Cv8Log` |
| `0x04 + service-name-len + user-len + source-dir-len` | `1` | `target-dir-len` | u8 | |
| `0x05 + service-name-len + user-len + source-dir-len` | `1` | `target-dir-flag` | u8 | observed `0x01` |
| `0x06 + service-name-len + user-len + source-dir-len` | `target-dir-len` | `target-dir` | string | observed `/tmp/codex_service_setting/.../1Cv8Log` |

## Service Setting Remove

Source capture:
- `logs/session_1771358460_3653727_127_0_0_1_49386/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/service_setting_remove_response.hex`

RAC output reference:
- `artifacts/rac/v16/service_setting_remove_rac.out`

### Поля ответа (из `rac`)

`rac service-setting remove` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 16.0 |

### RPC

Request method: `0x8f` (`service-setting remove --cluster <id> --server <id> --setting <id>`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)
- offset `0x20` (request): `setting_uuid` (16 bytes)

### Поля запроса (из `rac`)

Observed request parameters for `rac service-setting remove`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x8f`) | 1 | 16.0 |
| `server` | UUID | yes (request `0x8f`) | 2 | 16.0 |
| `setting` | UUID | yes (request `0x8f`) | 3 | 16.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 4 | 16.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 5 | 16.0 |

### Record Layout (Observed)

Offsets are relative to the start of the request body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `cluster` | UUID | |
| `0x10` | `16` | `server` | UUID | |
| `0x20` | `16` | `setting` | UUID | |

## Service Setting Apply

Source capture:
- `logs/session_1771358454_3653578_127_0_0_1_58378/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/service_setting_apply_response.hex`

RAC output reference:
- `artifacts/rac/v16/service_setting_apply_rac.out`

### Поля ответа (из `rac`)

`rac service-setting apply` produces no output. The response payload is an ACK-only block.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `ack` | u32_be | yes | 1 | 16.0 |

### RPC

Request method: `0x90` (`service-setting apply --cluster <id> --server <id>`)
Response: ACK-only (`01000000`), no RPC method id in the response frame.

Payload structure (method body):
- offset `0x00` (request): `cluster_uuid` (16 bytes)
- offset `0x10` (request): `server_uuid` (16 bytes)

### Поля запроса (из `rac`)

Observed request parameters for `rac service-setting apply`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes (auth/context `0x09`, request `0x90`) | 1 | 16.0 |
| `server` | UUID | yes (request `0x90`) | 2 | 16.0 |
| `cluster-user` | string | yes (auth/context `0x09`) | 3 | 16.0 |
| `cluster-pwd` | string | yes (auth/context `0x09`) | 4 | 16.0 |

### Record Layout (Observed)

Offsets are relative to the start of the request body.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `cluster` | UUID | |
| `0x10` | `16` | `server` | UUID | |

## Hypotheses

- `active` is a 2-byte boolean/enum (`0x0000` = inactive); verify if it becomes non-zero after `apply`.
- `source-dir-flag` and `target-dir-flag` are presence flags (both observed `0x01`).

## Open Questions

- Does `active` ever encode additional states beyond `0/1`, or is it strictly boolean?
- Are `source-dir-flag`/`target-dir-flag` always `0x01`, or do they encode a directory type?
- Does providing `--infobase-name` change the record layout or insert a new field in the setting record?

## Gap Analysis (Required)

- Capture `service-setting list/info` after a setting becomes active to confirm the `active` encoding.
- Capture `get-service-data-dirs-for-transfer` with a service that has no transfer directory to see if flags become `0x00` or lengths become `0`.
- Capture `service-setting insert` with `--infobase-name` to confirm how `infobase-name` is encoded when non-empty.
