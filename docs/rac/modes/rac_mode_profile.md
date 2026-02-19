# RAC Mode: Profile

## Purpose

Manage security profiles and ACLs.

## CLI Surface

Primary entrypoint: `rac profile <command>`.

Key commands:
- `list`
- `update --name=<name> ...`
- `remove --name=<name>`
- `acl directory/com/addin ...`

## Protocol Notes

Message format notes: not yet captured.

Related method mapping:
- `docs/rac/documentation/rac_method_map.md`
- `docs/rac/documentation/rac_cli_method_map.generated.md`

## Status

- `description`: done
- `message_formats`: missing
- `captures`: pending
- `rpc_mapping`: partial
