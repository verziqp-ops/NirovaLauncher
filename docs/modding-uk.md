# Створення модів для Nirova / Horizon 26.3

Цей посібник пояснює два наявні формати модів. Лаунчер не перетворює Java-моди Fabric або NeoForge на сумісні Rust-моди автоматично.

## Який формат обрати

| Формат | Для кого | Що можна робити |
|---|---|---|
| `.hmod` | Початківець, без Rust | Блоки, предмети, рецепти, лут, теги, моделі, текстури, структури та інші data-driven ресурси |
| `.hnmod` | Розробник Rust | Логіка гри, події, серверні механіки, UI та інші зміни через native hooks / mixins |

Починай із `.hmod`. Він не виконує довільний код і не потребує компілятора Rust.

## Швидкий старт: мод без програмування

Потрібні Rust toolchain і встановлений клієнт Nirova/Horizon 26.3.

```sh
cargo install --git https://github.com/horizon-rust/horizon cargo-horizon-mod
cargo horizon-mod new my-first-mod --name "My First Mod"
cd my-first-mod
cargo horizon-mod build
cargo horizon-mod check target/horizon-mod/my_first_mod.hmod
```

Готовий файл буде в `target/horizon-mod/my_first_mod.hmod`. Для запуску в dev-режимі:

```sh
cargo horizon-mod run
```

Після цього відкрий гру з увімкненим dev-модом. Зміни в контент-моді застосовуються після перезапуску гри.

## Структура контент-мода

```text
my-first-mod/
├── horizon-mod.toml
├── assets/
│   └── my_first_mod/
│       └── textures/
└── data/
    └── my_first_mod/
        └── horizon/
            ├── block/
            ├── item/
            ├── recipe/
            └── loot_table/
```

Мінімальний маніфест:

```toml
id = "my_first_mod"
name = "My First Mod"
version = "0.1.0"
description = "My first Nirova content mod"
authors = ["Your name"]
api = "0.2"
side = "both"
capabilities = ["registries"]
```

Приклад блока на основі ванільного залізного блока:

```json
{
  "template": "minecraft:iron_block",
  "properties": {
    "hardness": 4.0,
    "light": 3,
    "sounds": "minecraft:amethyst_block"
  },
  "item": true
}
```

Файл блока збережи в `data/my_first_mod/horizon/block/<назва>.json`. Імена файлів та namespace мають відповідати правилам формату. Для повного переліку полів і обмежень дивись технічний довідник `docs/mods.md`.

## Моди з кодом на Rust

Native-моди мають формат `.hnmod` і збираються під конкретний commit гри. Вони можуть виконувати native code з правами користувача, тому встановлюй їх лише з джерел, яким довіряєш. Лаунчер показує попередження перед збіркою. Несумісні версії API або commit можуть зламати збірку.

Початковий шаблон і правила:
- `docs/native-mods.md`
- `horizon-native-mod-template`
- `horizon-mixins`

Після встановлення Rust toolchain відкрий шаблон у редакторі коду, вивчи `horizon-mixin.toml`, додай hook і збери мод за інструкцією шаблону. Для сумісності використовуй стабільні native hook points і вказуй потрібний рівень hooks API.

## Важливі обмеження

- Цільова версія цього лаунчера — 26.3; моди для інших версій не стають сумісними автоматично.
- Java JAR-моди Fabric/NeoForge не запускаються як є: вони залежать від Java-класів і своїх loader API.
- Контент-моди `.hmod` не можуть виконувати довільний Rust/Java-код.
- Native-моди `.hnmod` — не sandbox: перевіряй вихідний код і автора перед запуском.
- Не клади в мод файли Minecraft, на які ти не маєш права поширення.

## Що має стати простішим у Nirova

План наступного рівня інструментів лаунчера:
1. Майстер створення `.hmod` з формами замість ручного TOML/JSON.
2. Візуальний редактор блоків та предметів із live preview.
3. Перевірка маніфесту, namespace, текстур і рецептів із зрозумілими помилками.
4. Кнопки «Зібрати», «Перевірити» та «Запустити з модом».
5. Експорт `.hmod` і шаблон проєкту Rust для розробників native-модів.

Цей перелік є планом інструментів, а не заявою, що візуальний редактор уже реалізовано.
