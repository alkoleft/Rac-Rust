# RAC Lock Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).

Sources (v11):
- `artifacts/rac/v11_help/lock_help.txt`
- `artifacts/rac/v11_help/lock_list.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Lock List

Source capture:
- `logs/session_1771361253_3690919_127_0_0_1_60046/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/lock_list_response.hex`

RAC output reference:
- `artifacts/rac/lock_list_rac.out`
- `artifacts/rac/v11_help/lock_list.out`

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

Request method: `0x48` (`lock list --cluster <id>`)
Response method: `0x49`

Payload structure (method body):
- offset `0x00` (request): `16 <cluster_uuid_16b>`
- offset `0x00` (response): `items_count:u8`

### Поля запроса (из `rac`)

Observed request parameters for `rac lock list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `cluster-user` | string | yes | 2 | 11.0 |
| `cluster-pwd` | string | yes | 3 | 11.0 |
| `infobase` | UUID | no | 4 | 11.0 |
| `connection` | UUID | no | 5 | 11.0 |
| `session` | UUID | no | 6 | 11.0 |

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
- Do `--infobase`, `--connection`, or `--session` introduce extra request fields beyond the cluster UUID in method `0x48`?

### Gap Analysis (Required)

- Capture `lock list --infobase <uuid>`, `--connection <uuid>`, and `--session <uuid>` to confirm request field order and whether additional parameters are appended after `<cluster_uuid_16b>`.
- Capture at least one lock record with a non-zero `object` UUID and correlate it to confirm that `object` is always present at the tail.
