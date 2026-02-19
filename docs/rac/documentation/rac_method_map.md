# RAC Method Map (Observed)

Data source: captures from local cluster via proxy on `127.0.0.1:1565`.

## RPC envelope (inside `opcode=0x0e` frames)

Most request/response payloads start with:

- `01 00 00 01 <method_id:u8> ...`

Observed special case:

- `01 00 00 00` in server replies (status/ack frame without method id).

## Known methods

| Request method | Response method | Observed CLI command | Notes |
|---|---|---|---|
| `0x0b` | `0x0c` | `cluster list` | cluster list request/response |
| `0x0d` | `0x0e` | `cluster info --cluster <id>` | cluster info |
| `0x09` | `01 00 00 00` | many cluster-scoped commands | set cluster context (includes cluster UUID) |
| `0x02` | `0x03` | `cluster admin list --cluster <id>` | list cluster administrators (requires cluster auth) |
| `0x05` | `01 00 00 00` | `cluster admin register --cluster <id> --name <name>` | register cluster administrator; response is ack only |
| `0x12` | `0x13` | `manager list --cluster <id>` | manager list |
| `0x14` | `0x15` | `manager info --cluster <id> --manager <id>` | manager info |
| `0x16` | `0x17` | `server list --cluster <id>` | server list |
| `0x18` | `0x19` | `server info --cluster <id> --server <id>` | server info |
| `0x1d` | `0x1e` | `process list --cluster <id>` | process list |
| `0x1f` | `0x20` | `process info --cluster <id> --process <id>` | process info |
| `0x2a` | `0x2b` | `infobase summary list --cluster <id>` | returns empty list in this env |
| `0x32` | `0x33` | `connection list --cluster <id>` | connection list |
| `0x7c` | `0x7d` | `limit list --cluster <id>` | list limits |
| `0x7e` | `0x7f` | `limit info --cluster <id> --limit <name>` | limit info |
| `0x80` | `01 00 00 00` | `limit update --cluster <id> --name <name>` | update/create limit, ACK-only response |
| `0x81` | `01 00 00 00` | `limit remove --cluster <id> --name <name>` | remove limit, ACK-only response |
| `0x87` | `0x88` | `agent version` and extra step in process commands | returns platform version string |
| `0x89` | `0x8a` | `service-setting info --cluster <id> --server <id> --setting <id>` | service setting info |
| `0x8b` | `0x8c` | `service-setting list --cluster <id> --server <id>` | service setting list |
| `0x8d` | `0x8e` | `service-setting insert/update --cluster <id> --server <id>` | create/update service setting (setting UUID zero vs non-zero) |
| `0x8f` | `01 00 00 00` | `service-setting remove --cluster <id> --server <id> --setting <id>` | remove service setting, ACK-only response |
| `0x90` | `01 00 00 00` | `service-setting apply --cluster <id> --server <id>` | apply service settings, ACK-only response |
| `0x91` | `0x92` | `service-setting get-service-data-dirs-for-transfer --cluster <id> --server <id> [--service-name <name>]` | transfer directories |

## Parameter patterns

- Method `0x09` request payload:
  - `01 00 00 01 09 16 <16-byte cluster uuid> 00 00`
- Methods with cluster UUID:
  - `01 00 00 01 <method> 16 <16-byte cluster uuid>`
- Methods with object UUID:
  - `01 00 00 01 <method> 16 <16-byte cluster uuid> <16-byte object uuid>`
- Version request:
  - `01 00 00 01 87`
- Version response:
  - `01 00 00 01 88 0a 38 2e 35 2e 31 2e 31 31 35 30` (`8.5.1.1150`)

## Transport-level framing

- Server/client frame format in regular packets:
  - `opcode:u8`
  - `payload_len:varuint` (LEB128-like)
  - `payload`
- Example varuint length:
  - `0x84 0x01` => `132`

## Session structure

Typical command flow:

1. Special init packet (`1c 53 57 50 ... connect.timeout ...`), not regular framed packet.
2. Service negotiation:
   - c2s `opcode=0x0b`: `v8.service.Admin.Cluster`, `16.0`
   - s2c `opcode=0x0c`: echo + success tail
3. One or more `opcode=0x0e` RPC packets.
4. c2s `opcode=0x0d len=1 payload=01` close/end.
