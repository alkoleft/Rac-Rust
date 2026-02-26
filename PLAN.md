# RAC Message Formats + CLI/Library (Read/Info Scope)

## Summary
Build a definitive Markdown spec of RAC message formats for read/info commands and implement a Rust library + CLI that encodes/decodes those messages. The spec will include transport framing, RPC envelope, per-command request/response IDs, parameter formats, and chain sequences (init → service negotiation → context → RPC → close). The library will provide typed codecs; the CLI will expose read/info calls.

## Scope
- Commands: **read/info only** (list/info/summary/connection/session/lock/profile/counter/limit/agent version).
- Output: **Markdown spec** at `docs/rac/messages/rac_message_formats.md`.
- “Chain” definition: **transport + RPC sequence** (init + negotiation + context + RPC + close).
- Detail level: **max semantics** (name fields whenever defensible, mark uncertain fields as hypotheses).

## Deliverables
1. `docs/rac/messages/rac_message_formats.md` (spec)
2. `src/rac_wire/` library module with codecs
3. CLI tool (`apps/rac_cli/src/bin/rac_lite.rs` extended) using the library
4. Tests against existing captures

---

## Implementation Plan

### 1) Spec Extraction Pipeline (non-code)
**Inputs**
- Captures in `logs/session_*`
- Method map in `docs/rac/documentation/rac_cli_method_map.generated.md`
- Existing notes in `docs/rac/documentation/rac_protocol_notes.md`, `skills/onec-protocol-reverse/references/rac_method_map.md`

**Steps**
1. Enumerate read/info commands from `docs/rac/documentation/rac_cli_method_map.generated.md`.
2. For each command, locate its capture sessions and pull:
   - c2s/s2c frames from `client_to_server.stream.bin` and `server_to_client.stream.bin`
   - `events.log` for sequencing
3. For each response payload:
   - Identify stable header bytes
   - Extract UUIDs (16 bytes)
   - Extract length-prefixed strings
   - Extract numeric fields (guess u32/u64/float where possible)
4. Compare multiple instances of same command (if available) to isolate parameter fields.
5. Record:
   - Request method_id and parameters (name + format)
   - Response method_id and field list (name + format)
   - Any secondary frames (e.g., extra version frame for process info)

**Output**
- `docs/rac/messages/rac_message_formats.md` with one section per command:
  - `Command`, `Request method_id`, `Response method_id`
  - `Chain` (init → negotiation → context → RPC → close)
  - `Parameters` (field, type, format notes)
  - `Response fields` (field, type, format notes)
  - `Evidence` (capture session IDs)

### 2) Library Design (`src/rac_wire/`)
**Modules**
- `frame.rs`: transport framing (opcode + varuint length + payload)
- `rpc.rs`: `01 00 00 01 <method_id>` envelope, context call `0x09`
- `types.rs`: `Uuid16`, `Str8`, `VarUInt`, `Bool`, `U32`, `U64`, `F64?`
- `parse.rs`: decode helpers from bytes with offset
- `format.rs`: encode helpers

**Interfaces**
- `decode_frame(bytes) -> Frame`
- `encode_frame(opcode, payload)`
- `encode_rpc(method_id, payload)`
- `decode_rpc(payload) -> {method_id, body}`
- Field decode helpers with `Result<(value, new_offset)>`

### 3) CLI (`rac_lite`)
**Goals**
- Implement read/info commands only
- Use library for encoding/decoding
- Provide output parity with `rac` for known fields

**Commands to implement**
- `cluster list/info`
- `manager list/info`
- `server list/info`
- `process list/info`
- `infobase summary list/info`, `infobase info`
- `connection list/info`
- `session list/info`
- `lock list`
- `profile list`
- `counter list`
- `limit list`
- `agent version`

### 4) Tests
- Golden decode tests: use existing `logs/session_*` binary files
- Tests that:
  - Frame parsing consumes full stream
  - Method IDs match `docs/rac/documentation/rac_cli_method_map.generated.md`
  - Known strings/UUIDs extracted correctly

---

## Public API/Interface Changes
- New library module `src/rac_wire/*`
- Extended `rac_lite` CLI with read/info commands

---

## Acceptance Criteria
- `docs/rac/messages/rac_message_formats.md` lists every read/info command with:
  - request/response method IDs
  - parameter formats
  - response field formats
  - chain sequence
- `rac_lite` can query at least:
  - cluster list/info
  - manager list/info
  - server list/info
  - process list/info
  - infobase summary list/info
  - connection list/info
  - session list/info
- Tests pass and prove decode stability against captured sessions

---

## Assumptions / Defaults
- Use existing capture artifacts in `logs/`.
- Treat unknown field semantics as `hypothesis` labels in spec.
- Favor stable field typing (UUID/string/int) over speculative semantics.

---

If you want the spec to include non-read commands or JSON output in addition to Markdown, we’ll expand scope after this phase.
