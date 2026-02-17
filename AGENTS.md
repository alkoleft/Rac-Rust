# Project Rules (Codex Agents)

These rules apply to all Codex-driven implementation and review work in this repository.

## Parsing Rules
- Parsing must be strictly sequential (no `offset` variables, no `seek/skip` on cursors).
- Use `RecordCursor` for reading/decoding records.
- Semantic scanning is запрещен (no `scan_*` helpers like `scan_len_prefixed_strings`, `scan_uuid_bytes`, etc).

## Module Structure Rules
- Each mode’s commands must be implemented in its own module:
  - `apps/rac_protocol/src/commands/<mode>.rs`
- Do not add new command implementations directly into `apps/rac_protocol/src/commands/mod.rs`.

## Test Rules
- Tests must have strict, deterministic assertions.
- No `if` or conditional branches in tests (no “fitting to implementation”).
- After implementation changes, run relevant tests (at least `cargo test -p rac_protocol`) and report results.

## Review Expectations
- Review must enforce all rules above before commit.
