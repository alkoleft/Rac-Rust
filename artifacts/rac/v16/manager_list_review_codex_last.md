Summary
- Moved all command implementations into per‑mode modules and re‑exported them from `apps/rac_protocol/src/commands/mod.rs`.
- Replaced semantic scans and offset-based parsing with strict sequential `RecordCursor` reads across commands and wire parsing.
- Updated counter list handling to return string names and adjusted CLI output accordingly.
- Rewrote `apps/rac_protocol/tests/rac_wire_decode.rs` to use deterministic artifacts and sequential parsing, removing conditional skips.

Key changes
- New modules: `apps/rac_protocol/src/commands/agent.rs`, `apps/rac_protocol/src/commands/server.rs`, `apps/rac_protocol/src/commands/process.rs`, `apps/rac_protocol/src/commands/connection.rs`, `apps/rac_protocol/src/commands/lock.rs`, `apps/rac_protocol/src/commands/profile.rs`, `apps/rac_protocol/src/commands/counter.rs`.
- Updated parsers: `apps/rac_protocol/src/commands/cluster.rs`, `apps/rac_protocol/src/commands/infobase.rs`, `apps/rac_protocol/src/commands/session.rs`.
- RecordCursor-only parsing and removal of scan/offset helpers: `apps/rac_protocol/src/codec.rs`, `apps/rac_protocol/src/rac_wire/mod.rs`, `apps/rac_protocol/src/rac_wire/frame.rs`.
- CLI display update: `apps/rac_protocol/src/bin/rac_lite.rs`, `apps/rac_protocol/src/bin/rac_lite/console_output.rs`.
- Deterministic tests: `apps/rac_protocol/tests/rac_wire_decode.rs`.

Behavioral notes
- `CounterListResp.counters` is now `Vec<String>` instead of UUIDs.
- Lock list now returns object UUIDs (parsed sequentially) instead of scan-based UUID aggregation.

Tests
- `cargo test -p rac_protocol`

Result: all tests passed.  
Warning: `decode_varuint` is now unused in `apps/rac_protocol/src/rac_wire/frame.rs`.

Re-check (manual)
- No `scan_*` helpers remain in `apps/rac_protocol/src`.
- No `seek`/`skip` usage in `RecordCursor`.
- No offset-based parsing in command modules; all record parsing uses `RecordCursor`.

If you want, I can either remove the unused `decode_varuint` function or add a small unit test that exercises it to clear the warning.

Possible next steps
1. Remove or repurpose the unused `decode_varuint` in `apps/rac_protocol/src/rac_wire/frame.rs`.
2. Add artifact-backed tests for the new `server`, `process`, `connection`, and `lock` parsers.