# Known Data Types (RAC)

Use this when inferring fields from gaps and sizes.

| Type | Size | Notes |
|---|---|---|
| `u8` | 1 | unsigned byte |
| `u16_be` | 2 | big-endian |
| `u32_be` | 4 | big-endian |
| `u64_be` | 8 | big-endian |
| `UUID` | 16 | 128-bit |
| `bool` | 1 | typically `0x00`/`0x01` |
| `datetime` | 8 | `u64_be` ticks, 100us since `0001-01-01 00:00:00` |
| `string` | 1 + N | `u8` length prefix + UTF-8 bytes (observed) |

Notes:
- Validate string encoding per capture (UTF-8 observed so far).
- Nullable UUID is 16 bytes of `00`.
