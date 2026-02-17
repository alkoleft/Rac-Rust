# RAC Help Methods Summary

Reference commands derived from `rac help <mode>` output. For each mode below, the path shows the CLI invocation, followed by the most important commands, required parameters, and a short description.

## Common parameters

- `--cluster=<uuid>`: identifies the target server cluster (required for most modes).
- `--cluster-user=<name>` / `--cluster-pwd=<pwd>`: credentials of the cluster administrator.
- `--host=<host>` / `<host>:<port>`: target administration server (default `localhost:1545`).

## Cluster (`rac cluster <command>`)

- `admin list`: list cluster administrators; requires `--cluster`.
- `admin register`: add a cluster administrator (`--name`, `--pwd`, `--descr`, `--auth=pwd[,os]`, `--agent-user`, `--agent-pwd`).
- `admin remove --name=<name>`: remove administrator.
- `info --cluster=<uuid>`: single-cluster metadata.
- `list`: all cluster records.
- `insert`: register new cluster (must supply `--host`, `--port`, `--name`, memory/security/load balancing options, `--agent-user`, `--agent-pwd`).
- `update`: change cluster parameters (same options as `insert` plus `--cluster`).
- `remove --cluster=<uuid>`: delete cluster.

## Manager (`rac manager <command>`)

- Requires `--cluster` + credentials.
- `info --manager=<uuid>`: get manager details.
- `list`: enumerate managers.

## Server (`rac server <command>`)

- Requires `--cluster`, credentials.
- `info --server=<uuid>`: working server details.
- `list`: list working servers.
- `insert`: register server (`--agent-host`, `--agent-port`, `--port-range`, `--name`, `--using`, `--infobases-limit`, memory/connection limits, SPN, restart schedule, `--add-prohibiting-assignment-rule`).
- `update`: change parameters (same set of limits, range, SPN, restart schedule).
- `remove --server=<uuid>`: delete server.

## Process (`rac process <command>`)

- `info --process=<uuid> [--licenses]`: working process info.
- `list [--server=<uuid>] [--licenses]`: list processes.
- `turn-off --process=<uuid>`: disable process.

## Connection (`rac connection <command>`)

- `info --connection=<uuid>`: connection metadata.
- `list [--process=<uuid>] [--infobase=<uuid>] [--infobase-user=<name>] [--infobase-pwd=<pwd>]`: enumerate connections.
- `disconnect --process=<uuid> --connection=<uuid>`: drop connection (infobase credentials optional).

## Session (`rac session <command>`)

- `info --session=<uuid> [--licenses]`: session details and license info.
- `list [--infobase=<uuid>] [--licenses]`: active sessions.
- `terminate --session=<uuid> [--error-message=<string>]`: force-stop.
- `interrupt-current-server-call --session=<uuid> [--error-message=<string>]`.

## Lock (`rac lock list`)

- `list [--infobase=<uuid>] [--connection=<uuid>] [--session=<uuid>]`: current session locks.

## Rule (`rac rule <command>`)

- Commands: `apply [--full|--partial]`, `info --server=<uuid> --rule=<uuid>`, `list --server=<uuid>`, `insert/update/remove` with parameters `--server`, `--rule`, `--position`, `--object-type`, `--infobase-name`, `--rule-type=auto|always|never`, `--application-ext`, `--priority`.

## Profile (`rac profile <command>`)

- `list`: list security profiles.
- `update --name=<name> [--descr|--config|--priv|--full-privileged-mode|--privileged-mode-roles|--crypto|--right-extension|--right-extension-definition-roles|--all-modules-extension|--modules-available-for-extension|--modules-not-available-for-extension]`.
- `remove --name=<name>`: delete profile.
- `acl` subgroup handles `directory`, `com`, `addin` commands with their own update/remove options.

## Counter (`rac counter <command>`)

- `list`: counters inventory.
- `info --counter=<name>`: counter data.
- `update`: create/update counter (`--name`, `--collection-time`, `--group`, `--filter-type`, `--filter`, toggles like `--duration`, `--cpu-time`, `--memory`, `--read`, `--write`, `--duration-dbms`, `--dbms-bytes`, `--service`, `--call`, `--number-of-active-sessions`, `--number-of-sessions`, `--descr`).
- `values --counter=<name> [--object=<filter-spec>]`: readings.
- `clear --counter=<name> [--object=<filter-spec>]`, `accumulated-values --counter=<name> [--object=<filter-spec>]`, `remove --name=<name>`.

## Limit (`rac limit <command>`)

- `list`: enumerates resource limits.
- `info --limit=<name>`: limit settings.
- `update --name=<name> --action=<none|set-low-priority-thread|interrupt-current-call|interrupt-session>` plus optional target counters (`--counter`), numeric thresholds (`--duration`, `--cpu-time`, `--memory`, `--read`, `--write`, `--duration-dbms`, `--dbms-bytes`, `--service`, `--call`, `--number-of-active-sessions`, `--number-of-sessions`), `--error-message`, `--descr`.
- `remove --name=<name>`.

## Service Setting (`rac service-setting <command>`)

- Requires `--cluster`, credentials, and `--server=<uuid>`.
- `info --setting=<uuid>`, `list`.
- `insert --service-name=<name> [--infobase-name=<name>] [--service-data-dir=<dir>]`.
- `update --setting=<uuid> [--service-data-dir=<dir>]`.
- `get-service-data-dirs-for-transfer [--service-name=<name>]`.
- `remove --setting=<uuid>`, `apply` to commit settings.

## Binary Data Storage (`rac binary-data-storage <command>`)

- Requires `--cluster`, credentials, `--infobase=<uuid>`, infobase admin credentials.
- `info --storage=<uuid> | --name=<name>`, `list`.
- `create-full-backup --server-path=<path>`, `create-diff-backup --server-path=<path> --full-backup-server-path=<path>`.
- `load-full-backup --server-path=<path>`, `load-diff-backup --server-path=<path> --full-backup-server-path=<path>`.
- `clear-unused-space --storage=<uuid> | --name=<name> [--by-universal-date=<date>]`.

## Agent (`rac agent <command>`)

- Requires `--agent-user`/`--agent-pwd`.
- `admin list/register/remove` same options as `cluster admin`.
- `version`: get agent version string.

## Service (`rac service list`)

- `list`: returns services available to the cluster manager.

## Notes

- `docs/documentation/rac_cli_method_map.generated.md` links each command to observed RPC method IDs.
