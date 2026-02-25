# Message Format Template (RAC)

Use this template for message format documentation in `docs/rac/messages/`.

---

# <TITLE> (Observed)

Source capture:
- `logs/<session>/server_to_client.stream.bin`

Payload example:
- `artifacts/rac/<label>.hex`

RAC output reference:
- `artifacts/rac/<label>_rac.out`

## RPC

Available from: `<min version>` |
Request method: `0x??` (`<command> ...`)
Response method: `0x??`

Payload structure (method body):
- offset `0x00`: `<field>`
- offset `0x..`: `<field>`

## Поля ответа (из `rac`)

Observed field names in `rac <mode> <command>` output, with capture mapping status.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `<field>` | `<type>` | yes/no/hypothesis | `<order>` | `<min version>` |

## Поля запроса (из `rac`)

Observed request parameters for `rac <mode> <command>`.

| Field | Type | Found In Capture | Order In Capture | Version |
|---|---|---|---|---|
| `<field>` | `<type>` | yes/no/hypothesis | `<order>` | `<min version>` |

## Record Layout (Observed)

Offsets are relative to the start of a record.

| Offset | Size | Field | Type | Notes |
|---|---|---|---|---|
| `0x00` | `16` | `<field>` | UUID | |
| `0x10` | `1` | `<field_len>` | u8 | |
| `0x11` | `<field_len>` | `<field>` | string | UTF-8 |
| `0x11 + <field_len>` | `4` | `<gap>` | gap | unknown, keep bytes |

Notes:
- Gaps: list all unrecognized areas with offsets and sizes.
- For each gap, add candidate field interpretations based on size/type patterns.

## Hypotheses

- `<hypothesis>`

## Open Questions

- `<question>`

## Gap Analysis (Required)

For each gap/unknown field:
- Size and offset.
- Candidate field types (based on known type sizes).
- What capture changes could confirm it (e.g., toggle a RAC field).
