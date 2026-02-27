# RAC Profile Message Formats (Observed)

Protocol version (service negotiation): `v8.service.Admin.Cluster` `11.0` (per v11 help).

Sources (v11):
- `artifacts/rac/v11/help/profile_help.txt`
- `artifacts/rac/v11/v11_profile_list_nonempty2_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_list_nonempty2_server_to_client.decode.txt`
- `artifacts/rac/v11/v11_profile_list_nonempty2_response.hex`
- `artifacts/rac/v11/v11_profile_list_nonempty2_rac.out`
- `artifacts/rac/v11/v11_profile_update_all_yes_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_update_all_yes_request.hex`
- `artifacts/rac/v11/v11_profile_update_cfg_no_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_update_cfg_yes_re_no_client_to_server.decode.txt`

Sources (v16):
- `artifacts/rac/v16/help/profile_help.txt`
- `artifacts/rac/v16/help/profile_list.out`
- `docs/rac/documentation/rac_cli_method_map.generated.md` (method IDs)

## Profile List

Source capture:
- `logs/session_1772227765_2726275_127_0_0_1_39084/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/v11/v11_profile_list_nonempty2_response.hex`

RAC output reference:
- `artifacts/rac/v11/v11_profile_list_nonempty2_rac.out`
- `artifacts/rac/v16/help/profile_list.out`

### Поля ответа (из `rac`)

Observed field names in `rac profile list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `name` | string | yes | 1 | 11.0 |
| `descr` | string | yes | 2 | 11.0 |
| `directory` | enum (u8: list/full) | yes | 3 | 11.0 |
| `com` | enum (u8: list/full) | yes | 4 | 11.0 |
| `addin` | enum (u8: list/full) | yes | 5 | 11.0 |
| `module` | enum (u8: list/full) | yes | 6 | 11.0 |
| `app` | enum (u8: list/full) | yes | 7 | 11.0 |
| `config` | bool | yes | 8 | 11.0 |
| `priv` | bool | yes | 9 | 11.0 |
| `inet` | enum (u8: list/full) | yes | 10 | 11.0 |
| `crypto` | bool | yes | 11 | 11.0 |
| `right-extension` | bool | yes | 12 | 11.0 |
| `right-extension-definition-roles` | string (list) | yes | 13 | 11.0 |
| `all-modules-extension` | bool | yes | 14 | 11.0 |
| `modules-available-for-extension` | string (list) | yes | 15 | 11.0 |
| `modules-not-available-for-extension` | string (list) | yes | 16 | 11.0 |
| `privileged-mode-roles` | string (list) | yes | 17 | 11.0 |

### RPC

Request method: `0x59` (`profile list --cluster <id>`)
Response method: `0x5a`

Notes:
- Preceded by `ClusterAuth` request `0x09` with `cluster`, `cluster-user`, `cluster-pwd` (observed in capture).

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x04`)
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
| `0x00` | `1` | `name_len` | u8 | |
| `0x01` | `name_len` | `name` | string | UTF-8 |
| `0x01 + name_len` | `1` | `descr_len` | u8 | |
| `0x02 + name_len` | `descr_len` | `descr` | string | UTF-8 |
| `0x02 + name_len + descr_len` | `1` | `directory_access` | u8 | enum: list/full |
| `0x03 + name_len + descr_len` | `1` | `com_access` | u8 | enum: list/full |
| `0x04 + name_len + descr_len` | `1` | `addin_access` | u8 | enum: list/full |
| `0x05 + name_len + descr_len` | `1` | `module_access` | u8 | enum: list/full |
| `0x06 + name_len + descr_len` | `1` | `app_access` | u8 | enum: list/full |
| `0x07 + name_len + descr_len` | `1` | `config` | u8 | 0/1 |
| `0x08 + name_len + descr_len` | `1` | `priv` | u8 | 0/1 |
| `0x09 + name_len + descr_len` | `1` | `inet_access` | u8 | enum: list/full |
| `0x0a + name_len + descr_len` | `1` | `crypto` | u8 | 0/1 |
| `0x0b + name_len + descr_len` | `1` | `right_extension` | u8 | 0/1 |
| `0x0c + name_len + descr_len` | `1` | `right_extension_definition_roles_len` | u8 | |
| `0x0d + name_len + descr_len` | `right_extension_definition_roles_len` | `right_extension_definition_roles` | string | list, `;`-separated |
| `...` | `1` | `all_modules_extension` | u8 | 0/1 |
| `...` | `1` | `modules_available_for_extension_len` | u8 | |
| `...` | `modules_available_for_extension_len` | `modules_available_for_extension` | string | list, `;`-separated |
| `...` | `1` | `modules_not_available_for_extension_len` | u8 | |
| `...` | `modules_not_available_for_extension_len` | `modules_not_available_for_extension` | string | list, `;`-separated |
| `...` | `1` | `privileged_mode_roles_len` | u8 | |
| `...` | `privileged_mode_roles_len` | `privileged_mode_roles` | string | list, `;`-separated |

