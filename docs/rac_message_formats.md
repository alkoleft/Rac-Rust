# RAC Message Formats (Read/Info Scope)

Generated from local captures in `logs/` and method map in `docs/rac_cli_method_map.generated.md`.

## Transport Framing (Confirmed)

- **Init packet**: special pre-frame blob starting with `1c 53 57 50` (`SWP`) and containing `connect.timeout`.
- **Framed packets**: `opcode:u8 + len:varuint + payload[len]`.
- **Varuint**: LEB128-like, e.g. `0x84 0x01` => 132.

## Session Chain (Confirmed)

1. `client -> server` init packet (raw, not framed)
2. `server -> client` frame `opcode=0x02 len=1 payload=80`
3. `client -> server` service negotiation (frame `opcode=0x0b`, payload includes `v8.service.Admin.Cluster` + `16.0`)
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

## Commands

### Cluster List

- **Request**: method `0x0b`.
- **Response**: method `0x0c`.
- **Chain**: init → service negotiation → RPC (`0x0b`) → close.
- **Parameters**: none.
- **Response body layout** (after `01 00 00 01 0c`):
  - `u8 count` (observed `0x01`).
  - Repeated record:
    - `uuid[16]` (raw bytes, no `0x16` prefix in response).
    - `u32_be expiration-timeout` (observed `0x0000003c` → 60).
    - `str8 host` (observed `alko-home`).
    - `u32_be unknown_0` (observed `0x00000000`).
    - `u16_be port` (observed `0x0605` → 1541).
    - `u64_be unknown_1` (observed `0x0000000000000000`).
    - `str8 name` (observed `Локальный кластер`).
    - `tail[32]` (8 x `u32` unknown; observed bytes
      `00000000 00000000 00000000 00000000 01000000 00010000 00000000 00000000`).
- **Evidence**: `logs/session_1771110767_483969_127_0_0_1_48522`.

### Cluster Info

- **Request**: method `0x0d`.
- **Response**: method `0x0e`.
- **Chain**: init → service negotiation → RPC (`0x0d`) → close.
- **Parameters**: `16 <cluster_uuid>`.
- **Response body layout** (after `01 00 00 01 0e`):
  - Single record in the same layout as `cluster list` (no leading count byte).
- **Evidence**: `logs/session_1771110778_484133_127_0_0_1_39376`.

### Manager List

- **Request**: `0x09` (context), then method `0x12`.
- **Response**: method `0x13`.
- **Chain**: init → negotiation → context (`0x09`) → RPC (`0x12`) → close.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of manager records containing manager UUIDs and strings (host/name).
- **Evidence**: `logs/session_1771103984_389177_127_0_0_1_37414`.

### Manager Info

- **Request**: `0x09` (context), then method `0x14`.
- **Response**: method `0x15`.
- **Parameters**: `16 <cluster_uuid> <manager_uuid>`.
- **Response fields** (hypothesis): manager record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103991_389768_127_0_0_1_49396`.

### Server List

- **Request**: `0x09` (context), then method `0x16`.
- **Response**: method `0x17`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of server records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103985_389222_127_0_0_1_37426`.

### Server Info

- **Request**: `0x09` (context), then method `0x18`.
- **Response**: method `0x19`.
- **Parameters**: `16 <cluster_uuid> <server_uuid>`.
- **Response fields** (hypothesis): server record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103992_389824_127_0_0_1_49406`.

### Process List

- **Request**: `0x09` (context), then method `0x1d`.
- **Response**: method `0x1e`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of process records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103985_389267_127_0_0_1_37442`.

### Process Info

