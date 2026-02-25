# RAC v11 Capture Tasks (Progress)

Фаза 1: read-only команды (без изменений на сервере).

Статусы:
- [ ] В очереди
- [~] В работе
- [x] Готово

## Read-only: базовые list/info

- [x] agent admin list
- [x] cluster list
- [x] cluster info
- [x] cluster admin list
- [x] connection list
- [x] lock list (без фильтра)
- [x] server list
- [x] server info
- [x] process list
- [x] process list --licenses
- [x] process info
- [x] process info --licenses
- [x] rule list
- [ ] rule info
- [ ] rule apply --partial
- [x] counter list
- [x] counter info
- [x] counter values
- [x] counter accumulated-values
- [x] limit list
- [x] limit info
- [x] service list
- [x] profile list

## Read-only: для выборочных захватов

- [x] session list (нужен для lock list --session)
- [x] infobase summary list (нужен для lock list --infobase)

## Read-only: фильтры (если есть IDs)

- [ ] lock list --session <id> (в session list пусто)
- [x] lock list --connection <id>
- [x] lock list --infobase <id>

## Документация

- [~] Обновить `docs/rac/messages/rac_message_formats_*.md` по новым захватам
- [x] Обновить `docs/rac/documentation/rac_cli_method_map.generated.md` при новых RPC id
- [x] Обновить `docs/rac/modes/rac_modes_registry.md` при изменениях
