# MD2BB
Утилита, транслирующая Markdown разметку в аналогичный BBcode.

## Загрузки
В [релизах](https://github.com/White-Oak/md2bb/releases)

## Usage
```
# транслирует text.md в text.bb
md2bb
```
```
md2bb -o README.bb README.md
```
```
md2bb -r rules.csv -o README.bb README.md
```
Опция -h выведет помощь.

## Компиляция
Требования: Rust 1.5+, Cargo.
```
# Собирает бинарник в ./target/release/md2bb
cargo build --release
```

## Спойлеры в Markdown

В MD нет спойлеров, но они могут быть полезны для форматтирования статьи в BB. Ниже указано, как, получив симпатичную MD разметку, работать со спойлерами в трансляторе.
```
#### Название спойлера
Это текст спойлера. Он не очень большой, но у него громадные амбиции.

А тут обычный текст.
```
```
[spoiler Название спойлера]
Это текст спойлера. Он не очень большой, но у него громадные амбиции.[/spoiler]
А тут обычный текст.
```
Обратите внимание, что за текстом спойлера **должна быть** пустая строка в MD.

## Правила

Правила описываются парами `регулярка, строка-заменитель` в rules.csv. Редактируйте их по своим желаниям.
При сборке из исходников, предоставленный rules.csv включается в сборку, поэтому его не нужно копировать вместе с исполняемым файлом.
