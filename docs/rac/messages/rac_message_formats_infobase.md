# RAC Message Formats: Infobase API

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11/help/infobase_help.txt`
- `artifacts/rac/v11/help/infobase_list.out`
- `artifacts/rac/v11/help/infobase_info.out`
- `artifacts/rac/v11/help/infobase_summary_list.out`
- `artifacts/rac/v11/help/infobase_summary_info.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

Derived from `docs/rac/messages/rac_message_formats.md`.

## Commands

Command set for protocol version `11.0` (observed via `rac` help/output):
- `infobase summary list`
- `infobase summary info`
- `infobase summary update`
- `infobase info`
- `infobase create`
- `infobase update`
- `infobase drop`

Note:
- `infobase list` is not available in v11 (command prints help instead).

### Infobase Summary List

- **v11 output reference**: `artifacts/rac/v11/help/infobase_summary_list.out`
- **Request**: `0x09` (context), then method `0x2a`.
- **Response**: method `0x2b`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response body layout** (after `01 00 00 01 2b`):
  - `u8 count` (observed `0x01`).
  - Repeated record:
    - `uuid[16]` (raw bytes).
    - `u8 tag` (observed `0x2c`, meaning unknown).
    - `str8 descr` (observed `Description`).
    - `str8 name` (observed `yaxunit`).
- **Evidence**: `artifacts/rac/v11/v11_infobase_summary_list_ro_client_to_server.decode.txt`, `artifacts/rac/v11/v11_infobase_summary_list_ro_server_to_client.decode.txt`, `artifacts/rac/v11/v11_infobase_summary_list_ro_response.hex`, `artifacts/rac/v11/v11_infobase_summary_list_ro_rac.out`.

#### Поля ответа (из `rac`)

Observed field names in `rac infobase summary list` output (v11), with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `infobase` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 2 | 11.0 |
| `descr` | string | yes | 3 | 11.0 |

#### Поля запроса (из `rac`)

Observed request parameters for `rac infobase summary list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 11.0 |


### Infobase Info

- **v11 output reference**: `artifacts/rac/v11/help/infobase_info.out`
- **v11 capture status**: no `v11_` success capture yet; fields below require v11 confirmation.
- **Request**: `0x09` (context), then method `0x30` (observed).
- **Response**: method `0x31`.
- **Parameters**: `16 <cluster_uuid> <infobase_uuid>`.
- **Response body layout** (after `01 00 00 01 31`), observed sequence:
  - `uuid[16]`.
  - `u8 tag` (observed `0x2c`).
  - `u32_be` (observed `0x00000000`).
  - `str8 dbms` (observed `PostgreSQL`).
  - `str8 name` (observed `yaxunit`).
  - `str8 unknown_str` (len=3, bytes `ef bf bd`).
  - `str8 db-server` (observed `localhost`).
  - `str8 db-user` (observed `postgres`).
  - `str8 empty` (len=0).
  - `str8 len=2` (bytes `45 3c`, shown as `E<`).
  - `bytes[4]` (observed `03 b5 78 00`, not UTF-8).
  - `str8 denied-message` (observed `Message`).
  - `str8 denied-parameter` (observed `PARAMETER`).
  - `str8 empty` (len=0).
  - `str8 len=2` (bytes `46 4a`, shown as `FJ`).
  - `u32_be` (observed `0xfc124000`).
  - `str8 descr` (observed `Description`).
  - `str8 unknown_str` (observed `ru_RU`).
  - `str8 db-name` (observed `yaxunit`).
  - `str8 permission-code` (observed `CODE`).
  - `tail[28]` (7 x `u32` unknown; observed bytes
    `00000000 00000000 00010000 00000000 000003e7 00000378 00000309`).
- **Evidence**: `logs/session_1771110795_484387_127_0_0_1_41870`.

#### Поля ответа (из `rac`)

Observed field names in `rac infobase info` output (v11), with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `infobase` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 2 | 11.0 |
| `dbms` | string | yes | 3 | 11.0 |
| `db-server` | string | yes | 4 | 11.0 |
| `db-name` | string | yes | 5 | 11.0 |
| `db-user` | string | yes | 6 | 11.0 |
| `security-level` | u32 | yes | 7 | 11.0 |
| `license-distribution` | enum (`allow/deny`) | yes | 8 | 11.0 |
| `scheduled-jobs-deny` | enum (`on/off`) | yes | 9 | 11.0 |
| `sessions-deny` | enum (`on/off`) | yes | 10 | 11.0 |
| `denied-from` | datetime | yes | 11 | 11.0 |
| `denied-message` | string | yes | 12 | 11.0 |
| `denied-parameter` | string | yes | 13 | 11.0 |
| `denied-to` | datetime | yes | 14 | 11.0 |
| `permission-code` | string | yes | 15 | 11.0 |
| `external-session-manager-connection-string` | string | yes | 16 | 11.0 |
| `external-session-manager-required` | enum (`yes/no`) | yes | 17 | 11.0 |
| `security-profile-name` | string | yes | 18 | 11.0 |
| `safe-mode-security-profile-name` | string | yes | 19 | 11.0 |
| `reserve-working-processes` | enum (`yes/no`) | yes | 20 | 11.0 |
| `descr` | string | yes | 21 | 11.0 |

#### Поля запроса (из `rac`)

Observed request parameters for `rac infobase info` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `infobase` | UUID | yes | 4 | 11.0 |


### Infobase Summary Info

- **v11 output reference**: `artifacts/rac/v11/help/infobase_summary_info.out`
- **Request**: `0x09` (context), then method `0x2e`.
- **Response**: method `0x2f`.
- **Parameters**: `16 <cluster_uuid> <infobase_uuid>`.
- **Response fields** (hypothesis): infobase summary record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103996_390065_127_0_0_1_49450`.

