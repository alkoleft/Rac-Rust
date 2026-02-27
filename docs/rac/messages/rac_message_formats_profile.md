# RAC Profile Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `11.0` (per v11 help).

Sources (v11):
- `artifacts/rac/v11/help/profile_help.txt`
- `artifacts/rac/v11/v11_profile_list_ro_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_list_ro_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_profile_list_ro_response.hex`
- `artifacts/rac/v11/v11_profile_list_ro_rac.out`

Sources (v16):
- `artifacts/rac/v16/help/profile_help.txt`
- `artifacts/rac/v16/help/profile_list.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Profile List

Source capture:
- `logs/session_1772063106_1251253_127_0_0_1_57350/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/profile_list_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_profile_list_ro_rac.out`
- `artifacts/rac/v16/help/profile_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac profile list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `profile` | UUID (hypothesis) | no (empty list) | 1 | 11.0 |

### RPC

Request method: `0x59` (`profile list --cluster <id>`)
Response method: `0x5a`

Notes:
- Preceded by `ClusterAuth` request `0x09` with `cluster`, `cluster-user`, `cluster-pwd` (observed in capture).

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x00`)
- offset `0x01`: first record starts here (if any)

### Поля запроса (из `rac`)

Observed request parameters for `rac profile list`.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |

### Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `profile` | UUID | hypothesis; no records observed |

Notes:
- No profile records present in the capture (empty list), so the record layout is unconfirmed.

## Hypotheses

- Each profile record is a single UUID; update if non-empty list capture shows name/flags fields.

## Open Questions

- What fields are returned in `rac profile list` (name, description, flags)?
- Does the list record include booleans or ACL-related counts beyond the identifier?

## Gap Analysis (Required)

- Unknown record payload (expected after `count:u8`). Candidate layouts: UUID-only (16 bytes), or `name:str8` followed by flags/booleans.
- Required capture to confirm: create at least one profile, re-run `rac profile list` with non-empty output, and extract `artifacts/rac/profile_list_response_nonempty.hex`.

## Profile Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
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
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `name` | string | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
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
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl directory remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `alias` | string (URL) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
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
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl com remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `com-name` | string | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `addin-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `hash` | string (base64) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `addin-name` | string | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `module-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `hash` | string (base64) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `module-name` | string | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `app-name` | string | no | - | 11.0 |
| `descr` | string | no | - | 11.0 |
| `wild` | string (URL) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `app-name` | string | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |

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
| `cluster` | UUID | no | - | 11.0 |
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
- `artifacts/rac/v16/help/profile_help.txt`

### RPC

Request/response method IDs: not captured yet (v11 help only).

### Поля запроса (из `rac`)

Observed request parameters for `rac profile acl inet remove` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | no | - | 11.0 |
| `name` | string (profile) | no | - | 11.0 |
| `access` | enum (`list`, `full`) | no | - | 11.0 |
| `inet-name` | string | no | - | 11.0 |

### Поля ответа

Not captured yet (likely ACK-only).
