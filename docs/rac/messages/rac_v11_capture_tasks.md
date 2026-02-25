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
- [ ] server list
- [ ] server info
- [ ] process list
- [ ] process list --licenses
- [ ] process info
- [ ] process info --licenses
- [ ] rule list
- [ ] rule info
- [ ] rule apply --partial
- [ ] counter list
- [ ] counter info
- [ ] counter values
- [ ] counter accumulated-values
- [ ] limit list
- [ ] limit info
- [ ] service list
- [ ] profile list

## Read-only: для выборочных захватов

- [ ] session list (нужен для lock list --session)
- [ ] infobase summary list (нужен для lock list --infobase)

## Read-only: фильтры (если есть IDs)

- [ ] lock list --session <id>
- [ ] lock list --connection <id>
- [ ] lock list --infobase <id>

## Документация

- [ ] Обновить `docs/rac/messages/rac_message_formats_*.md` по новым захватам
- [ ] Обновить `docs/rac/documentation/rac_cli_method_map.generated.md` при новых RPC id
- [ ] Обновить `docs/rac/modes/rac_modes_registry.md` при изменениях
