# Реестр захватов v11

Формат: команда и связанные файлы/шаблоны. Все пути относительные к `artifacts/rac/v11`.

| Команда | Файлы |
| --- | --- |
| `agent admin list` | `v11_agent_admin_list_ro_*` |
| `cluster list` | `v11_cluster_list_ro_*` |
| `cluster info` | `v11_cluster_info_ro_*` |
| `cluster admin list` | `v11_cluster_admin_list_ro_*` |
| `connection list` | `v11_connection_list_ro_*` |
| `lock list` | `v11_lock_list_ro_*`, `v11_lock_list_{connection,infobase,session}_ro_*` |
| `server list` | `v11_server_list_ro_*` |
| `server info` | `v11_server_info_ro_*` |
| `process list` | `v11_process_list_ro_*` |
| `process list --licenses` | `v11_process_list_licenses_ro_*` |
| `process info` | `v11_process_info_ro_*` |
| `process info --licenses` | `v11_process_info_licenses_ro_*` |
| `rule list` | `v11_rule_list_ro_*` |
| `counter list` | `v11_counter_list_ro_*` |
| `counter info` | `v11_counter_info_ro_*` |
| `counter values` | `v11_counter_values_ro_*` |
| `counter accumulated-values` | `v11_counter_accumulated_values_ro_*` |
| `limit list` | `v11_limit_list_ro_*` |
| `limit info` | `v11_limit_info_ro_*` |
| `service list` | `v11_service_list_ro_*` |
| `profile list` | `v11_profile_list_ro_*` |
| `session list` | `v11_session_list_ro_*` |
| `infobase summary list` | `v11_infobase_summary_list_ro_*` |
| Негативные сценарии | `v11_error_*` |
| Help/CLI output | `help/*` |

## Legacy

Старые артефакты v11 со старыми именами перенесены в `legacy/`.
