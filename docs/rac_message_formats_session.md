# RAC Message Formats: Session API

Derived from `docs/rac_message_formats.md`.

## Commands

### Session List

- **Request**: `0x09` (context), then method `0x41`.
- **Response**: method `0x42`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response fields** (hypothesis): list of session records (UUID + strings + numeric fields).
- **Response example**: `artifacts/session_list_response.hex`.
- **Evidence**: `logs/session_1771166394_1157675_127_0_0_1_49430`.

Confirmed from `artifacts/session_list_response.hex`:

- `u8 count` at `0x05` = `3`.
- `str8` fields mapped to `rac` output names (offset -> value -> field):
  - `0x17` -> `1CV8C` -> `app-id`
  - `0x8e`, `0x2e2`, `0x544` -> `alko-home` -> `host`
  - `0x268` -> `Designer` -> `app-id`
  - `0x4bf` -> `SystemBackgroundJob` -> `app-id`
  - `0x1d2`, `0x429`, `0x592` -> `DefUser` -> `user-name`
  - `0x1b3` -> `ru` -> `locale`
  - `0x407`, `0x570` -> `ru_RU` -> `locale`
  - `0x24e`, `0x4a5` -> `127.0.0.1` -> `client-ip`
  - `0xba`, `0x30e` -> `file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic` -> unknown string (license path, not present in `rac` output)
  - `0x17c` -> `381094` -> unknown string (likely license metadata)
  - `0x3d0` -> `376212` -> unknown string (likely license metadata)
  - `0x187`, `0x3db` -> `500000025347` -> unknown string (likely license metadata)
- `uuid[16]` mapped to `rac` output names:
  - `0x06` -> `25510e27-f24a-4586-9ac9-9f7837c0dea1` -> `session`
  - `0x257` -> `56bde8c0-d008-4d33-a6b9-8db9b6f82de5` -> `session`
  - `0x4ae` -> `eb61231d-7bee-4a06-8869-41f70e2289de` -> `session`
  - `0x97`, `0x2eb`, `0x54d` -> `717bdda7-2f60-4577-b262-f1fc8c0e472c` -> `infobase`
  - `0x294` -> `94c6bd33-8041-42c6-87b4-53f735d9198c` -> `connection`
  - `0x40c`, `0x575` -> `f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5` -> `process`
  - `0x4f6` -> `f16db2e2-a24c-4b72-843a-43af5bd87ed8` -> `connection`

Hypotheses:

- A long UTF-8 block containing license metadata (starts near `0xf3` with `Клиент, ... 19.02.2026 22:10:50 (UTC), file:///...`) is a separate string field but its length prefix is not yet isolated.
- Numerous numeric fields (`u32`/`u64`) exist between strings; need record boundary confirmation to label them.

Numeric fields (record 1, `u32_be` matches `rac` output values; offsets tentative):

- `0x28` -> `bytes-all` = `235586`
- `0x34` -> `calls-all` = `366`
- `0x54` -> `dbms-bytes-all` = `745095`
- `0xb0` -> `passive-session-hibernate-time` = `1200`
- `0xb4` -> `hibernate-session-terminate-time` = `86400`
- `0x6d` -> `duration-all` = `15817` (unaligned; verify)
- `0x71` -> `duration-all-dbms` = `201` (unaligned; verify)
- `0x1ed` -> `memory-total` = `302372987`
- `0x205` -> `read-total` = `19653880`
- `0x21d` -> `write-total` = `13360861`
- `0x22d` -> `duration-all-service` = `120`
- `0x246` -> `cpu-time-total` = `3047`

Decoding notes (short):

- Payload = `01 00 00 01 42` + `u8 count` + repeated records.
- Strings are `str8` (1-byte length + UTF-8 bytes).
- UUIDs are raw 16 bytes (no marker).
- Numeric fields appear as big-endian integers (`u32_be`/`u64_be`); see `bytes-all`, `calls-all`, and other counters in `rac` output to match offsets.

