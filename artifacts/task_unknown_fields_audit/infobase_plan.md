# Infobase mapping plan (unknown/tail fields)

## Current state
- `schemas/rac/infobase.toml` contains many `unknown_*` fields and `tail[28]`.
- `docs/rac/messages/rac_message_formats_infobase.md` lists expected RAC output fields but does not map them to wire offsets.
- `artifacts/rac/v16/v16_20260226_053425_infobase_info_rac.out` is empty (permission error), so we lack actual output values for mapping.

## Why mapping is blocked
- Unknown string fields (5) could correspond to:
  - `external-session-manager-connection-string`
  - `security-profile-name`
  - `safe-mode-security-profile-name`
  - plus possibly other string fields from v11/v16
- `tail[28]` contains 7 x u32, but without labeled output, we cannot map to:
  - `security-level`, `license-distribution`, `scheduled-jobs-deny`, `sessions-deny`
  - `denied-from`, `denied-to` (likely u64) vs. u32 tail layout

## Required captures
1. Successful `rac infobase info` output with sufficient privileges.
   - Ensure the command returns full field set (not permission error).
   - Save:
     - `logs/session_*/client_to_server.stream.bin`
     - `logs/session_*/server_to_client.stream.bin`
     - payload hex in `artifacts/rac/<label>.hex`
     - CLI output in `artifacts/rac/<label>_rac.out`
2. Contrast capture with non-default values for key fields:
   - `security-level` non-zero
   - `scheduled-jobs-deny on`
   - `license-distribution deny`
   - `sessions-deny on` (if available)
   - `denied-from` / `denied-to` set to non-default
   - `external-session-manager-connection-string` non-empty
   - `external-session-manager-required yes`
   - `security-profile-name` and `safe-mode-security-profile-name` non-empty

## Expected outcome
- Map `unknown_*` fields to RAC output names (or mark reserved/constant fields).
- Expand `tail[28]` into explicit typed fields (or identify which belong elsewhere).
- Update `schemas/rac/infobase.toml` and add deterministic tests.

## Next actions once captures are available
- Decode record offsets and align with RAC output values.
- Replace `unknown_*`/`tail` with named fields and add asserts on sample captures.