#### Поля ответа (из `rac`)

Same field set as `infobase summary list` (see above).

#### Поля запроса (из `rac`)

Observed request parameters for `rac infobase summary info` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 11.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 11.0 |
| `infobase` | UUID | yes | 4 | 11.0 |

### Infobase Summary Update

Sources:
- `artifacts/rac/v11/help/infobase_help.txt` (command parameters)

### RPC

Request/response method ids: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac infobase summary update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `infobase` | UUID | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |

### Поля ответа

Output not captured yet. Likely ACK-only.

### Infobase Create

Sources:
- `artifacts/rac/v11/help/infobase_help.txt` (command parameters)

### RPC

Request/response method ids: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac infobase create` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `create-database` | flag | no | - | 11.0 |
| `name` | string | no | - | 11.0 |
| `dbms` | enum | no | - | 11.0 |
| `db-server` | string | no | - | 11.0 |
| `db-name` | string | no | - | 11.0 |
| `locale` | string | no | - | 11.0 |
| `db-user` | string | no | - | 11.0 |
| `db-pwd` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `date-offset` | string | no | - | 11.0 |
| `security-level` | u32 | no | - | 11.0 |
| `scheduled-jobs-deny` | enum (`on/off`) | no | - | 11.0 |
| `license-distribution` | enum (`deny/allow`) | no | - | 11.0 |

### Поля ответа

Output not captured yet. Likely returns created `infobase` UUID or ACK-only.

### Infobase Update

Sources:
- `artifacts/rac/v11/help/infobase_help.txt` (command parameters)

### RPC

Request/response method ids: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac infobase update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `infobase` | UUID | no | - | 11.0 |
| `infobase-user` | string | no | - | 11.0 |
| `infobase-pwd` | string | no | - | 11.0 |
| `dbms` | enum | no | - | 11.0 |
| `db-server` | string | no | - | 11.0 |
| `db-name` | string | no | - | 11.0 |
| `db-user` | string | no | - | 11.0 |
| `db-pwd` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `denied-from` | datetime | no | - | 11.0 |
| `denied-message` | string | no | - | 11.0 |
| `denied-parameter` | string | no | - | 11.0 |
| `denied-to` | datetime | no | - | 11.0 |
| `permission-code` | string | no | - | 11.0 |
| `sessions-deny` | enum (`on/off`) | no | - | 11.0 |
| `scheduled-jobs-deny` | enum (`on/off`) | no | - | 11.0 |
| `license-distribution` | enum (`deny/allow`) | no | - | 11.0 |
| `external-session-manager-connection-string` | string | no | - | 11.0 |
| `external-session-manager-required` | enum (`yes/no`) | no | - | 11.0 |
| `reserve-working-processes` | enum (`yes/no`) | no | - | 11.0 |
| `security-profile-name` | string | no | - | 11.0 |
| `safe-mode-security-profile-name` | string | no | - | 11.0 |

### Поля ответа

Output not captured yet. Likely ACK-only.

### Infobase Drop

Sources:
- `artifacts/rac/v11/help/infobase_help.txt` (command parameters)

### RPC

Request/response method ids: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac infobase drop` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `infobase` | UUID | no | - | 11.0 |
| `infobase-user` | string | no | - | 11.0 |
| `infobase-pwd` | string | no | - | 11.0 |
| `drop-database` | flag | no | - | 11.0 |
| `clear-database` | flag | no | - | 11.0 |

### Поля ответа

Output not captured yet. Likely ACK-only.


### Infobase Info

- **Request**: `0x09` (context), `0x0a` (infobase context), then method `0x30`.
- **Response**: method `0x31`.
- **Parameters**: `16 <cluster_uuid> <infobase_uuid>`.
- **Response fields** (hypothesis): infobase record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103995_390019_127_0_0_1_49436`.
