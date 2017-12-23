# opencorpora

[![Build Status](https://travis-ci.org/antage/opencorpora.svg?branch=master)](https://travis-ci.org/antage/opencorpora)

Данная библиотека написана для парсинга словаря проекта [opencorpora.org](http://opencorpora.org/dict.php). Библиотека читает словарь в XML-формате и формирует данные в виде набора структур.

[Документация](https://docs.rs/opencorpora)

## Ссылки
* [Формат словаря](http://opencorpora.org/?page=export).
* [Список граммем](http://opencorpora.org/dict.php?act=gram).
* [Ограничения на граммемы](http://opencorpora.org/dict.php?act=gram_restr).

## Пример использования

```rust
extern crate opencorpora;

use opencorpora::Dict;

fn main() {
    use std::fs::File;

    let file = File::open("dict.xml").unwrap();
    match Dict::read_from_xml(&file) {
        Ok(dict) => {
            println!("Version: {}", dict.version);
            println!("Revision: {}", dict.revision);
            println!("Grammemes count: {}", dict.grammemes.len());
            println!("Restrictions count: {}", dict.restrictions.len());
            println!("Lemmata count: {}", dict.lemmata.len());
            println!("Link types count: {}", dict.link_kinds.len());
            println!("Links count: {}", dict.links.len());
        },
        Err(e) => {
            println!("ERROR: {}", e);
        },
    }
}
```
