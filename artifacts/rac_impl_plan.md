# RAC Implementation Plan

| # | Mode | Command | Description | Message formats | Analyzed | Implemented | Notes |
|---|------|---------|-------------|-----------------|----------|-------------|-------|
| 1 | `connection` | `info` | `-` | `-` | yes | - | - |
| 2 | `lock` | `list` | ``docs/modes/rac_mode_lock.md`` | ``docs/messages/rac_message_formats_lock.md`` | yes | - | - |
| 3 | `rule` | `apply` | ``docs/modes/rac_mode_rule.md`` | ``docs/messages/rac_message_formats_rule.md`` | yes | - | - |
| 4 | `rule` | `list` | `-` | `-` | yes | - | - |
| 5 | `rule` | `info` | `-` | `-` | yes | - | - |
| 6 | `rule` | `insert` | `-` | `-` | yes | - | - |
| 7 | `rule` | `update` | `-` | `-` | yes | - | - |
| 8 | `rule` | `remove` | `-` | `-` | yes | - | - |
| 9 | `counter` | `list` | ``docs/modes/rac_mode_counter.md`` | ``docs/messages/rac_message_formats_counter.md`` | yes | - | - |
| 10 | `counter` | `info` | `-` | `-` | yes | - | - |
| 11 | `counter` | `update` | `-` | `-` | yes | - | - |
| 12 | `counter` | `values` | `-` | `-` | yes | - | - |
| 13 | `counter` | `accumulated-values` | `-` | `-` | yes | - | - |
| 14 | `counter` | `clear` | `-` | `-` | yes | - | - |
| 15 | `counter` | `remove` | `-` | `-` | yes | - | - |
| 16 | `limit` | `info` | `-` | `-` | yes | - | - |
| 17 | `limit` | `update` | `-` | `-` | yes | - | - |
| 18 | `limit` | `remove` | `-` | `-` | yes | - | - |
| 19 | `service-setting` | `list` | ``docs/modes/rac_mode_service-setting.md`` | ``docs/messages/rac_message_formats_service-setting.md`` | yes | - | - |
| 20 | `service-setting` | `info` | `-` | `-` | yes | - | - |
| 21 | `service-setting` | `insert` | `-` | `-` | yes | - | - |
| 22 | `service-setting` | `update` | `-` | `-` | yes | - | - |
| 23 | `service-setting` | `get-service-data-dirs-for-transfer` | `-` | `-` | yes | - | - |
| 24 | `service-setting` | `remove` | `-` | `-` | yes | - | - |
| 25 | `service-setting` | `apply` | `-` | `-` | yes | - | - |
| 26 | `agent` | `admin list` | ``docs/modes/rac_mode_agent.md`` | `-` | yes | - | - |
