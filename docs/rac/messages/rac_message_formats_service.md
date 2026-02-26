# RAC Service Message Formats (v11)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `11.0` (per v11 help).

Sources (v16):
- `artifacts/rac/v16/help/service_help.txt`
- `artifacts/rac/v16/help/service_list.out`
- `artifacts/rac/v11/v11_service_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_service_list_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_service_list_ro_response.hex`
- `artifacts/rac/v11/v11_service_list_ro_rac.out`

## Service List

Sources:
- `artifacts/rac/v16/help/service_help.txt`
- `artifacts/rac/v16/help/service_list.out`
- `artifacts/rac/v11/v11_service_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_service_list_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_service_list_ro_response.hex`
- `artifacts/rac/v11/v11_service_list_ro_rac.out`

### RPC

- **Request**: `0x09` (context), then method `0x23`.
- **Response**: method `0x24`.
- **Parameters**: `16 <cluster_uuid>`.

### Поля запроса (из `rac`)

Observed request parameters for `rac service list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 16.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 16.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 16.0 |

### Поля ответа

Observed field names in `rac service list` output (v16), with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 16.0 |
| `main-only` | u32 | yes | 2 | 16.0 |
| `manager` | UUID | yes | 3 | 16.0 |
| `descr` | string | yes | 4 | 16.0 |
