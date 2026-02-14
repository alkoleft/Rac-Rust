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
   - Use `scripts/capture_rac_command.sh` in this skill for RAC commands.
   - Or run equivalent proxy + client workflow for a different protocol.
3. Decode streams:
   - `cargo run --bin rac_decode -- <session>/client_to_server.stream.bin`
   - `cargo run --bin rac_decode -- <session>/server_to_client.stream.bin`
4. Update method/framing notes using templates in `references/`.
5. Validate hypotheses by implementing minimal live client calls (`rac_lite` style).

## Workflow

1. Read `references/reverse_workflow.md` at the start of each new protocol task.
2. If protocol is RAC or RAC-like, read `references/rac_baseline.md`.
3. If mapping methods, read `references/rac_method_map.md`.
4. Before final conclusions, read `references/pitfalls_checklist.md`.
5. If task is static reverse/decomp-style analysis of `rac`, read `references/rac_static_reverse.md`.
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

- `scripts/capture_rac_command.sh`
  - Deterministic single-command capture via local proxy.
- `scripts/method_map_from_sessions.sh`
  - Batch extract method IDs from session directories using `rac_decode`.
- `scripts/map_rac_commands_to_methods.sh`
  - Run a capture matrix and generate `docs/rac_cli_method_map.generated.md`.
- `scripts/dump_rac_static_info.sh`
  - Dump static metadata from `rac` binary (ELF profile, debuglink, IDS keys, supported modes).
- `docs/rac_help_methods.md`
  - Quick reference for `rac help <mode>` commands, parameters, and descriptions.

Run scripts from repository root unless user asks otherwise.

## Output Requirements

When reporting protocol analysis:

1. Present confirmed facts first (bytes, offsets, method IDs, frame format).
2. Mark hypotheses explicitly.
3. Include concrete file references to captures.
4. Record new mappings in `references/` for reuse.
5. If building replacement behavior, show one verified live command/output.
