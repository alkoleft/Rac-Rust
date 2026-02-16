# RAC Method Map (Current)

Observed business mapping from live captures:

| Request method | Response method | Observed command |
|---|---|---|
| `0x0b` | `0x0c` | `cluster list` |
| `0x0d` | `0x0e` | `cluster info --cluster <id>` |
| `0x09` | `01 00 00 00` | cluster context set before many cluster-scoped calls |
| `0x12` | `0x13` | `manager list --cluster <id>` |
| `0x14` | `0x15` | `manager info --cluster <id> --manager <id>` |
| `0x16` | `0x17` | `server list --cluster <id>` |
| `0x18` | `0x19` | `server info --cluster <id> --server <id>` |
| `0x1d` | `0x1e` | `process list --cluster <id>` |
| `0x1f` | `0x20` | `process info --cluster <id> --process <id>` |
| `0x2a` | `0x2b` | `infobase summary list --cluster <id>` |
| `0x32` | `0x33` | `connection list --cluster <id>` |
| `0x41` | `0x42` | `session list --cluster <id>` |
| `0x48` | `0x49` | `lock list --cluster <id>` |
| `0x59` | `0x5a` | `profile list --cluster <id>` |
| `0x76` | `0x77` | `counter list --cluster <id>` |
| `0x7c` | `0x7d` | `limit list --cluster <id>` |
| `0x87` | `0x88` | `agent version` |
| `0x08` | `01 00 00 00` | `agent auth` (username/password), ack only |
| `0x00` | `0x01` | `agent admin list` (hypothesis) |

## Update Rule

When adding new entries:

1. Link request and response from the same capture session.
2. Include command string used for capture.
3. Mark uncertain mapping as `hypothesis`.

## Notes

- `connection list --cluster <id>` confirmed by capture `logs/session_1771281887_2714678_127_0_0_1_59850` (2026-02-17).
- `rule list --cluster <id>` failed in tested build with:
  - `Error parsing option: server`
  - capture showed only context method `0x09` before failure.
- `agent admin list --agent-user=admin --agent-pwd=pass` capture:
  - `logs/session_1771283676_2737297_127_0_0_1_59274` (2026-02-16)
