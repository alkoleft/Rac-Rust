Сделал. Реализовал `limit list` с последовательным декодированием, добавил структуры и тест, обновил вывод в `rac_lite`.

**Что изменено**
1. Добавил `LimitRecord`, последовательный декодер `parse_limit_list_body`, обновил `LimitListResp` и `limit_list` для использования нового декодера.  
`apps/rac_protocol/src/commands/mod.rs`
2. Добавил вывод списка лимитов в консоль (`rac_lite`).  
`apps/rac_protocol/src/bin/rac_lite/console_output.rs`  
`apps/rac_protocol/src/bin/rac_lite.rs`
3. Добавил тест на golden capture без `scan_*` и без `if`.  
`apps/rac_protocol/src/commands/mod.rs`

**Тесты**
- `cargo test -p rac_protocol parse_limit_list_from_golden_capture`

Если нужно, могу также добавить маппинг `action` в человекочитаемые значения.