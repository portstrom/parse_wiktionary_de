// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate parse_wiki_text;
extern crate parse_wiktionary_de;

fn main() {
    let mut args = std::env::args();
    if args.len() != 4 {
        eprintln!("invalid use");
        std::process::exit(1);
    }
    let command = args.nth(1).unwrap();
    let title = args.next().unwrap();
    let path = args.next().unwrap();
    let wiki_text = match &command as _ {
        "file" => match std::fs::read_to_string(path) {
            Err(error) => {
                eprintln!("Failed to read file: {}", error);
                std::process::exit(1);
            }
            Ok(file_contents) => file_contents,
        },
        "text" => path.replace("\\n", "\n"),
        _ => {
            eprintln!("invalid use");
            std::process::exit(1);
        }
    };
    let result = parse_wiktionary_de::create_configuration().parse(&wiki_text);
    if !result.warnings.is_empty() {
        eprintln!("Parse Wiki Text warnings: {:#?}", result.warnings);
    }
    let result = parse_wiktionary_de::parse(&title, &wiki_text, &result.nodes);
    println!("{:#?}", result);
    for warning in result.warnings {
        let mut warning_start = warning.start;
        while !wiki_text.is_char_boundary(warning_start) {
            warning_start -= 1;
        }
        let mut warning_end = warning.end;
        while !wiki_text.is_char_boundary(warning_end) {
            warning_end += 1;
        }
        let mut lines_remaining_start = 3;
        let mut snippet_start = warning_start;
        while snippet_start > 0 {
            if wiki_text.as_bytes()[snippet_start - 1] == b'\n' {
                if lines_remaining_start == 0 {
                    break;
                }
                lines_remaining_start -= 1;
            }
            snippet_start -= 1;
        }
        let mut lines_remaining_end = 3;
        let mut snippet_end = warning_end;
        while snippet_end < wiki_text.len() {
            if wiki_text.as_bytes()[snippet_end] == b'\n' {
                if lines_remaining_end == 0 {
                    break;
                }
                lines_remaining_end -= 1;
            }
            snippet_end += 1;
        }
        println!(
            "\n\x1b[9{color}m\x1b[1mwarning\x1b[m / \x1b[97mstart: {start}\x1b[m / \x1b[97mend: {end}\x1b[m / \x1b[97mlanguage: {language:?}\x1b[m / \x1b[97mmessage: {message:?}\x1b[m\n{snippet_start}\x1b[9{color}m{snippet_warning}\x1b[m{snippet_end}",
            color = if warning.message == parse_wiktionary_de::WarningMessage::Supplementary {
                '3'
            } else {
                '1'
            },
            start = warning.start,
            end = warning.end,
            language = warning.language,
            message = warning.message,
            snippet_start = &wiki_text[snippet_start..warning_start],
            snippet_warning = &wiki_text[warning_start..warning_end],
            snippet_end = &wiki_text[warning_end..snippet_end]
        );
    }
}
