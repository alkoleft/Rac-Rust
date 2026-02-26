# RAC Profile Message Formats (v11)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `11.0` (per v11 help).

Sources (v16):
- `artifacts/rac/v16/help/profile_help.txt`
- `artifacts/rac/v16/help/profile_list.out`
- `artifacts/rac/v11/v11_profile_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_list_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_profile_list_ro_response.hex`
- `artifacts/rac/v11/v11_profile_list_ro_rac.out`

## Profile List

Sources:
- `artifacts/rac/v16/help/profile_help.txt`
- `artifacts/rac/v16/help/profile_list.out`
- `artifacts/rac/v11/v11_profile_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_list_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_profile_list_ro_response.hex`
- `artifacts/rac/v11/v11_profile_list_ro_rac.out`

### RPC

- **Request**: `0x09` (context), then method `0x59`.
- **Response**: method `0x5a`.
- **Parameters**: `16 <cluster_uuid>`.

### Поля запроса (из `rac`)

Observed request parameters for `rac profile list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 16.0 |
| `cluster-user` | string | yes (in auth/context `0x09`) | 2 | 16.0 |
| `cluster-pwd` | string | yes (in auth/context `0x09`) | 3 | 16.0 |

### Поля ответа

Capture returned an empty list (`items_count=0`), so record layout is not confirmed.

Observed response prefix (payload hex): `01 00 00 01 5a 00`.

## Profile Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string | no | - | 16.0 |
| `descr` | string | no | - | 16.0 |
| `config` | bool (`yes/no`) | no | - | 16.0 |
| `priv` | bool (`yes/no`) | no | - | 16.0 |
| `full-privileged-mode` | bool (`yes/no`) | no | - | 16.0 |
| `privileged-mode-roles` | string (list) | no | - | 16.0 |
| `crypto` | bool (`yes/no`) | no | - | 16.0 |
| `right-extension` | bool (`yes/no`) | no | - | 16.0 |
| `right-extension-definition-roles` | string (list) | no | - | 16.0 |
| `all-modules-extension` | bool (`yes/no`) | no | - | 16.0 |
| `modules-available-for-extension` | string (list) | no | - | 16.0 |
| `modules-not-available-for-extension` | string (list) | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile Remove

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Directory List

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl directory list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |

### Поля ответа

Not captured yet.

## Profile ACL Directory Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl directory update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `alias` | string (URL) | no | - | 16.0 |
| `descr` | string | no | - | 16.0 |
| `physicalPath` | string (URL) | no | - | 16.0 |
| `allowedRead` | bool (`yes/no`) | no | - | 16.0 |
| `allowedWrite` | bool (`yes/no`) | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Directory Remove

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl directory remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `alias` | string (URL) | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL COM List

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl com list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |

### Поля ответа

Not captured yet.

## Profile ACL COM Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl com update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `com-name` | string | no | - | 16.0 |
| `descr` | string | no | - | 16.0 |
| `fileName` | string (URL) | no | - | 16.0 |
| `id` | UUID | no | - | 16.0 |
| `host` | string (URL) | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL COM Remove

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl com remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `com-name` | string | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Addin List

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl addin list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |

### Поля ответа

Not captured yet.

## Profile ACL Addin Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl addin update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `addin-name` | string | no | - | 16.0 |
| `descr` | string | no | - | 16.0 |
| `hash` | string (base64) | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Addin Remove

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl addin remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `addin-name` | string | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Module List

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl module list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |

### Поля ответа

Not captured yet.

## Profile ACL Module Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl module update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `module-name` | string | no | - | 16.0 |
| `descr` | string | no | - | 16.0 |
| `hash` | string (base64) | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Module Remove

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl module remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `module-name` | string | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL App List

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl app list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |

### Поля ответа

Not captured yet.

## Profile ACL App Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl app update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `app-name` | string | no | - | 16.0 |
| `descr` | string | no | - | 16.0 |
| `wild` | string (URL) | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL App Remove

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl app remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `app-name` | string | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Inet List

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl inet list` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |

### Поля ответа

Not captured yet.

## Profile ACL Inet Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl inet update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `inet-name` | string | no | - | 16.0 |
| `descr` | string | no | - | 16.0 |
| `protocol` | string | no | - | 16.0 |
| `url` | string (URL) | no | - | 16.0 |
| `port` | u32 | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Inet Remove

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl inet remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 16.0 |
| `cluster-user` | string | no | - | 16.0 |
| `cluster-pwd` | string | no | - | 16.0 |
| `name` | string (profile) | no | - | 16.0 |
| `access` | enum (`list`, `full`) | no | - | 16.0 |
| `inet-name` | string | no | - | 16.0 |

### Поля ответа

Not captured yet (likely ACK-only).
