# RAC Static Reverse Notes

Target analyzed:

- `/opt/1cv8/x86_64/8.5.1.1150/rac`
- Date analyzed: 2026-02-14

## Binary Profile

- ELF 64-bit, stripped executable.
- Size: ~796 KB.
- Build ID: `3a24c849df957b40`.
- `.gnu_debuglink` points to `rac.debug` (not found on host).

Key implication:

- Full source-level decompilation is limited without `rac.debug`.
- Still useful: dynamic symbols, resources, strings, and behavioral captures.

## Runtime Dependencies

Important linked modules:

- `core85.so`
- `nuke85.so`

Observation:

- `rac` is a CLI shell/entrypoint layer.
- Much business logic is in `core85.so`.

## Strong Findings From Static Inspection

1. Resource-based command model
   - `rac_root.res` contains keys `IDS_MODE_DESCR_*`, `IDS_CMD_DESCR_*`, `IDS_OPT_DESCR_*`.
   - This is a reliable inventory source for protocol reverse planning.

2. Command processor architecture exposed in strings
   - `AgentCommandProcessor`
   - `ClusterCommandProcessor`
   - `ManagerCommandProcessor`
   - `ServerCommandProcessor`
   - `ProcessCommandProcessor`
   - `ServiceCommandProcessor`
   - `InfoBaseCommandProcessor`
   - `ConnectionCommandProcessor`
   - `SessionCommandProcessor`
   - `LockCommandProcessor`
   - `RuleCommandProcessor`
   - `SecurityProfileCommandProcessor`
   - `CounterCommandProcessor`
   - `LimitCommandProcessor`
   - `ServiceSettingCommandProcessor`
   - `BinaryDataStorageCommandProcessor`

3. Formatter class names for result schemas
   - `ClusterInfoFormatter`
   - `ClusterManagerInfoFormatter`
   - `WorkingServerInfoFormatter`
   - `WorkingProcessInfoFormatter`
   - `InfoBaseInfoFormatter`
   - `InfoBaseConnectionInfoFormatter`
   - etc.

4. Confirmed extra modes in this build
   - `session`
   - `lock`
   - `rule`
   - `profile`
   - `counter`
   - `limit`
   - `service-setting`
   - `binary-data-storage`

5. `IDS_CMD_DESCR_*` reveals wider feature surface
   - includes profile ACL related command families and binary-data-storage operations.

## Useful Repro Commands

Inventory modes/commands/options from binary:

```bash
strings -a -n 4 /opt/1cv8/x86_64/8.5.1.1150/rac | rg '^IDS_MODE_DESCR_' | sort -u
strings -a -n 4 /opt/1cv8/x86_64/8.5.1.1150/rac | rg '^IDS_CMD_DESCR_' | sort -u
strings -a -n 4 /opt/1cv8/x86_64/8.5.1.1150/rac | rg '^IDS_OPT_DESCR_' | sort -u
```

Probe supported modes live:

```bash
/opt/1cv8/x86_64/8.5.1.1150/rac --help
/opt/1cv8/x86_64/8.5.1.1150/rac help <mode>
```

## Reverse Engineering Strategy Impact

For mass protocol reverse:

1. Use resource keys to build capture matrix first.
2. Prioritize command families by business value:
   - `cluster/server/process/connection/infobase`
   - then `session/rule/profile/counter/limit/service-setting`
3. For each command family, capture:
   - list
   - info
   - create/update/remove/operation variants
4. Feed captures into method map inference workflow (`reverse_workflow.md`).
