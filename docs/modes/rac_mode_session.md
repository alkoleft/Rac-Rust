# RAC Mode: Session

## Purpose

Inspect and terminate sessions.

## CLI Surface

Primary entrypoint: `rac session <command>`.

Key commands:
- `list [--infobase=<uuid>]`
- `info --session=<uuid> [--licenses]`
- `terminate --session=<uuid>`
- `interrupt-current-server-call --session=<uuid>`

## Protocol Notes

Message format notes: `docs/messages/rac_message_formats_session.md`.

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: present
- `captures`: pending
- `rpc_mapping`: partial
