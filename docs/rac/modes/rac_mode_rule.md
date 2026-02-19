# RAC Mode: Rule

## Purpose

Manage assignment rules for working servers.

## CLI Surface

Primary entrypoint: `rac rule <command>`.

Key commands:
- `apply [--full|--partial]`
- `list --server=<uuid>`
- `info --server=<uuid> --rule=<uuid>`
- `insert/update/remove`

## Protocol Notes

Message format notes: `docs/rac/messages/rac_message_formats_rule.md`.

Related method mapping:
- `docs/rac/documentation/rac_method_map.md`
- `docs/rac/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: done
- `captures`: done
- `rpc_mapping`: done
