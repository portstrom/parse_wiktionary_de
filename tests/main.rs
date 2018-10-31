// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate parse_wiki_text;
extern crate parse_wiktionary_de;

#[test]
fn main() {
    let _ = parse_wiktionary_de::parse(
        "",
        "",
        &parse_wiktionary_de::create_configuration().parse("").nodes,
    );
}
