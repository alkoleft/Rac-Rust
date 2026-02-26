---
name: onec-protocol-reverse
description: Reverse engineer 1C wire protocols (RAC and adjacent admin/service protocols) from traffic captures and build replacement clients. Use when the user asks to analyze unknown 1C binary payloads, map request/response methods, decode framing, compare command traces, or implement tooling that works without official `rac`.
---

# Onec Protocol Reverse

Use this workflow for unknown 1C protocols and for extending known RAC protocol mapping.

## Quick Start

1. Build tools in the current workspace:
   - `cargo build --release`
2. Capture one command/session:
   - Use `scripts/rac/capture_rac_command.sh` in this skill for RAC commands.
   - Raw captures go under `./logs/`. Extracted byte sequences go under `./artifacts/rac/`.
   - Or run equivalent proxy + client workflow for a different protocol.
3. Decode streams:
   - `cargo run -p rac_cli --bin rac_decode -- <session>/client_to_server.stream.bin`
   - `cargo run -p rac_cli --bin rac_decode -- <session>/server_to_client.stream.bin`
4. Update method/framing notes using templates in `references/`.
5. Validate hypotheses by implementing minimal live client calls (`rac_lite` style).

## Workflow

1. Read `references/reverse_workflow.md` at the start of each new protocol task.
2. If protocol is RAC or RAC-like, read `references/rac_baseline.md`.
3. If mapping methods, read `references/rac_method_map.md`.
4. Before final conclusions, read `references/pitfalls_checklist.md`.
5. If task is static reverse/decomp-style analysis of `rac`, read `references/rac_static_reverse.md`.
6. If documenting message formats, use `references/message_format_template.md`.
7. If inferring fields from gaps, read `references/data_types.md`.
6. Execute capture matrix:
   - Baseline list/info commands.
   - Same command with different entity IDs.
   - Empty-result vs non-empty-result cases.
7. Infer transport framing before field-level decoding:
   - Check fixed header markers.
   - Test `u8 len` vs varuint.
   - Verify boundaries against full stream length.
8. Infer RPC envelope:
   - Identify stable prefixes.
   - Extract method IDs.
   - Separate context-setting calls from business calls.
9. Build/extend replacement client:
   - Implement handshake.
   - Implement one method end-to-end.
   - Re-test against live server.

## Scripts

- `scripts/rac/capture_rac_command.sh`
  - Deterministic single-command capture via local proxy.
- `scripts/rac/method_map_from_sessions.sh`
  - Batch extract method IDs from session directories using `rac_decode`.
- `scripts/rac/map_rac_commands_to_methods.sh`
  - Run a capture matrix and generate `docs/rac/documentation/rac_cli_method_map.generated.md`.
- `scripts/rac/dump_rac_static_info.sh`
  - Dump static metadata from `rac` binary (ELF profile, debuglink, IDS keys, supported modes).
- `docs/rac/documentation/rac_help_methods.md`
  - Quick reference for `rac help <mode>` commands, parameters, and descriptions.

Run scripts from repository root unless user asks otherwise. Captures are written to `./logs/` by default via `LOG_DIR`.

## Extracting Response Examples

Use `scripts/rac/extract_rac_response_example.sh` to save response payload bytes into `./artifacts/rac/` for documentation.

## Capture/Decode Practices (Chat-Derived)

- Keep raw capture data in `./logs/`. Do not cite `logs/` as durable evidence; extract only the required byte sequences into `./artifacts/rac/` and reference those in docs.
- When listing evidence in docs, prefer `artifacts/rac/<label>.hex` over `logs/session_*`.
- If a proxy listen port is in use, change `LISTEN_ADDR` (e.g., `127.0.0.1:1566`) before retrying.
- For `rac_decode`, the correct invocation in this repo is:
  - `cargo run -p rac_cli --quiet --bin rac_decode -- <stream.bin>`
- For multi-record list responses:
  - Determine record boundaries by locating repeating entity UUIDs (e.g., `session` UUID).
  - Report offsets relative to each record start.
  - Build a “sequence line”: `field → gap → field → gap`, and search gaps for `u32_be` values from `rac` output.
  - Use multiple captures with activity (“load”) to turn zeros into non-zero values and confirm field offsets.

## Output Requirements

When reporting protocol analysis:

1. Present confirmed facts first (bytes, offsets, method IDs, frame format).
2. Mark hypotheses explicitly.
3. Include concrete file references to captures.
4. Record new mappings in `references/` for reuse.
5. If building replacement behavior, show one verified live command/output.
6. For message format docs, use this block order and content:
   - `Поля ответа (из rac)`: table with `Field | Type | Found In Capture | Order In Capture`, and preserve on-wire order.
   - Always include a `Version` column that records the **minimum** RAC protocol version where the field is available.
   - `RPC`: request/response method IDs and payload envelope notes.
   - `Record Layout`: one or more variants; offsets relative to record start; table `Offset | Size | Field | Type | Notes`.
   - Always include unknown/gap regions with sizes and offsets.
   - `Hypotheses` and `Open Questions` blocks.
   - Add a `Gap Analysis` block with candidate field interpretations and capture changes needed to confirm them.
7. For schema updates (`schemas/rac/*.toml`):
   - Add `version = "<min_version>"` to every field.
   - Version is **minimum supported** RAC protocol version for the field.
   - If the field already has a higher version in the file but you confirm it exists earlier, **lower** it to the correct minimum.
   - Requests can also be described in schema via `[request.*]` sections. The code generator now emits request encoders into the same `*_generated.rs` as record decoders (no separate `--requests-out`).
   - Use `literal = [..]` in request fields to encode fixed padding bytes.
8. When analyzing gaps, infer likely field placements from type sizes and nearby value patterns, and request capture changes if needed to validate (e.g., toggling RAC fields or forcing non-zero values).
9. Reference known data types and sizes (strings, datetime, boolean, UUID, u32/u64) when proposing hypotheses.
10. After successful analysis, update `docs/rac/modes/rac_modes_registry.md` with the response method mapping.

## Log Hygiene

- Keep only artifacts needed for documentation. After extracting required bytes into `./artifacts/rac/` and updating docs, delete or prune extra `./logs/` sessions.
