# RAC Manager Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v16):
- `artifacts/rac/v16/help/manager_help.txt`
- `artifacts/rac/v16/help/manager_list.out`
- `artifacts/rac/v16/help/manager_info.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Manager List

Source capture:
- `logs/session_1771287345_2785336_127_0_0_1_47884/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/manager_list_response.hex`

RAC output reference:
- `artifacts/rac/v16/manager_list_rac.out`
- `artifacts/rac/v16/help/manager_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac manager list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `manager` | UUID | yes | 1 | 16.0 |
| `descr` | string | yes | 2 | 16.0 |
| `host` | string | yes | 3 | 16.0 |
| `using` | enum (u32) | yes | 4 | 16.0 |
| `port` | u16 | yes | 5 | 16.0 |
| `pid` | string (digits) | yes | 6 | 16.0 |

### RPC

Request method: `0x12` (`manager list --cluster <id>`)
Response method: `0x13`

### Поля запроса (из `rac`)

Observed request parameters for `rac manager list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 16.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 16.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 16.0 |

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x01`)
- offset `0x01`: first record starts here

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `manager` | UUID | |
| `0x10` | `1` | `descr_len` | u8 | |
| `0x11` | `descr_len` | `descr` | string | UTF-8, observed `Главный менеджер кластера` |
| `0x11 + descr_len` | `1` | `host_len` | u8 | |
| `0x12 + descr_len` | `host_len` | `host` | string | UTF-8, observed `alko-home` |
| `0x12 + descr_len + host_len` | `4` | `using` | u32_be | observed `0x00000001` -> `main` |
| `0x16 + descr_len + host_len` | `2` | `port` | u16_be | observed `0x0605` -> 1541 |
| `0x18 + descr_len + host_len` | `1` | `pid_len` | u8 | |
| `0x19 + descr_len + host_len` | `pid_len` | `pid` | string | ASCII digits, observed `314037` |

Notes:
- Gaps: none observed in this record layout.

## Manager Info

Source capture:
- `logs/session_1771287351_2785436_127_0_0_1_40168/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v16/manager_info_response.hex`

RAC output reference:
- `artifacts/rac/v16/manager_info_rac.out`
- `artifacts/rac/v16/help/manager_info.out`

### Поля ответа (из `rac`)

Same field set as `manager list` (see above).

### RPC

Request method: `0x14` (`manager info --cluster <id> --manager <id>`)
Response method: `0x15`

### Поля запроса (из `rac`)

Observed request parameters for `rac manager info`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 16.0 |
| `manager` | UUID | yes | 2 | 16.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 3 | 16.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 4 | 16.0 |

Payload structure (method body):
- single record in the same layout as `manager list` (no leading count byte)

## Hypotheses

- None at this time.

## Open Questions

- Confirm `using` enum values beyond `main` (observed `0x00000001`).
