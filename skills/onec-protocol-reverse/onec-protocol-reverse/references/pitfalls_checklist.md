# Pitfalls Checklist

Use this before concluding protocol structure.

## Transport Pitfalls

- Do not assume TCP packet boundaries equal protocol message boundaries.
- Do not assume payload length is one byte; test varuint for all frames.
- Verify parser consumes full stream without leftover bytes.

## Semantic Pitfalls

- Distinguish handshake/service frames from business RPC frames.
- Expect context-setting RPC before target command (`cluster`, session, entity).
- Treat `01 00 00 00` as possible status/ack, not a business method.

## Validation Pitfalls

- Do not promote hypotheses to facts before live replay succeeds.
- Compare same command with at least two different UUIDs.
- Compare empty-result and non-empty-result responses.

## Reporting Pitfalls

- Always include exact capture file paths.
- Mark uncertain field decoding as `hypothesis`.
- Keep method map versioned by capture date and server build.
