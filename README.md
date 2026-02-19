# v8_protocols

Эксперимент по реверс‑инжинирингу протоколов 1С с помощью AI.  
Цель: собирать трафик, раскладывать его на обмены, строить карты методов и постепенно заменять официальный `rac` на собственные утилиты.

## Что внутри

- `apps/v8_proxy` — TCP‑прокси для перехвата и логирования сессий.
- `apps/rac_protocol` — инструменты для RAC: декодер фрейминга и минимальный клиент.
- `apps/rac_rest` — REST‑шлюз к RAC (read‑only).
- `docs/` — заметки по протоколу и карта методов.

## Быстрый старт

Сборка:

```bash
cargo build --release -p v8_proxy
cargo build --release -p rac_protocol
cargo build --release -p rac_rest
```

Запуск прокси:

```bash
cargo run --release -p v8_proxy -- \
  --listen 127.0.0.1:15410 \
  --target 127.0.0.1:1541 \
  --log-dir ./logs \
  --try-inflate=true
```

Декодирование потока RAC:

```bash
cargo run -p rac_protocol --bin rac_decode -- logs/<session>/client_to_server.stream.bin
cargo run -p rac_protocol --bin rac_decode -- logs/<session>/server_to_client.stream.bin
```

Мини‑клиент без `rac` (WIP):

```bash
cargo run -p rac_protocol --bin rac_lite -- agent-version 127.0.0.1:1545
cargo run -p rac_protocol --bin rac_lite -- cluster-list 127.0.0.1:1545
```

REST сервис для RAC (read‑only):

1) Настроить `rac_rest.toml` (пример в корне репозитория).
2) Запуск:

```bash
cargo run -p rac_rest -- --config rac_rest.toml
```

Эндпоинты (GET, «чистый» JSON):

1. `/agent/version`
2. `/clusters`
3. `/clusters/{cluster}`
4. `/clusters/{cluster}/managers`
5. `/clusters/{cluster}/managers/{manager}`
6. `/clusters/{cluster}/servers`
7. `/clusters/{cluster}/servers/{server}`
8. `/clusters/{cluster}/processes`
9. `/clusters/{cluster}/processes/{process}`
10. `/clusters/{cluster}/infobases/summary`
11. `/clusters/{cluster}/infobases/summary/{infobase}`
12. `/clusters/{cluster}/infobases/{infobase}`
13. `/clusters/{cluster}/connections`
14. `/clusters/{cluster}/connections/{connection}`
15. `/clusters/{cluster}/sessions`
16. `/clusters/{cluster}/sessions/{session}`
17. `/clusters/{cluster}/locks`
18. `/clusters/{cluster}/profiles`
19. `/clusters/{cluster}/counters`
20. `/clusters/{cluster}/counters/{counter}`
21. `/clusters/{cluster}/limits`
22. `/clusters/{cluster}/limits/{limit}`

Примеры:

```bash
curl http://127.0.0.1:8081/agent/version
curl http://127.0.0.1:8081/clusters
curl http://127.0.0.1:8081/clusters/550e8400-e29b-41d4-a716-446655440000
curl http://127.0.0.1:8081/clusters/550e8400-e29b-41d4-a716-446655440000/sessions
```

## Как использовать в исследовании

1. Прокси ставится между клиентом 1С и сервером.
2. Трафик сохраняется в `logs/`, раскладывается на пары request/response.
3. Дальше — анализ фрейминга, соответствия методов, построение карты команд в `docs/`.

## Статус

Проект экспериментальный, многое WIP. Форматы и утилиты будут меняться по мере накопления наблюдений.

## Полезные файлы

- `docs/documentation/rac_protocol_notes.md`
- `docs/documentation/rac_method_map.md`
