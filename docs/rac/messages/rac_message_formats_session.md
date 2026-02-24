# RAC Message Formats: Session API

Protocol version (service negotiation): `v8.service.Admin.Cluster` `16.0` (observed in captures).


Aligned with current decoder implementation in `apps/rac_protocol/src/commands/session.rs`.

## Sources

- `artifacts/rac/session_list_response.hex`
- `artifacts/rac/session_info_response.hex`
- `artifacts/rac/session_info_response_load.hex`
- `artifacts/rac/session_info_response_1cv8c.hex`
- `artifacts/rac/session_info_response_1cv8c_dbproc.hex`

## Commands

### Session List

- **Request**: `0x09` (context), then method `0x41`.
- **Response**: method `0x42`.
- **Parameters**: `16 <cluster_uuid>`.
- **Response body layout**: `u8 count` followed by `count` session records.

**Record boundary detection (current decoder):**

- Each record starts with a `uuid16` that passes a RFC4122 sanity check.
- Immediately after the UUID is `app-id` encoded as `str8` and validated against a conservative ASCII whitelist.
- The list decoder scans the body for the next valid record start to split records.

### Session Info

- **Request**: `0x09` (context), then method `0x45`.
- **Response**: method `0x46`.
- **Parameters**: `16 <cluster_uuid> <session_uuid>`.
- **Response body layout**: a single session record using the same layout as in Session List.

## Session Record Layout (decoded order)

Fields are read sequentially by the decoder. When the payload ends early, the decoder fills missing fields with defaults (empty string, `0`, `false`, or zero UUID).

| Field | Type | Optional | Notes | Version |
| --- | --- | --- | --- | --- |
| `session` | `uuid16` | no | Record start anchor. | 11.0 |
| `app-id` | `str8` | no | Examples: `Designer`, `1CV8C`, `SystemBackgroundJob`. | 11.0 |
| `counters.blocked-by-dbms` | `u32_be` | yes | Present in the stream, but still being interpreted. Missing values default to `0`. | 11.0 |
| `counters.blocked-by-ls` | `u32_be` | yes | Present in the stream, but still being interpreted. Missing values default to `0`. | 11.0 |
| `counters.bytes-all` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.bytes-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.calls-all` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.calls-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `connection` | `uuid16` | yes | Missing values default to zero UUID. | 11.0 |
| `counters.dbms-bytes-all` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.dbms-bytes-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `db-proc-info` | `str8` | yes | Only present when DB procedure info is reported. Missing values default to empty string. | 11.0 |
| `counters.db-proc-took` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `db-proc-took-at` | `datetime` | yes | 1C timestamp, decoded to ISO string. Missing values default to empty string. | 11.0 |
| `counters.duration-all` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-all-dbms` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-current` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-current-dbms` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-last-5min-dbms` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `host` | `str8` | yes | Missing values default to empty string. | 11.0 |
| `infobase` | `uuid16` | yes | Missing values default to zero UUID. | 11.0 |
| `last-active-at` | `datetime` | yes | 1C timestamp, decoded to ISO string. Missing values default to empty string. | 11.0 |
| `hibernate` | `bool` | yes | Currently treated as a boolean flag. Missing values default to `false`. | 11.0 |
| `counters.passive-session-hibernate-time` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.hibernate-session-terminate-time` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `license` | `license-block` | yes | Parsed by `parse_licenses`. Missing values default to an empty `SessionLicense`. | 11.0 |
| `locale` | `str8` | yes | Missing values default to empty string. | 11.0 |
| `process` | `uuid16` | yes | Missing values default to zero UUID. | 11.0 |
| `session-id` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `started-at` | `datetime` | yes | 1C timestamp, decoded to ISO string. Missing values default to empty string. | 11.0 |
| `user-name` | `str8` | yes | Missing values default to empty string. | 11.0 |
| `counters.memory-current` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.memory-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.memory-total` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.read-current` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.read-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.read-total` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.write-current` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.write-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.write-total` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-current-service` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-last-5min-service` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.duration-all-service` | `u32_be` | yes | Missing values default to `0`. | 11.0 |
| `current-service-name` | `str8` | yes | Missing values default to empty string. | 11.0 |
| `counters.cpu-time-current` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.cpu-time-last-5min` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `counters.cpu-time-total` | `u64_be` | yes | Missing values default to `0`. | 11.0 |
| `data-separation` | `str8` | yes | Missing values default to empty string. | 11.0 |
| `client-ip` | `str8` | yes | Missing values default to empty string. | 11.0 |

### License Block Layout (decoded order)

The decoder reads the license block immediately after the hibernate timers.

| Field | Type | Optional | Notes | Version |
| --- | --- | --- | --- | --- |
| `licenses-count` | `u8` | no | Number of license entries in the payload. | 11.0 |
| `file-name` | `str8` | yes | Only the first entry is decoded. Missing values default to empty string. | 11.0 |
| `full-presentation` | `str8` | yes | Only the first entry is decoded. Missing values default to empty string. | 11.0 |
| `issued-by-server` | `bool` | yes | Only the first entry is decoded. Missing values default to `false`. | 11.0 |
| `license-type` | `u32_be` | yes | Only the first entry is decoded. Missing values default to `0`. | 11.0 |
| `max-users-all` | `u32_be` | yes | Only the first entry is decoded. Missing values default to `0`. | 11.0 |
| `max-users-current` | `u32_be` | yes | Only the first entry is decoded. Missing values default to `0`. | 11.0 |
| `network-key` | `bool` | yes | Only the first entry is decoded. Missing values default to `false`. | 11.0 |
| `server-address` | `str8` | yes | Only the first entry is decoded. Missing values default to empty string. | 11.0 |
| `process-id` | `str8` | yes | Only the first entry is decoded. Missing values default to empty string. | 11.0 |
| `server-port` | `u32_be` | yes | Only the first entry is decoded. Missing values default to `0`. | 11.0 |
| `key-series` | `str8` | yes | Only the first entry is decoded. Missing values default to empty string. | 11.0 |
| `brief-presentation` | `str8` | yes | Only the first entry is decoded. Missing values default to empty string. | 11.0 |

Notes:

- The top-level fields `retrieved-by-server`, `software-license`, `network-key` in `SessionRecord` default to `false` when absent.
- Empty strings for `file-name` and `process-id` are preserved as empty strings by the decoder.
- The `SessionLicense.software_license` field is currently hard-coded to `false` by the decoder.

## Decoder Behaviors Worth Noting

- All reads are sequential; there is no random access or fixed offsets in the current parser.
- `Session List` decoding uses a heuristic to find record boundaries; if the record count is present but fewer valid starts are found, decoding fails.
- For counters, the type widths in this document match the current implementation (even where wider types may be a future adjustment).

## Known Payload Types

See `docs/rac/documentation/rac_data_types.md` for data type encodings and sizes used by the session decoder.
