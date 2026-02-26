# RAC Lock Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v16):
- `artifacts/rac/v16/help/lock_help.txt`
- `artifacts/rac/v16/help/lock_list.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)
- `artifacts/rac/v11/v11_lock_list_connection_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_lock_list_connection_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_lock_list_connection_ro_response.hex`
- `artifacts/rac/v11/v11_lock_list_connection_ro_rac.out`
- `artifacts/rac/v11/v11_lock_list_infobase_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_lock_list_infobase_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_lock_list_infobase_ro_response.hex`
- `artifacts/rac/v11/v11_lock_list_infobase_ro_rac.out`
- `artifacts/rac/v11/v11_lock_list_session_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_lock_list_session_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_lock_list_session_ro_response.hex`
- `artifacts/rac/v11/v11_lock_list_session_ro_rac.out`

## Lock List

Source capture:
- `artifacts/rac/v11/v11_lock_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_lock_list_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_lock_list_connection_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_lock_list_infobase_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_lock_list_session_ro_server_to_client.decode.txt`

Payload example:
- `artifacts/rac/v11/v11_lock_list_ro_response.hex`
- `artifacts/rac/v11/v11_lock_list_connection_ro_response.hex`
- `artifacts/rac/v11/v11_lock_list_infobase_ro_response.hex`
- `artifacts/rac/v11/v11_lock_list_session_ro_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_lock_list_ro_rac.out`
- `artifacts/rac/v16/help/lock_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac lock list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `connection` | UUID | yes | 1 | 11.0 |
| `descr` | string | yes | 2 | 11.0 |
| `locked` | datetime (u64 ticks, 100us since 0001-01-01) | yes | 3 | 11.0 |
| `session` | UUID | yes | 4 | 11.0 |
| `object` | UUID | yes | 5 | 11.0 |

### RPC

Request methods:
- `0x48` (`lock list --cluster <id>`)
- `0x4a` (`lock list --infobase <uuid>`)
- `0x4c` (`lock list --connection <uuid>`)
Response methods:
- `0x49` (for `0x48`)
- `0x4b` (for `0x4a`)
- `0x4d` (for `0x4c`)

Payload structure (method body):
- offset `0x00` (request): `16 <cluster_uuid_16b>`
- offset `0x10` (request, `--infobase`): `16 <infobase_uuid_16b>`
- offset `0x10` (request, `--connection`): `16 <connection_uuid_16b>`
- offset `0x10` (request, `--session`): not present in capture; payload matches `lock list --cluster <id>`
- offset `0x00` (response): `items_count:u8`

### Поля запроса (из `rac`)

Observed request parameters for `rac lock list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes | 2 | 11.0 |
| `cluster-pwd` | string | yes | 3 | 11.0 |
| `infobase` | UUID | yes | 4 | 11.0 |
| `connection` | UUID | yes | 5 | 11.0 |
| `session` | UUID | no (payload matches `--cluster` only) | 6 | 11.0 |

Notes:
- `cluster-user`/`cluster-pwd` are sent via the context setter (`rpc_method_id=0x09`) before the `lock list` request. Order in that context payload: `cluster`, `cluster-user`, `cluster-pwd`.

### Record Layout (Observed)

Offsets are relative to the start of a record.

Variant A (no `descr-flag`):

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `connection` | UUID | |
| `0x10` | `1` | `descr-len` | u8 | |
| `0x11` | `descr-len` | `descr` | string | UTF-8 |
| `0x11 + descr-len` | `8` | `locked` | datetime (u64 ticks, 100us since 0001-01-01) | |
| `0x19 + descr-len` | `16` | `session` | UUID | |
| `0x29 + descr-len` | `16` | `object` | UUID | |

Variant B (with `descr-flag` byte):

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `connection` | UUID | |
| `0x10` | `1` | `descr-len` | u8 | |
| `0x11` | `1` | `descr-flag` | u8 | observed `0x01` in some records |
| `0x12` | `descr-len` | `descr` | string | UTF-8 |
| `0x12 + descr-len` | `8` | `locked` | datetime (u64 ticks, 100us since 0001-01-01) | |
| `0x1a + descr-len` | `16` | `session` | UUID | |
| `0x2a + descr-len` | `16` | `object` | UUID | |

### Hypotheses

- `descr-flag` encodes the lock subtype (observed `0x01` for `ServerJobExecutorContext`/`AgentStandardCall` records).

### Open Questions

- Which values can `descr-flag` take besides `0x01`, and does it appear for other lock kinds?
- Does `--session` introduce extra request fields beyond the cluster UUID? Current capture shows no extra payload fields.

### Gap Analysis (Required)

- If `--session` should be sent on wire, capture a case where `rac` actually includes it (current capture did not).
- Capture at least one lock record with a non-zero `object` UUID and correlate it to confirm that `object` is always present at the tail.
