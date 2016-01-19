# MD2BB
The tool translates Markdown files into BB code.

## Downloads
At [downloads]() section

## Usage
```
# translates text.md to text.bb
md2bb
```
```
md2bb -o README.bb README.md
```
```
md2bb -r rules.csv -o README.bb README.md
```
-h flag will print help notes.

## Compilation
Requirements: Rust 1.5+, Cargo.
```
# Builds to ./target/release/md2bb
cargo build --release
```

## Spoilers in Markdown

There are no spoilers in MD, though, they can be useful when formatting your article in BB. The following showshow to use neat MD text and convert it to spoilers in BB.
```
#### Spolier name here
That's a spoiler text. It's not very big, but it has great ambitions.

Just a casual text here.
```
```
[spoiler Spolier name here]
That's a spoiler text. It's not very big, but it has great ambitions.[/spoiler]
Just a casual text here.
```
Note that an empty newline should follow spoiler text in Markdown.

## Rules

Rules are described as a pair of `regex, replacement string` in rules.csv. Edit them to your likes.
Note that rules.csv is included in binary, when building from sources, so there is no need to carry it around.