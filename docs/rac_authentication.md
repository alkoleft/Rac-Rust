# RAC Authentication (Observed)

This page documents the observed authentication flow for RAC agent commands.
New authentication variants can be added as separate sections.

## Scope

Captured command:

`rac agent admin list --agent-user=admin --agent-pwd=pass`

Capture directory:

`logs/session_1771283676_2737297_127_0_0_1_59274`

## Transport Recap

- Init packet `SWP` then framed packets.
- Business RPC frames use `opcode=0x0e`.
- RPC head is `01 00 00 01 <method_id>`.

## Agent Authentication Flow

### Summary

| Case | Auth Request | Auth Ack | Failure Envelope |
|---|---|---|---|
| Success | `0x08` with user/pass | `01 00 00 00` | none |
| Failure | `0x08` with user/pass | none | `01 00 00 ff ... #Failure ...` |

### 1) Auth Request (client -> server)

Frame:

`opcode=0x0e`

Payload (hex):

`01 00 00 01 08 05 61 64 6d 69 6e 04 70 61 73 73`

Decoded:

- `01 00 00 01` RPC head
- `0x08` method id
- `0x05 "admin"` username
- `0x04 "pass"` password

### 2) Auth Ack (server -> client)

Frame:

`opcode=0x0e`

Payload (hex):

`01 00 00 00`

Observed as positive ack for valid credentials.

### 3) Business Call (client -> server)

After ack, the client sends:

Payload (hex):

`01 00 00 01 00`

Hypothesis: method `0x00` is `agent admin list`.

### 4) Business Response (server -> client)

Payload (hex):

`01 00 00 01 01 01 05 61 64 6d 69 6e 00 03 ef bf bd 01 00 00`

Decoded:

- `01 00 00 01` RPC head
- `0x01` response method id
- `0x05 "admin"` admin name
- Remaining bytes: unknown fields (see Open Questions)

### Failure Response (server -> client)

Captured command:

`rac agent admin list --agent-user=admin --agent-pwd=wrong`

Capture directory:

`logs/session_1771284372_2745705_127_0_0_1_37174`

Frame:

`opcode=0x0e`

Payload (hex):

`01 00 00 ff 20 76 38 2e 73 65 72 76 69 63 65 2e 41 64 6d 69 6e 2e 43 6c 75 73 74 65 72 23 46 61 69 6c 75 72 65 51 01 d0 90 d0 b4 d0 bc d0 b8 d0 bd d0 b8 d1 81 d1 82 d1 80 d0 b0 d1 82 d0 be d1 80 20 d0 ba d0 bb d0 b0 d1 81 d1 82 d0 b5 d1 80 d0 b0 20 d0 bd d0 b5 20 d0 b0 d1 83 d1 82 d0 b5 d0 bd d1 82 d0 b8 d1 84 d0 b8 d1 86 d0 b8 d1 80 d0 be d0 b2 d0 b0 d0 bd 00 80`

Decoded text:

- `v8.service.Admin.Cluster#Failure`
- `Администратор кластера не аутентифицирован`

## Method Map Updates

See `skills/onec-protocol-reverse/references/rac_method_map.md` for the current
mapping entries.

## Open Questions

- Confirm error response when auth fails.
- Decode remaining fields in the admin list record.
- Confirm `0x00 -> 0x01` mapping with additional captures.

## Cluster Authentication Flow

### Summary

| Case | Auth Request | Auth Ack | Failure Envelope |
|---|---|---|---|
| Success | `0x09` with cluster UUID + user/pass | `01 00 00 00` | none |
| Failure | `0x09` with cluster UUID + user/pass | none | `01 00 00 ff ... #Failure ...` |

Captured command:

`rac infobase summary list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --cluster-user=cadmin --cluster-pwd=cpass`

Capture directory:

`logs/session_1771284163_2743105_127_0_0_1_43908`

### 1) Cluster Auth / Context (client -> server)

Frame:

`opcode=0x0e`

Payload (hex):

`01 00 00 01 09 16 19 82 0a d3 6f 4d 8a a7 16 15 16 b1 de a0 77 06 63 61 64 6d 69 6e 05 63 70 61 73 73`

Decoded:

- `01 00 00 01` RPC head
- `0x09` method id
- `1619820a-d36f-4d8a-a716-1516b1dea077` cluster UUID (16 bytes)
- `0x06 "cadmin"` cluster user
- `0x05 "cpass"` cluster password

### 2) Auth Ack (server -> client)

Frame:

`opcode=0x0e`

Payload (hex):

`01 00 00 00`

Observed as positive ack for valid credentials.

### 3) Business Call (client -> server)

After ack, the client sends:

Payload (hex):

`01 00 00 01 2a 16 19 82 0a d3 6f 4d 8a a7 16 15 16 b1 de a0 77`

Method `0x2a` is `infobase summary list`.

### Failure Response (server -> client)

Captured command:

`rac infobase summary list --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --cluster-user=cadmin --cluster-pwd=wrong`

Capture directory:

`logs/session_1771284296_2744683_127_0_0_1_40444`

Frame:

`opcode=0x0e`

Payload (hex):

