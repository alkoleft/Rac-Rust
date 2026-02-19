**Summary**
Implemented `cluster admin register` end-to-end with request encoding, response handling, and CLI wiring. Added request/response structs and console output for ack status. Updated RAC protocol serialization and constants, plus a golden encode test using the capture. Files touched include `apps/rac_protocol/src/commands/cluster.rs`, `apps/rac_protocol/src/client/protocol.rs`, `apps/rac_protocol/src/rac_wire/consts.rs`, `apps/rac_protocol/src/bin/rac_lite.rs`, and `apps/rac_protocol/src/bin/rac_lite/console_output.rs`.

**Tests**
1. `cargo test -p rac_protocol` (started, blocked on `tests/live_infobase.rs` waiting for live env; terminated)
2. `cargo test -p rac_protocol -- --skip live_infobase` (pass, includes live_server and wire decode tests)

**Notes**
- Added CLI: `rac_lite cluster admin register --cluster <uuid> --cluster-user <name> --cluster-pwd <pwd> --name <name> --pwd <pwd> --descr <descr> --auth <pwd|os|pwd,os>`.
- Console output prints `cluster-admin-register: ok` when ack received.