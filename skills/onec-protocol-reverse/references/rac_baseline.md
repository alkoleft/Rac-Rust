# RAC Baseline (Observed)

Validated against 1C `rac` 8.5.1.1150 with local captures.

## Transport

Regular framed packets:

- `opcode:u8`
- `payload_len:varuint` (LEB128-like)
- `payload[payload_len]`

Example length >127:

- `0x84 0x01` => 132

Init packet is special and precedes framed packets:

- starts with `1c 53 57 50` (`SWP`)
- includes `connect.timeout`

## Session Skeleton

1. Client sends init packet.
2. Server replies `opcode=0x02 len=1 payload=80`.
3. Client sends service negotiation:
   - `opcode=0x0b`
   - payload contains `v8.service.Admin.Cluster` and `16.0`
4. Server replies `opcode=0x0c`.
5. Business RPC on `opcode=0x0e`.
6. Client closes with `opcode=0x0d len=1 payload=01`.

## RPC Envelope In `opcode=0x0e`

Common request/response head:

- `01 00 00 01 <method_id:u8> ...`

Common server ack/status payload:

- `01 00 00 00`

## Known Stable Fields

- Cluster UUID (sample): `1619820a-d36f-4d8a-a716-1516b1dea077`
- Host string example: `alko-home`
- Cluster display name example: `Локальный кластер`
- Agent version flow:
  - request method `0x87`
  - response method `0x88`
  - response string: `8.5.1.1150`

## Parameter Patterns

- Cluster context:
  - `01 00 00 01 09 16 <cluster_uuid_16b> 00 00`
- Cluster-scoped method:
  - `01 00 00 01 <method> 16 <cluster_uuid_16b>`
- Object-scoped method:
  - `01 00 00 01 <method> 16 <cluster_uuid_16b> <object_uuid_16b>`
