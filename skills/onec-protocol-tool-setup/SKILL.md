---
name: onec-protocol-tool-setup
description: Set up and verify tooling for capturing and decoding 1C RAC/admin protocol traffic (proxy/capture, decode, and dependencies). Use when tooling is missing, capture pipeline fails, logs are empty, decode drift occurs, or the environment prerequisites are unknown.
---

# Onec Protocol Tool Setup

## Overview

Verify that the capture pipeline and decode tools are installed, reachable, and producing usable streams before deeper protocol work.

## Workflow

1. Read `references/tooling_checklist.md` and run the pre-flight checks.
2. Confirm ports/paths:
   - `LISTEN_ADDR` is free.
   - `./logs/` and `./artifacts/rac/` are writable.
3. Run a control capture:
   - One known command with expected output.
   - Confirm streams exist and `rac_decode` parses them.
4. If any step fails, use the troubleshooting section in the checklist and report the exact failing command/output.

## Output Requirements

When reporting setup results:

1. List tools verified and missing.
2. Provide the exact command(s) used for the control capture.
3. State whether `rac_decode` parsed both directions.
4. If blocked, list the smallest next action needed to proceed.