Record boundaries (from `artifacts/session_list_response.hex`):

- `record[0]` starts at `0x06` (session UUID) and ends at `0x256`, length `0x251`.
- `record[1]` starts at `0x257` (session UUID) and ends at `0x4ad`, length `0x257`.
- `record[2]` starts at `0x4ae` (session UUID) and ends at `0x60d`, length `0x160`.
- Boundary rule (validated on this capture): each record begins with `session` UUID; the next record starts at the next `session` UUID; the last record ends at payload end.

Relative offsets (from record start, for this capture):

- `record[0] session` UUID at `+0x00`.
- `record[0] app-id` (`1CV8C`) at `+0x11`.
- `record[0] host` (`alko-home`) at `+0x88`.
- `record[0] infobase` UUID at `+0x91`.
- `record[0] locale` (`ru`) at `+0x1ad`.
- `record[0] user-name` (`DefUser`) at `+0x1cc`.
- `record[0] client-ip` (`127.0.0.1`) at `+0x248`.
- `record[1] session` UUID at `+0x00`.
- `record[1] connection` UUID at `+0x3d`.
- `record[1] app-id` (`Designer`) at `+0x11`.
- `record[1] host` (`alko-home`) at `+0x8b`.
- `record[1] infobase` UUID at `+0x94`.
- `record[1] process` UUID at `+0x1b5`.
- `record[1] locale` (`ru_RU`) at `+0x1b0`.
- `record[1] user-name` (`DefUser`) at `+0x1d2`.
- `record[1] client-ip` (`127.0.0.1`) at `+0x24e`.
- `record[2] session` UUID at `+0x00`.
- `record[2] connection` UUID at `+0x48`.
- `record[2] app-id` (`SystemBackgroundJob`) at `+0x11`.
- `record[2] host` (`alko-home`) at `+0x96`.
- `record[2] infobase` UUID at `+0x9f`.
- `record[2] process` UUID at `+0xc7`.
- `record[2] locale` (`ru_RU`) at `+0xc2`.
- `record[2] user-name` (`DefUser`) at `+0xe4`.

Record[1] sequence line with gaps (relative offsets):

- `record[1] +0x00..+0x0f` `session` UUID.
- `gap +0x10..+0x10` (1 byte).
- `record[1] +0x11..+0x19` `app-id` (`Designer`) `str8`.
- `gap +0x1a..+0x3c` (0x23 bytes): `bytes-last-5min` = `685` at `+0x2d` (u32_be); `calls-last-5min` = `10` at `+0x39` (u32_be) (ambiguous, value repeats).
- `record[1] +0x3d..+0x4c` `connection` UUID.
- `gap +0x4d..+0x8a` (0x3e bytes): `dbms-bytes-all` = `654414` at `+0x51` (u32_be); `calls-last-5min` = `10` at `+0x69` (u32_be) (ambiguous); `duration-all-dbms` = `85` at `+0x6e` (u32_be).
- `record[1] +0x8b..+0x94` `host` (`alko-home`) `str8`.
- `record[1] +0x95..+0xa4` `infobase` UUID.
- `gap +0xa5..+0x1af` (0x10c bytes): likely contains timestamps and most counters (`started-at`, `last-active-at`, `bytes-all`, `calls-all`, `duration-all`, `memory-*`); not yet isolated.
- `record[1] +0x1b0..+0x1b5` `locale` (`ru_RU`) `str8`.
- `record[1] +0x1b5..+0x1c4` `process` UUID.
- `gap +0x1c5..+0x1d1` (0x0d bytes): `session-id` = `1` at `+0x1c5` (u32_be).
- `record[1] +0x1d2..+0x1d9` `user-name` (`DefUser`) `str8`.
- `gap +0x1da..+0x24d` (0x74 bytes): `duration-last-5min-service` = `6` at `+0x1e3` (u32_be); `cpu-time-last-5min` = `5` at `+0x1ea` (u32_be) (ambiguous, repeats at `+0x23e`, `+0x245`); `read-total` = `1294878` at `+0x205` (u32_be); `write-total` = `1356665` at `+0x21d` (u32_be); `calls-last-5min` = `10` at `+0x229` (u32_be) (ambiguous).
- `record[1] +0x24e..+0x257` `client-ip` (`127.0.0.1`) `str8`.