`01 00 00 ff 20 76 38 2e 73 65 72 76 69 63 65 2e 41 64 6d 69 6e 2e 43 6c 75 73 74 65 72 23 46 61 69 6c 75 72 65 51 01 d0 90 d0 b4 d0 bc d0 b8 d0 bd d0 b8 d1 81 d1 82 d1 80 d0 b0 d1 82 d0 be d1 80 20 d0 ba d0 bb d0 b0 d1 81 d1 82 d0 b5 d1 80 d0 b0 20 d0 bd d0 b5 20 d0 b0 d1 83 d1 82 d0 b5 d0 bd d1 82 d0 b8 d1 84 d0 b8 d1 86 d0 b8 d1 80 d0 be d0 b2 d0 b0 d0 bd 00 80`

Decoded text:

- `v8.service.Admin.Cluster#Failure`
- `Администратор кластера не аутентифицирован`

## Error Envelope (Observed)

When authentication fails (agent or cluster), the server responds with an error
payload on `opcode=0x0e`:

- Prefix: `01 00 00 ff`
- Error class: `v8.service.Admin.Cluster#Failure`
- Error message: UTF-8 text, null-terminated, followed by `0x80` (observed)

See the failure captures above for full payloads.

## Infobase Authentication Flow

Captured command (success):

`rac infobase info --infobase=717bdda7-2f60-4577-b262-f1fc8c0e472c --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --cluster-user=cadmin --cluster-pwd=cpass --infobase-user=iadmin --infobase-pwd=ipass`

Capture directory:

`logs/session_1771284695_2749969_127_0_0_1_51404`

### Summary

| Case | Cluster Auth | Infobase Auth | Ack(s) | Failure Envelope |
|---|---|---|---|---|
| Success | `0x09` with cluster UUID + user/pass | `0x0a` with cluster UUID + user/pass | two `01 00 00 00` | none |
| Failure | `0x09` with cluster UUID + user/pass | `0x0a` with cluster UUID + user/pass | two `01 00 00 00` | `01 00 00 ff ... #Rights ...` |

### 1) Cluster Auth / Context (client -> server)

`opcode=0x0e`

Payload (hex):

`01 00 00 01 09 16 19 82 0a d3 6f 4d 8a a7 16 15 16 b1 de a0 77 06 63 61 64 6d 69 6e 05 63 70 61 73 73`

### 2) Cluster Ack (server -> client)

`opcode=0x0e` payload:

`01 00 00 00`

### 3) Infobase Auth (client -> server)

`opcode=0x0e`

Payload (hex):

`01 00 00 01 0a 16 19 82 0a d3 6f 4d 8a a7 16 15 16 b1 de a0 77 06 69 61 64 6d 69 6e 05 69 70 61 73 73`

Decoded:

- `0x0a` method id
- cluster UUID
- `0x06 "iadmin"`
- `0x05 "ipass"`

Note: infobase UUID is not present in the auth call; it is provided in the
subsequent business call (`0x30`).

### 4) Infobase Ack (server -> client)

`opcode=0x0e` payload:

`01 00 00 00`

### 5) Business Call (client -> server)

Payload (hex):

`01 00 00 01 30 16 19 82 0a d3 6f 4d 8a a7 16 15 16 b1 de a0 77 71 7b dd a7 2f 60 45 77 b2 62 f1 fc 8c 0e 47 2c`

Method `0x30` is `infobase info`.

### 6) Business Response (server -> client)

Response method `0x31` with infobase details:

`01 00 00 01 31 71 7b dd a7 2f 60 45 77 b2 62 f1 fc 8c 0e 47 2c ...`

## Infobase Authentication Failure

Captured command:

`rac infobase info --infobase=717bdda7-2f60-4577-b262-f1fc8c0e472c --cluster 1619820a-d36f-4d8a-a716-1516b1dea077 --cluster-user=cadmin --cluster-pwd=cpass --infobase-user=iadmin --infobase-pwd=wrong`

Capture directory:

`logs/session_1771284700_2750018_127_0_0_1_59556`

Failure frame:

`opcode=0x0e`

Payload (hex):

`01 00 00 ff 1f 76 38 2e 73 65 72 76 69 63 65 2e 41 64 6d 69 6e 2e 43 6c 75 73 74 65 72 23 52 69 67 68 74 73 6d 01 d0 9d d0 b5 d0 b4 d0 be d1 81 d1 82 d0 b0 d1 82 d0 be d1 87 d0 bd d0 be 20 d0 bf d1 80 d0 b0 d0 b2 d0 bf d0 be d0 bb d1 8c d0 b7 d0 be d0 b2 d0 b0 d1 82 d0 b5 d0 bb d1 8f 20 d0 bd d0 b0 20 d0 b8 d0 bd d1 84 d0 be d1 80 d0 bc d0 b0 d1 86 d0 b8 d0 be d0 bd d0 bd d1 83 d1 8e 20 d0 b1 d0 b0 d0 b7 d1 83 20 79 61 78 75 6e 69 74 00 80`

Decoded text:

- `v8.service.Admin.Cluster#Rights`
- `Недостаточно прав пользователя на информационную базу yaxunit`
