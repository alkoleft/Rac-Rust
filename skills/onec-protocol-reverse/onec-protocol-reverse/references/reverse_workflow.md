# Reverse Workflow For 1C Protocols

## 0. Pre-Flight Checks

1. Ensure capture pipeline is actually in path:
   - Proxy listens on expected port.
   - `LISTEN_ADDR` is free.
   - `./logs/` and `./artifacts/` are writable.
2. Verify core tools:
   - `cargo`, `jq`, `openssl`, `tcpdump`/`tshark` or `mitmproxy`.
3. Run a tiny control capture:
   - One known command with known output.
   - Confirm streams are produced and `rac_decode` parses them.

## 1. Capture Plan

1. Start with read-only commands.
2. Capture at least:
   - one list command with non-empty result
   - one info command with explicit UUID
   - one command that returns empty list/result
3. Keep a command-to-session table immediately.

## 2. Framing Inference

1. Identify transport init marker (if any) before framed packets.
2. Test frame format candidates against full stream:
   - `opcode:u8 + len:u8 + payload`
   - `opcode:u8 + len:varuint + payload`
3. Accept framing only if parser consumes stream without drift.

## 3. Envelope Inference

1. Detect stable payload prefixes.
2. Extract request/response method IDs.
3. Detect context setters (cluster/session/entity selection).
4. Separate:
   - transport handshake frames
   - service negotiation frames
   - business RPC frames

## 4. Field Inference

1. Locate length-prefixed UTF-8 strings.
2. Locate UUID-like 16-byte sequences.
3. Compare same method with different entities to isolate ID fields.
4. Compare empty vs non-empty response to isolate collection envelope.

## 5. Validation Loop

1. Implement minimal client call for one method.
2. Execute against real server.
3. Compare output with official utility.
4. Promote only validated findings to method map.

## 6. Evidence Journal (Required)

Track work as facts vs hypotheses. Use this lightweight format:

```
FACT: <verified observation> (source: <file/offset>)
HYPOTHESIS: <theory> (confidence: High|Medium|Low)
QUESTION: <open unknown>
EXPERIMENT: <planned capture/compare>
RESULT: <experiment outcome> (source: <file/offset>)
```

Hypothesis status:
- Confirmed: multiple facts, no contradictions
- Uncertain: some support, some gaps
- Refuted: clear contradicting facts

## 7. Reporting Template

Use this structure:

1. Confirmed transport format.
2. Confirmed envelope format.
3. Method map (`req -> resp`, purpose, required parameters).
4. Known entity layouts (UUID/string/numeric fields).
5. Open questions and next captures required.

## 8. Troubleshooting Quick Checks

- Empty captures: verify proxy is in the path and `LISTEN_ADDR` is correct.
- Decode drift: framing hypothesis wrong; re-check payload lengths vs stream size.
- Non-deterministic fields: capture same command twice and diff to isolate timestamps/counters.