Record[0] sequence line with gaps (relative offsets):

- `record[0] +0x00..+0x0f` `session` UUID.
- `gap +0x10..+0x10` (1 byte).
- `record[0] +0x11..+0x16` `app-id` (`1CV8C`) `str8`.
- `gap +0x17..+0x87` (0x71 bytes):
  - `bytes-all` = `235586` at `+0x22` (u32_be) (load 1CV8C: `7807077`).
  - `bytes-last-5min` = `0` at `+0x2a` (u32_be) (load 1CV8C: `7563545`).
  - `calls-all` = `366` at `+0x2e` (u32_be) (load 1CV8C: `6514`).
  - `calls-last-5min` = `0` at `+0x36` (u32_be) (load 1CV8C: `6139`).
  - `dbms-bytes-all` = `745095` at `+0x4e` (u32_be) (load 1CV8C: `10914466`).
  - `dbms-bytes-last-5min` = `0` at `+0x56` (u32_be) (load 1CV8C: `9969187`).
  - `duration-all` = `15817` at `+0x67` (u32_be) (load 1CV8C: `168659`).
  - `duration-all-dbms` = `201` at `+0x6b` (u32_be) (load 1CV8C: `6944`).
  - `duration-last-5min` = `0` at `+0x7b` (u32_be) (load 1CV8C: `152700`).
  - `duration-last-5min-dbms` = `0` at `+0x83` (u32_be) (load 1CV8C: `6694`).
- `record[0] +0x88..+0x91` `host` (`alko-home`) `str8`.
- `record[0] +0x92..+0xa1` `infobase` UUID.
- `gap +0xa1..+0x1ac` (0x10c bytes): `passive-session-hibernate-time` = `1200` at `+0xaa` (u32_be); `hibernate-session-terminate-time` = `86400` at `+0xae` (u32_be).
- `record[0] +0x1ad..+0x1af` `locale` (`ru`) `str8`.
- `gap +0x1b0..+0x1cb` (0x1c bytes): `session-id` = `3` at `+0x1bf` (u32_be) (repeat).
- `record[0] +0x1cc..+0x1d3` `user-name` (`DefUser`) `str8`.
- `gap +0x1d4..+0x247` (0x74 bytes):
  - `memory-last-5min` = `0` at `+0x1df` (u32_be) (load 1CV8C: `52244975`).
  - `memory-total` = `302372987` at `+0x1e7` (u32_be) (load 1CV8C: `378922812`).
  - `read-last-5min` = `0` at `+0x1f7` (u32_be) (load 1CV8C: `4441787`).
  - `read-total` = `19653880` at `+0x1ff` (u32_be) (load 1CV8C: `24095751`).
  - `write-last-5min` = `0` at `+0x20f` (u32_be) (load 1CV8C: `1386452`).
  - `write-total` = `13360861` at `+0x217` (u32_be) (load 1CV8C: `14747313`).
  - `duration-last-5min-service` = `0` at `+0x223` (u32_be) (load 1CV8C: `5413`).
  - `duration-all-service` = `120` at `+0x227` (u32_be) (load 1CV8C: `5563`).
  - `cpu-time-last-5min` = `0` at `+0x238` (u32_be) (load 1CV8C: `68587`).
  - `cpu-time-total` = `3047` at `+0x240` (u32_be) (load 1CV8C: `71702`).
- `record[0] +0x248..+0x251` `client-ip` (`127.0.0.1`) `str8`.

Record[2] sequence line with gaps (relative offsets):

