# RAC Mode: Limit

## Purpose

Manage resource limits bound to counters.

## CLI Surface

Primary entrypoint: `rac limit <command>`.

Key commands:
- `list`
- `info --limit=<name>`
- `update --name=<name> --action=<...> ...`
- `remove --name=<name>`

## Protocol Notes

Message format notes: not yet captured.

Related method mapping:
- `docs/documentation/rac_method_map.md`
- `docs/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: done (list/info/update/remove)
- `captures`: list/info/update/remove
- `rpc_mapping`: partial (list/info/update/remove)
