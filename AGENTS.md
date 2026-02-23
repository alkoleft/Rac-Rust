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

## Codegen Rules
- Prefer maximum use of code generation for protocol structures and commands.
- Keep abstractions, wrappers, and extra data structures to the minimum required for correctness.
- When possible, define RPC request fields inline under `[rpc.*]` (use `fields` and optional `derive`) instead of separate `[request.*]`.
- Responses should be described in `[response.*]`; generate response structs only when `body.struct = true`.
- Response bodies may define `field` to control the response struct field name (e.g., `admins`, `version`).

## Test Rules
- Tests must have strict, deterministic assertions.
- No `if` or conditional branches in tests (no “fitting to implementation”).
- After implementation changes, run relevant tests (at least `cargo test -p rac_protocol`) and report results.

## Review Expectations
- Review must enforce all rules above before commit.

## Git Workflow Rules
- At the start of a new task, create a new git branch.
- Do not create new branches for subtasks within the same ongoing task; continue on the existing branch.
- Use the `codex/` prefix for new branch names (example: `codex/super-task`).
- Create a commit for each stage of work.
