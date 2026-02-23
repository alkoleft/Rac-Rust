# RAC Message Formats (Read/Info Scope)

Generated from local captures in `logs/` and method map in `docs/rac/documentation/rac_cli_method_map.generated.md`. Response byte examples are stored in `artifacts/rac/`.

## Transport Framing (Confirmed)

- **Init packet**: special pre-frame blob starting with `1c 53 57 50` (`SWP`) and containing connection parameters (e.g. `connect.timeout`).
- **Init packet fields (observed)**:
  - `magic`: `1c 53 57 50`
  - `version:u8`
  - `header_a:u16_be`
  - `header_b:u16_be`
  - `tag:u8`
  - `param_count:u8`
  - `params[param_count]`: `key_len:u8 + key[key_len] + value_type:u8 + value`
  - `value_type=0x04` => `u32_be` (example: `connect.timeout = 2000`)
- **Framed packets**: `opcode:u8 + len:varuint + payload[len]`.
- **Varuint**: LEB128-like, e.g. `0x84 0x01` => 132.

## Session Chain (Confirmed)

1. `client -> server` init packet (raw, not framed)
2. `server -> client` frame `opcode=0x02 len=1 payload=80`
3. `client -> server` service negotiation (frame `opcode=0x0b`, payload includes `v8.service.Admin.Cluster` + service version string)
   - Observed `16.0` in earlier captures, `11.0` in newer captures.
   - Treat as service interface version, not transport framing version.
4. `server -> client` service ack (frame `opcode=0x0c`)
5. RPC frames (frame `opcode=0x0e`, payload begins with `01 00 00 01 <method_id>`)
6. Close (frame `opcode=0x0d len=1 payload=01`)

## RPC Envelope (Confirmed)

- **RPC payload prefix**: `01 00 00 01 <method_id:u8> ...`.
- **Ack/Status**: `01 00 00 00` (no method id).
- **Cluster context setter**: method `0x09` payload `16 <cluster_uuid_16b> 00 00`.
- **Infobase context setter**: method `0x0a` payload `16 <cluster_uuid_16b> 00 00`.
- **Cluster-scoped call**: method `0xXX` payload `16 <cluster_uuid_16b>`.
- **Object-scoped call**: method `0xXX` payload `16 <cluster_uuid_16b> <object_uuid_16b>`.

Notes:
- UUIDs observed as raw 16 bytes in requests (cluster is prefixed by `0x16`).
- In responses, UUIDs often appear as `0x16 <uuid>` (hypothesis: marker byte).

## Command Groups

- `docs/rac/messages/rac_message_formats_cluster.md`
- `docs/rac/messages/rac_message_formats_manager.md`
- `docs/rac/messages/rac_message_formats_process.md`
- `docs/rac/messages/rac_message_formats_infobase.md`
- `docs/rac/messages/rac_message_formats_session.md`
- `docs/rac/messages/rac_message_formats_counter.md`
