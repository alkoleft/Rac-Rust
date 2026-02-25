# RAC Service Message Formats (v11)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `11.0` (per v11 help).

Sources (v11):
- `artifacts/rac/v11_help/service_help.txt`
- `artifacts/rac/v11_help/service_list.out`

## Service List

Sources:
- `artifacts/rac/v11_help/service_help.txt`
- `artifacts/rac/v11_help/service_list.out`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac service list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |

### Поля ответа

Not captured yet. The v11 help does not describe response fields.
