# RAC Connection List Record Format (Observed)

Source capture:
- `logs/session_1771281887_2714678_127_0_0_1_59850/server_to_client.stream.bin`

RAC output reference:
- `/tmp/rac_connection_list_cluster_valid.out`

## Fields From `rac` Output

Observed field names in `rac connection list` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture |
|---|---|---|---|
| `connection` | UUID | yes | 1 |
| `application` | string | yes | 2 |
| `connected-at` | datetime (u64 ticks, 100us since 0001-01-01) | yes | 3 (inside `blocked_by_ls + connected_at`) |
| `conn-id` | u32 | yes | 3 |
| `host` | string | yes | 4 |
| `infobase` | UUID (nullable) | yes | 5 |
| `process` | UUID | yes | 6 |
| `session-number` | u32 | yes | 7 (tail) |
| `blocked-by-ls` | u32 | hypothesis (maps to `blocked_by_ls`) | 3 (inside `blocked_by_ls`) |

## RPC Envelope

Response method: `0x33` (`connection list`)

Payload structure (method body):
- offset `0x00`: `count:u8` (observed `0x06`)
- offset `0x01`: first record starts here
- subsequent records start at the next connection UUID occurrence (variable length records)

## Record Layout (Observed)

Offsets are relative to the start of a record.

- `0x00` `connection_uuid[16]`
- `0x10` `app_len:u8`
- `0x11` `app[app_len]` (UTF-8, observed `RAS`, `AgentStandardCall`, `1CV8C`, `SystemBackgroundJob`)
- `0x11 + app_len` `blocked_by_ls:u32_be` (hypothesis)
  - observed `0x00000000`
- `0x15 + app_len` `connected_at_ticks:u64_be`
  - 100 microsecond ticks since `0001-01-01 00:00:00`
- `0x1d + app_len` `conn_id:u32_be`
  - example: `0x0000092b` => `2347`
- `0x21 + app_len` `host_len:u8`
- `0x22 + app_len` `host[host_len]` (UTF-8, observed `alko-home`)
- `0x22 + app_len + host_len` `infobase_uuid[16]`
  - can be all zeros when absent
- `0x32 + app_len + host_len` `process_uuid[16]`
- `0x42 + app_len + host_len` `session_number:u32_be`
  - observed `0x00000000`, `0x00000005`
  - matches `session-number` from `rac` output

## Record 1 Example (Offsets)

From the first record in the capture (connection `c030e65d-680a-41ed-a15a-6b859025f0b7`):

- `0x00..0x0f` connection UUID
- `0x10` app_len = `0x03`
- `0x11..0x13` app = `RAS`
- `0x14..0x17` blocked_by_ls (hypothesis) = `00 00 00 00`
- `0x18..0x1f` connected_at_ticks = `00 02 45 3a 9e af 75 60`
- `0x20..0x23` conn_id = `00 00 09 2b`
- `0x24` host_len = `0x09`
- `0x25..0x2d` host = `alko-home`
- `0x2e..0x3d` infobase UUID = `717bdda7-2f60-4577-b262-f1fc8c0e472c`
- `0x3e..0x4d` process UUID = `f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5`
- `0x4e..0x51` tail_u32 = `00 00 00 00`

## Open Questions

- Confirm `blocked_by_ls` by finding a non-zero value in a capture.
