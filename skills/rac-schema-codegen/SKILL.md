---
name: rac-schema-codegen
description: Update RAC protocol schema TOML under `schemas/rac/*.toml`, map completed RAC commands from `docs/rac/messages` and `artifacts/rac/v16/format_parse_tasklist.md`, and regenerate codegen via `scripts/rac/rac_codegen.py` or `scripts/rac/rac_codegen_all.py`. Use when adding or correcting RPC/request/response definitions, record layouts, or method IDs; this repo uses inline request fields under `[rpc.*]` (no separate request block unless reuse is required).
---

# RAC Schema & Codegen

## Overview
Define or update RAC protocol schemas and regenerate the corresponding Rust codegen with the repository tooling.

## Workflow
1. Identify completed commands from `artifacts/rac/v16/format_parse_tasklist.md` and the matching format docs in `docs/rac/messages/`.
2. Update the relevant `schemas/rac/*.toml` with sequential record layouts and inline RPC/request fields.
3. Regenerate code:
   - Single schema: `python3 scripts/rac/rac_codegen.py schemas/rac/<name>.toml --out apps/rac_protocol/src/commands/<name>_generated.rs`
   - All schemas: `python3 scripts/rac/rac_codegen_all.py`
4. If implementation behavior changes, run `cargo test -p rac_protocol` (expect missing fixture errors if artifacts are absent).

## Schema Rules
- Prefer inline request fields under `[rpc.*]` using `fields = [...]` when possible; only define `[request.*]` for reuse across multiple RPCs.
- Responses belong in `[response.*]`; generate a response struct only when `body.struct = true`.
- Use `field` on response bodies to control list/record field names.
- Keep schemas codegen-friendly: avoid `super::` references and minimize manual types.
- Parsing must be strictly sequential and based on `RecordCursor`; do not rely on scan helpers.

## References
- Read `references/schema-format.md` for the canonical TOML layout and examples (including the inline RPC/request nuance).
