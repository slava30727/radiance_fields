# Radiance Fields

## Сборка

Для сборки проекта необходимы линковщик `clang` и инструменты разработки на `rust` (rust toolchain), при их отсутствии:

```shell
sudo apt install rustup
rustup install stable
cargo --version
```

```output
cargo 1.77.2 (e52e36006 2024-03-26)
```

Сборка консольного приложения в режиме release. Для первой сборки понадобится подключение к интернету, чтобы `cargo` имел возможность подгрузить все необходимые библиотеки.

```shell
cargo build --release
```

Приложение будет находится в каталоге `target/release/` под названием `radiance_fields.exe`. Для запуска приложения в режиме замера производительности необходимо указать флаг `--bench` или `-b`. По умолчанию рендеринг изображения происходит с помощью GPU и Vulkan, но можно и в однопоточном режиме на CPU `--type singlecpu`, и в многопоточном режиме CPU `--type multicpu`. Для дополнительной информации по всем доступным флагам `--help`. По умолчанию в директории `assets/` должен лежать файл `render_configuration.toml`. После успешной работы программы в директории `output/` появится файл `result.png` (его название и директорию можно менять флагом `--out <DIR>`). Можно указать цель рендеринга: цвет и плотности (`--target color` и `--target density`).

### Пример

```shell
target/release/radiance_fields --help
target/release/radiance_fields --type gpu --bench
```

## Зависимости

Проект использует несколько библиотек (их список с версиями есть в файле `Cargo.toml`), в том числе:

1. `anyhow` - упрощённая работа с ошибками.
2. `bincode` - бинарная сериализация/десериализация типов Rust.
3. `bytemuck` - позволяет кроссплатформено безопасно преобразовывать данные в байты и обратно без копирования.
4. `clap` - позволяет очень просто парсить аргументы командной строки.
5. `glam` - предоставляет векторные типы и возможную из реализацию на SIMD (в зависимости от платформы).
6. `kdam` - простые индикаторы загрузки.
7. `png` - позволяет записывать картинки в файл в формате `.png`.
8. `rayon` - реализует "параллельные итераторы", что позволяет эффективно реализовать параллельный рендеринг на CPU.
9. `serde` - интерфейс сериализации/десериализации для Rust.
10. `thiserror` - позволяет удобно работать с ошибками в Rust.
11. `tokio` - предоставляет асинхронный runtime для Rust.
12. `wgpu` - предоставляет доступ к графическому адаптеру, его бэкэнд строго установлен на Vulkan.
13. `toml` - реализует интерфейс `serde` для сериализации/десериализации в формате `.toml`.

## Отчёт о реализации

1. Реализован прямой проход сети на CPU и GPU.
2. "Ближайшее" сэмплирование сетки на CPU.
3. Сэмплирование с трилинейной фильтрацией на CPU и GPU.
4. Чёрно-белый рендер плотностей в ячейках (флаг `--target density`)
5. Цветной рендер. (флаг `--target color` стоит по умолчанию)
6. Замерено время выполнения на CPU и GPU с учётом копирования и без.

## Отчёт о производительности

Вычислен наименьший результат по двум запускам на моём HUAWEI MateBook 16s / Intel i7 13700H / Intel Iris Xe Graphics / 16 GB.
В полях строки GPU <время рендеринга> + <время копирования>.

| Вычислитель/Разрешение | 256х256         | 512х512         | 1024х1024      | 2048х2048       |
| ---------------------- | --------------- | --------------- | -------------- | --------------- |
| CPU (1 thread)         | 2.24 с          | 7.4 с           | 27.16 с        | 107.26 c        |
| CPU (multithread)      | 224.22 мс       | 757.77 мс       | 2.84 с         | 11.28 с         |
| GPU                    | 635 мс + 3.78 с | 1.11 с + 2.58 с | 2.6 с + 2.46 с | 6.85 с + 2.92 c |

### Замечание

Можно заметить, что время копирования необычно большое, это потому что исходная модель в целях рендеринга на девайсе разрезается в набор текстур, размеры которых уже допустимы в Vulkan (исходная модель объёмом 1.75Гб).
