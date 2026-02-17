**Findings Addressed (Fixed)**
1. Non‑sequential parsing and offset usage in `apps/rac_protocol/src/rac_wire/frame.rs` and call sites `apps/rac_protocol/tests/rac_wire_decode.rs`, `apps/rac_protocol/src/commands/limit.rs`.  
   - `parse_frames` now parses strictly sequentially from the start using `RecordCursor`, no `start_offset` and no per‑frame offsets.
2. Heuristic/semantic scanning via substring/`windows` and payload scans in `apps/rac_protocol/src/client/debug.rs`, `apps/rac_protocol/src/bin/rac_decode.rs`, and `apps/rac_protocol/src/rac_wire/frame.rs`.  
   - Removed scanning helpers and SWP init detection; added deterministic service‑notice detection by fixed header prefix in `apps/rac_protocol/src/client/mod.rs` to preserve runtime behavior without heuristics.
3. Live test regression introduced by removing service‑name scanning (missing RPC header).  
   - Resolved by deterministic header check in `apps/rac_protocol/src/client/mod.rs`.

**Behavior Notes / Residual Risk**
- `rac_decode` no longer auto‑detects SWP init packets or performs heuristic string/UUID extraction. This is intentional for compliance, but it reduces convenience when decoding raw streams. If you need that functionality, we should re‑introduce it via a strict, spec‑based parser rather than scanning.

**Compliance Re‑check**
- No `scan_*` or heuristic scans remain in parsing paths.  
- Parsing is strictly sequential, no offset/seek/skip.  
- `RecordCursor` is used for frame parsing.  
- Tests remain deterministic and contain no conditional branches.

**Tests**
- `cargo test -p rac_protocol`

If you want me to re‑introduce SWP init decoding via a strict parser, say which exact framing rules to follow.