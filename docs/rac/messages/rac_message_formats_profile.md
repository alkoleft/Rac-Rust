# RAC Profile Message Formats (v11)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `11.0` (per v11 help).

Sources (v11):
- `artifacts/rac/v11_help/profile_help.txt`
- `artifacts/rac/v11_help/profile_list.out`

## Profile List

Sources:
- `artifacts/rac/v11_help/profile_help.txt`
- `artifacts/rac/v11_help/profile_list.out`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |

### Поля ответа

Not captured yet. The v11 help does not describe response fields.

## Profile Update

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `config` | bool (`yes/no`) | no | - | 11.0 |
| `priv` | bool (`yes/no`) | no | - | 11.0 |
| `full-privileged-mode` | bool (`yes/no`) | no | - | 11.0 |
| `privileged-mode-roles` | string (list) | no | - | 11.0 |
| `crypto` | bool (`yes/no`) | no | - | 11.0 |
| `right-extension` | bool (`yes/no`) | no | - | 11.0 |
| `right-extension-definition-roles` | string (list) | no | - | 11.0 |
| `all-modules-extension` | bool (`yes/no`) | no | - | 11.0 |
| `modules-available-for-extension` | string (list) | no | - | 11.0 |
| `modules-not-available-for-extension` | string (list) | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile Remove

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Directory List

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl directory list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

### Поля ответа

Not captured yet.

## Profile ACL Directory Update

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl directory update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `alias` | string (URL) | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `physicalPath` | string (URL) | no | - | 11.0 |
| `allowedRead` | bool (`yes/no`) | no | - | 11.0 |
| `allowedWrite` | bool (`yes/no`) | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Directory Remove

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl directory remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `alias` | string (URL) | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL COM List

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl com list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

### Поля ответа

Not captured yet.

## Profile ACL COM Update

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl com update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `com-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `fileName` | string (URL) | no | - | 11.0 |
| `id` | UUID | no | - | 11.0 |
| `host` | string (URL) | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL COM Remove

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl com remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `com-name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Addin List

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl addin list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

### Поля ответа

Not captured yet.

## Profile ACL Addin Update

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl addin update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `addin-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `hash` | string (base64) | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Addin Remove

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl addin remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `addin-name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Module List

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl module list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

### Поля ответа

Not captured yet.

## Profile ACL Module Update

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl module update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `module-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `hash` | string (base64) | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Module Remove

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl module remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `module-name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL App List

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl app list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

### Поля ответа

Not captured yet.

## Profile ACL App Update

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl app update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `app-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `wild` | string (URL) | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL App Remove

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl app remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `app-name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Inet List

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl inet list` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

### Поля ответа

Not captured yet.

## Profile ACL Inet Update

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl inet update` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `inet-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `protocol` | string | no | - | 11.0 |
| `url` | string (URL) | no | - | 11.0 |
| `port` | u32 | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).

## Profile ACL Inet Remove

Sources:
- `artifacts/rac/v11_help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl inet remove` (v11).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `cluster-user` | string | no | - | 11.0 |
| `cluster-pwd` | string | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `inet-name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).
