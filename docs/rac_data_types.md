# RAC Data Types

This document describes the data types currently used by the RAC session decoder (`apps/rac_protocol/src/codec.rs`).

## Scalar Types

| Type | Size (bytes) | Encoding | Notes |
| --- | ---: | --- | --- |
| `u8` | 1 | Unsigned 8-bit | Raw byte. |
| `u16_le` | 2 | Little-endian | Used for some license limits. |
| `u16_be` | 2 | Big-endian | Available in codec helpers. |
| `u32_be` | 4 | Big-endian | Most counters and IDs. |
| `u64_be` | 8 | Big-endian | High-volume counters and timestamps. |
| `i32_be` | 4 | Big-endian | Helper exists; not used in the session decoder at the moment. |
| `bool` | 1 | `u8` | `0x00` = false, non-zero = true. |

## Composite Types

| Type | Size (bytes) | Encoding | Notes |
| --- | ---: | --- | --- |
| `uuid16` | 16 | Raw bytes | Parsed as RFC4122 UUID without byte reordering. |
| `str8` | `1 + N` | `u8` length + UTF-8 bytes | Length is a single byte. Empty string has length 0. |
| `datetime` | 8 | `u64_be` | 1C timestamp. See conversion below. |

## Optional `str8` in RecordCursor

`RecordCursor::take_str8_opt` accepts the same layout as `str8`, but it also tolerates a single marker byte `0x01` immediately after the length. When present, the decoder skips that marker and then reads `N` bytes as UTF-8.

## 1C Timestamp Conversion

The decoder converts 1C timestamps to ISO strings using:

- Unit: `1/10000` second
- Epoch offset: `621355968000000`
- `unix_seconds = (raw - 621355968000000) / 10000`
- Output format: `YYYY-MM-DDTHH:MM:SS`

## Optional Reads and Truncation

All `*_opt` reads return `None` if the payload is truncated. This is how the session decoder handles shorter records without failing the whole parse.