Notes:
- The access field ordering (`directory/com/addin/module/app/inet`) matches observed payload order, but needs a capture with non-default access values to confirm exact mapping.

## Hypotheses

- Access fields (`directory/com/addin/module/app/inet`) are enums `list/full` encoded as `u8` in the record, but non-default values were not observed.
- `full-privileged-mode` is not present in the v11 request payload; verify if it appears in v16.

## Open Questions

- Confirm access field ordering by toggling one ACL category to `full`.
- Determine whether `full-privileged-mode` appears in v16 payloads or is inferred server-side.

## Gap Analysis (Required)

- Access fields (`directory/com/addin/module/app/inet`) are all `0` in captures. Needed: run `rac profile acl <category> ... --access full` for one category and capture `profile list` to map each access byte to its field.
- `full-privileged-mode` not observed in v11 payloads. Needed: capture with v16 client and toggle `--full-privileged-mode` to see if a new byte appears.

## Profile Update

Sources:
- `artifacts/rac/v16/help/profile_help.txt`
- `artifacts/rac/v11/v11_profile_update_all_yes_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_update_all_yes_request.hex`
- `artifacts/rac/v11/v11_profile_update_cfg_no_client_to_server.decode.txt`
- `artifacts/rac/v11/v11_profile_update_cfg_yes_re_no_client_to_server.decode.txt`

### RPC

Request method: `0x5b` (`profile update --cluster <id> ...`)
Response: ACK-only (`01 00 00 00`) with no explicit method ID.

### Поля запроса (из `rac`)

Observed request parameters for `rac profile update` (v16).

| Field | Type | Found In Capture | Order In Capture | Version |
| --- | --- | --- | --- | --- |
| `cluster` | UUID | yes | 1 | 11.0 |
| `name` | string | yes | 2 | 11.0 |
| `descr` | string | yes | 3 | 11.0 |
| `config` | bool (`yes/no`) | yes | 4 | 11.0 |
| `priv` | bool (`yes/no`) | yes | 5 | 11.0 |
| `full-privileged-mode` | bool (`yes/no`) | no | - | 11.0 |
| `privileged-mode-roles` | string (list) | yes | 16 | 11.0 |
| `crypto` | bool (`yes/no`) | yes | 10 | 11.0 |
| `right-extension` | bool (`yes/no`) | yes | 11 | 11.0 |
| `right-extension-definition-roles` | string (list) | yes | 12 | 11.0 |
| `all-modules-extension` | bool (`yes/no`) | yes | 13 | 11.0 |
| `modules-available-for-extension` | string (list) | yes | 14 | 11.0 |
| `modules-not-available-for-extension` | string (list) | yes | 15 | 11.0 |

### Поля ответа

ACK-only (`01 00 00 00`).

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
