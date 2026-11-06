# Аудит готовності NIROVA

Дата: 2026-11-06  
Гілка: nivrora/full-rebrand-russian-home  
Commit до аудиту: dad10e32e20d772e7eef31542d46a84c3e62ab9a

Це аудит структури й коду репозиторію. Він не є підтвердженням успішної збірки або повного проходження ігрових тестів.

## Висновок

NIROVA містить великий нативний Rust-клієнт/серверний код і окремий лаунчер. Це не лише оболонка над Java Edition. Але наявність модуля не доводить повну реалізацію кожної механіки.

| Напрям | Фактичний стан | Що ще потрібно |
|---|---|---|
| Rust game client | Є crates/horizon-client і великий game loop | Перевірити повноту механік і parity tests |
| Tick loop | Game::tick_world виконує системи послідовно | Необхідна інтеграція worker jobs або regionized scheduler |
| Region scheduler | horizon-threads::region має executor immutable snapshots і стабільний порядок результатів | Немає підключення до Game::tick_world, regionizer, merge/split та ownership checks |
| Redstone / block ticks | Scheduled block/fluid ticks і пов'язані системи виконуються в послідовному циклі | Не переносити їх у worker threads без доведеного протоколу |
| Запрошення друзів | Є horizon-friends, Host, Guest, invite code, direct/relay tunnel | Потрібні end-to-end тести на реальних мережевих сценаріях |
| Singleplayer sharing | Start Sharing відкриває світ у LAN і створює invite code | Потрібне тестування повного сценарію |
| Dedicated server sharing | Dedicated server код існує окремо | Поточний friends_start прямо відхиляє dedicated server; потрібен окремий host-flow |
| Автосинхронізація модів | Є перевірка сумісності контенту | Автозавантаження модів гостю не підтверджене; потрібні manifest, hash/signature checks і згода гравця |
| Java mods | Документація описує native Rust mods та data/content mods | Fabric/Forge/NeoForge Java bytecode напряму не запускається |
| Launcher / installer | Є GPUI launcher і workflow-и збірки | Потрібно перевірити всі екрани та актуальний CI; готовий Windows installer не підтверджений |

## Важливі технічні знахідки

### 1. Folia-подібна багатопоточність

У crates/horizon-threads/src/region.rs є виконавець задач зі знімками. Він запускає незалежні обчислення паралельно і повертає результати в канонічному порядку. Це інфраструктура для worker jobs, а не повна реалізація Folia.

У crates/horizon-client/src/game.rs функція Game::tick_world послідовно викликає tick_blocks, tick_mobs, tick_items, tick_block_entities, портали та інші системи. У цьому файлі немає використання RegionScheduler.

Тому вмикати регіональний tick зараз небезпечно. Спочатку потрібні:
1. Відтворювані tick fixtures і digest стану світу.
2. Профілювання CPU та MSPT, щоб знайти справжні bottleneck-и.
3. Read-only snapshots і версії секцій.
4. Паралельні pathfinding/sensor tasks із commit у початковому порядку.
5. Синхронний fallback, якщо дані snapshot застаріли.
6. Лише потім regionizer, owner checks, merge/split, entity handoff і global coordinator.

Редстоун, scheduled block/fluid ticks, neighbor updates, pistons і block entities не можна просто рознести по потоках: це може змінити порядок імпульсів, RNG та побічних ефектів.

### 2. Гра з друзями

Singleplayer host-flow уже є: game/friends.rs відкриває LAN і запускає horizon-friends Host. horizon-friends має direct connection, шифрування, invite code і optional relay; Guest створює локальний tunnel для підключення клієнта.

Але friends_start містить перевірку, яка відхиляє сценарій, якщо відкрито dedicated server або немає singleplayer-світу. Для dedicated server потрібен окремий контрольований host-flow з явним дозволом адміністратора.

### 3. Автосинхронізація модів

docs/mods.md описує контентні моди .hmod і нативні моди .hnmod. Документація також каже, що клієнт із відмінним набором контенту відхиляється під час підключення. Автоматичне завантаження відсутніх модів між host і guest не описане як готова функція.

Безпечна реалізація має передавати manifest (id, версія, API, сторона, SHA-256, розмір і джерело), показувати гостю список потрібних файлів, вимагати згоду, перевіряти хеш/підпис і не виконувати native code непомітно.

## Порядок реалізації

### P0 — базова перевірка
- Отримати зелений CI на актуальному commit.
- Запустити cargo tests для region scheduler, friends, game parity та launcher.
- Зафіксувати failing tests і виправити їх до додавання нових систем.

### P1 — друзі й моди
- End-to-end: host start → code publish → guest join → пакетний обмін → disconnect/stop.
- Перевірити direct, relay-only, wrong code, expired invite, version mismatch, full server і NAT failure.
- Додати окремий режим sharing для dedicated server.
- Реалізувати manifest handshake і безпечне завантаження модів лише після згоди користувача.

### P2 — безпечна багатопоточність
- Профілювання tick sections.
- Read-only snapshots, version checks і fallback.
- Паралельні pathfinding/sensor tasks.
- Differential tests проти послідовного еталона.

### P3 — експериментальний regionized mode
- Regionizer окремо для кожного dimension.
- Один власник запису для chunk/entity state.
- Буфер безпеки між активними регіонами.
- Черги між регіонами з чіткою tick boundary.
- Entity scheduler, teleport/handoff, global coordinator.
- Merge/split тільки на бар'єрах.
- Тести redstone, fluids, pistons, hoppers, mobs, portals і save/reload.
- Режим вимкнений за замовчуванням до проходження тестів.

### P4 — лаунчер і реліз
- Перевірити локалізації, екрани, стани помилок і налаштування.
- Перевірити імпорт версій, модів і модпаків.
- Перевірити friend join із лаунчера до гри.
- Після зеленого CI зібрати Windows installer і перевірити його на чистій Windows VM. Лише після цього публікувати реліз.

## Критерії готовності

- CI зелений на актуальному commit.
- Інтеграційні тести friends проходять на direct і relay сценаріях.
- Vanilla-exact mode зберігає digest послідовного еталона на тестових сценаріях.
- Worker не змінює live world поза потоком-власником.
- Синхронізація модів вимагає явної згоди та перевірки походження.
- Dedicated-server sharing відрізняється від singleplayer sharing.
- Installer запускається на чистій Windows і реліз містить перевірені артефакти.

## Файли, на яких базується аудит

- crates/horizon-client/src/game.rs — Game::tick_world.
- crates/horizon-threads/src/region.rs — executor immutable region jobs.
- docs/region-threading-ru.md — поточний план багатопоточності.
- crates/horizon-client/src/game/friends.rs — singleplayer host-flow.
- crates/horizon-friends/src/host.rs і guest.rs — host/guest transport.
- docs/play-with-friends.md — опис invite flow.
- docs/mods.md і docs/modpacks.md — формати модів та портинг.
- PR #2: https://github.com/verziqp-ops/NirovaLauncher/pull/2

## Офіційний орієнтир Folia

- https://docs.papermc.io/folia/reference/overview/
- https://docs.papermc.io/folia/reference/region-logic/

Folia описує незалежні регіони з власним tick loop, правилами ownership, merge/split і міжрегіональними чергами. Це архітектурний орієнтир, а не доказ, що NIROVA вже реалізувала Folia.
