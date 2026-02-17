**Summary**
1. Implemented sequential `manager list`/`manager info` parsing with `RecordCursor`, added `ManagerRecord` structures, and moved manager commands into `apps/rac_protocol/src/commands/manager.rs`.
2. Added deterministic tests for manager list/info using captured hex artifacts.
3. Updated `rac_lite` console output to display manager records and adjusted CLI output paths.

**Tests**
1. `cargo test -p rac_protocol`

**Files Changed**
1. `apps/rac_protocol/src/commands/manager.rs`
2. `apps/rac_protocol/src/commands/mod.rs`
3. `apps/rac_protocol/src/codec.rs`
4. `apps/rac_protocol/src/bin/rac_lite/console_output.rs`
5. `apps/rac_protocol/src/bin/rac_lite.rs`

If you want the manager `using` field mapped to more labels beyond `main`, say which values you expect and I’ll add them.