- **Request**: `0x09` (context), then method `0x1f`.
- **Response**: method `0x20`.
- **Parameters**: `16 <cluster_uuid> <process_uuid>`.
- **Response fields** (hypothesis): process record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103993_389869_127_0_0_1_49412`.

### Infobase Summary List

- **Request**: `0x09` (context), then method `0x2a`.
- **Response**: method `0x2b`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response body layout** (after `01 00 00 01 2b`):
  - `u8 count` (observed `0x01`).
  - Repeated record:
    - `uuid[16]` (raw bytes).
    - `u8 tag` (observed `0x2c`, meaning unknown).
    - `str8 descr` (observed `Description`).
    - `str8 name` (observed `yaxunit`).
- **Evidence**: `logs/session_1771110787_484272_127_0_0_1_34530`.

### Infobase Info

- **Request**: `0x09` (context), then method `0x30` (observed).
- **Response**: method `0x31`.
- **Parameters**: `16 <cluster_uuid> <infobase_uuid>`.
- **Response body layout** (after `01 00 00 01 31`), observed sequence:
  - `uuid[16]`.
  - `u8 tag` (observed `0x2c`).
  - `u32_be` (observed `0x00000000`).
  - `str8 dbms` (observed `PostgreSQL`).
  - `str8 name` (observed `yaxunit`).
  - `str8 unknown_str` (len=3, bytes `ef bf bd`).
  - `str8 db-server` (observed `localhost`).
  - `str8 db-user` (observed `postgres`).
  - `str8 empty` (len=0).
  - `str8 len=2` (bytes `45 3c`, shown as `E<`).
  - `bytes[4]` (observed `03 b5 78 00`, not UTF-8).
  - `str8 denied-message` (observed `Message`).
  - `str8 denied-parameter` (observed `PARAMETER`).
  - `str8 empty` (len=0).
  - `str8 len=2` (bytes `46 4a`, shown as `FJ`).
  - `u32_be` (observed `0xfc124000`).
  - `str8 descr` (observed `Description`).
  - `str8 unknown_str` (observed `ru_RU`).
  - `str8 db-name` (observed `yaxunit`).
  - `str8 permission-code` (observed `CODE`).
  - `tail[28]` (7 x `u32` unknown; observed bytes
    `00000000 00000000 00010000 00000000 000003e7 00000378 00000309`).
- **Evidence**: `logs/session_1771110795_484387_127_0_0_1_41870`.

### Infobase Summary Info

- **Request**: `0x09` (context), then method `0x2e`.
- **Response**: method `0x2f`.
- **Parameters**: `16 <cluster_uuid> <infobase_uuid>`.
- **Response fields** (hypothesis): infobase summary record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103996_390065_127_0_0_1_49450`.

### Infobase Info

- **Request**: `0x09` (context), `0x0a` (infobase context), then method `0x30`.
- **Response**: method `0x31`.
- **Parameters**: `16 <cluster_uuid> <infobase_uuid>`.
- **Response fields** (hypothesis): infobase record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103995_390019_127_0_0_1_49436`.

### Connection List

- **Request**: `0x09` (context), then method `0x32`.
- **Response**: method `0x33`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of connection records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103987_389408_127_0_0_1_37462`.

### Connection Info

- **Request**: `0x09` (context), then method `0x36`.
- **Response**: method `0x37`.
- **Parameters**: `16 <cluster_uuid> <connection_uuid>`.
- **Response fields** (hypothesis): connection record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103994_389914_127_0_0_1_49422`.

### Session List

- **Request**: `0x09` (context), then method `0x41`.
- **Response**: method `0x42`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of session records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103987_389453_127_0_0_1_37466`.

### Session Info

- **Request**: `0x09` (context), then method `0x45`.
- **Response**: method `0x46`.
- **Parameters**: `16 <cluster_uuid> <session_uuid>`.
- **Response fields** (hypothesis): session record (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103994_389974_127_0_0_1_49430`.

### Lock List

- **Request**: `0x09` (context), then method `0x48`.
- **Response**: method `0x49`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of lock records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103988_389509_127_0_0_1_37480`.

### Profile List

- **Request**: `0x09` (context), then method `0x59`.
- **Response**: method `0x5a`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of profile records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103989_389623_127_0_0_1_37500`.

### Counter List

- **Request**: `0x09` (context), then method `0x76`.
- **Response**: method `0x77`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of counter records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103990_389678_127_0_0_1_49366`.

### Limit List

- **Request**: `0x09` (context), then method `0x7c`.
- **Response**: method `0x7d`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of limit records (UUID + strings + numeric fields).
- **Evidence**: `logs/session_1771103991_389723_127_0_0_1_49380`.

### Agent Version

- **Request**: method `0x87`.
- **Response**: method `0x88`.
- **Parameters**: none.
- **Response fields**:
  - `str8 version` (confirmed; example: `8.5.1.1150`).
- **Evidence**: `logs/session_1771103983_389122_127_0_0_1_37406`.