- `record[2] +0x00..+0x0f` `session` UUID.
- `gap +0x10..+0x10` (1 byte).
- `record[2] +0x11..+0x24` `app-id` (`SystemBackgroundJob`) `str8`.
- `gap +0x25..+0x47` (0x23 bytes).
- `record[2] +0x48..+0x57` `connection` UUID.
- `gap +0x58..+0x95` (0x3e bytes): `dbms-bytes-all` = `3088` at `+0x5c` (u32_be); `duration-all-dbms` = `2` at `+0x79` (u32_be).
- `record[2] +0x96..+0x9f` `host` (`alko-home`) `str8`.
- `record[2] +0xa0..+0xaf` `infobase` UUID.
- `gap +0xb0..+0xc1` (0x13 bytes): `passive-session-hibernate-time` = `1200` at `+0xb8` (u32_be); `hibernate-session-terminate-time` = `86400` at `+0xbc` (u32_be).
- `record[2] +0xc2..+0xc7` `locale` (`ru_RU`) `str8`.
- `record[2] +0xc7..+0xd6` `process` UUID.
- `gap +0xd7..+0xe3` (0x0d bytes): `session-id` = `5` at `+0xd7` (u32_be).
- `record[2] +0xe4..+0xeb` `user-name` (`DefUser`) `str8`.
- `gap +0xec..+0x15f` (0x74 bytes): `memory-current` = `658205` at `+0xef` (u32_be); `duration-all-dbms` = `2` at `+0x159` (u32_be) (repeat).


### Session Info

- **Request**: `0x09` (context), then method `0x45`.
- **Response**: method `0x46`.
- **Parameters**: `16 <cluster_uuid> <session_uuid>`.
- **Response fields** (hypothesis): session record (UUID + strings + numeric fields).
- **Response example**: `artifacts/session_info_response.hex`.
- **Response example (load)**: `artifacts/session_info_response_load.hex`.
- **Response example (load 1CV8C)**: `artifacts/session_info_response_1cv8c.hex`.
- **Evidence**: `logs/session_1771168783_1186511_127_0_0_1_54594`.

Confirmed from `artifacts/session_info_response.hex` (relative to record start at `0x05`):

- `session` UUID at `+0x00`.
- `app-id` (`Designer`) `str8` at `+0x10` (len=8).
- `connection` UUID at `+0x3d`.
- `host` (`alko-home`) `str8` at `+0x8a`.
- `infobase` UUID at `+0x94`.
- `locale` (`ru_RU`) `str8` at `+0x1af`.
- `process` UUID at `+0x1b5`.
- `session-id` = `1` at `+0x1c5` (u32_be).
- `user-name` (`DefUser`) `str8` at `+0x1d1`.
- `client-ip` (`127.0.0.1`) `str8` at `+0x24d`.

Gaps with decoded counters (relative offsets):

- `gap +0x19..+0x3c`:
  - `bytes-all` = `253146` at `+0x25` (u32_be).
  - `bytes-last-5min` = `685` at `+0x2d` (u32_be).
  - `calls-all` = `3616` at `+0x31` (u32_be).
  - `calls-last-5min` = `10` at `+0x39` (u32_be) (value repeats elsewhere).
- `gap +0x4d..+0x89`:
  - `dbms-bytes-all` = `654414` at `+0x51` (u32_be).
  - `dbms-bytes-last-5min` = `0` at `+0x59` (u32_be) (confirmed by load capture: `19126411`).
  - `calls-last-5min` = `10` at `+0x69` (u32_be) (repeat).
  - `duration-all` = `2792` at `+0x6a` (u32_be).
  - `duration-all-dbms` = `85` at `+0x6e` (u32_be).
  - `duration-last-5min` = `13` at `+0x7e` (u32_be).
  - `duration-last-5min-dbms` = `0` at `+0x86` (u32_be) (confirmed by load capture: `369`).
