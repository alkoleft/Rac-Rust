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
| `0x12` | `0x13` | `manager list --cluster <id>` | manager list |
| `0x14` | `0x15` | `manager info --cluster <id> --manager <id>` | manager info |
| `0x16` | `0x17` | `server list --cluster <id>` | server list |
| `0x18` | `0x19` | `server info --cluster <id> --server <id>` | server info |
| `0x1d` | `0x1e` | `process list --cluster <id>` | process list |
| `0x1f` | `0x20` | `process info --cluster <id> --process <id>` | process info |
| `0x2a` | `0x2b` | `infobase summary list --cluster <id>` | returns empty list in this env |
| `0x32` | `0x33` | `connection list --cluster <id>` | connection list |
| `0x87` | `0x88` | `agent version` and extra step in process commands | returns platform version string |

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
