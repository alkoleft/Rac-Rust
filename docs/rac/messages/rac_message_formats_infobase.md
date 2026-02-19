# RAC Message Formats: Infobase API

Derived from `docs/rac/messages/rac_message_formats.md`.

## Commands

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