- `gap +0xa5..+0x1ae`:
  - `passive-session-hibernate-time` = `1200` at `+0xad` (u32_be).
  - `hibernate-session-terminate-time` = `86400` at `+0xb1` (u32_be).
- `gap +0x1d9..+0x24c`:
  - `memory-last-5min` = `413513` at `+0x1e5` (u32_be).
  - `memory-total` = `87297290` at `+0x1ed` (u32_be).
  - `read-last-5min` = `0` at `+0x1fd` (u32_be) (confirmed by load capture: `38445445`).
  - `read-total` = `1294878` at `+0x205` (u32_be).
  - `write-last-5min` = `0` at `+0x215` (u32_be) (confirmed by load capture: `38781554`).
  - `write-total` = `1356665` at `+0x21d` (u32_be).
  - `duration-last-5min-service` = `8` at `+0x229` (u32_be).
  - `duration-all-service` = `1922` at `+0x22d` (u32_be).
  - `cpu-time-last-5min` = `5` at `+0x23e` (u32_be) (confirmed by load capture: `2760`).
  - `cpu-time-total` = `1357` at `+0x246` (u32_be).

Diff vs load capture (same session, higher activity):

- `bytes-all` `+0x25`: `253146` -> `1422688`.
- `bytes-last-5min` `+0x2d`: `685` -> `1169405`.
- `calls-all` `+0x31`: `3616` -> `15020`.
- `calls-last-5min` `+0x39`: `10` -> `11402`.
- `dbms-bytes-all` `+0x51`: `654414` -> `19780825`.
- `dbms-bytes-last-5min` `+0x59`: `0` -> `19126411`.
- `duration-all` `+0x6a`: `2792` -> `6549`.
- `duration-all-dbms` `+0x6e`: `85` -> `454`.
- `duration-last-5min` `+0x7e`: `13` -> `3755`.
- `duration-last-5min-dbms` `+0x86`: `0` -> `369`.
- `memory-last-5min` `+0x1e5`: `413513` -> `400598`.
- `memory-total` `+0x1ed`: `87297290` -> `87708059`.
- `read-last-5min` `+0x1fd`: `0` -> `38445445`.
- `read-total` `+0x205`: `1294878` -> `39740323`.
- `write-last-5min` `+0x215`: `0` -> `38781554`.
- `write-total` `+0x21d`: `1356665` -> `40138219`.
- `duration-last-5min-service` `+0x229`: `8` -> `18`.
- `duration-all-service` `+0x22d`: `1922` -> `1942`.
- `cpu-time-last-5min` `+0x23e`: `5` -> `2760`.
- `cpu-time-total` `+0x246`: `1357` -> `4118`.

Load capture for `app-id=1CV8C` (same layout, different activity):

- `bytes-all` `+0x22`: `7807077`.
- `bytes-last-5min` `+0x2a`: `7563545`.
- `calls-all` `+0x2e`: `6514`.
- `calls-last-5min` `+0x36`: `6139`.
- `dbms-bytes-all` `+0x4e`: `10914466`.
- `dbms-bytes-last-5min` `+0x56`: `9969187`.
- `duration-all` `+0x67`: `168659`.
- `duration-all-dbms` `+0x6b`: `6944`.
- `duration-last-5min` `+0x7b`: `152700`.
- `duration-last-5min-dbms` `+0x83`: `6694`.
- `memory-last-5min` `+0x1df`: `52244975`.
- `memory-total` `+0x1e7`: `378922812`.
- `read-last-5min` `+0x1f7`: `4441787`.
- `read-total` `+0x1ff`: `24095751`.
- `write-last-5min` `+0x20f`: `1386452`.
- `write-total` `+0x217`: `14747313`.
- `duration-last-5min-service` `+0x223`: `5413`.
- `duration-all-service` `+0x227`: `5563`.
- `cpu-time-last-5min` `+0x238`: `68587`.
- `cpu-time-total` `+0x240`: `71702`.
