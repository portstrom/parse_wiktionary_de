// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

macro_rules! unwrap {
    ($context:ident $node:ident $warning_message:ident $value:expr) => {
        match $value {
            None => {
                ::add_warning($context, $node, ::WarningMessage::$warning_message);
                return None;
            }
            Some(value) => value,
        }
    };
}

pub fn parse_pos_template(
    context: &mut ::Context,
    template_node: &::Node,
    parameters: &[::Parameter],
) -> Option<::Pos> {
    if let [pos_parameter @ ::Parameter { name: None, .. }, language_parameter @ ::Parameter { name: None, .. }] =
        parameters
    {
        let pos = unwrap!(context pos_parameter ValueUnrecognized ::parse_text(&pos_parameter.value).and_then(|text| Some(match &text as _ {
            "Abkürzung" => ::Pos::Abbreviation,
            "Adjektiv" => ::Pos::Adjective,
            "Adverb" => ::Pos::Adverb,
            "Deklinierte Form" => ::Pos::DeclinedForm,
            "Eigenname" => ::Pos::ProperNoun,
            "Interjektion" => ::Pos::Interjection,
            "Konjugierte Form" => ::Pos::ConjugatedForm,
            "Konjunktion" => ::Pos::Conjunction,
            "Lokaladverb" => ::Pos::LocalAdverb,
            "Nachname" => ::Pos::LastName,
            "Numerale" => ::Pos::Numeral,
            "Postposition" => ::Pos::Postposition,
            "Präposition" => ::Pos::Preposition,
            "Redewendung" => ::Pos::Idiom,
            "Sprichwort" => ::Pos::Proverb,
            "Symbol" => ::Pos::Symbol,
            "Substantiv" => ::Pos::Noun,
            "Toponym" => ::Pos::Toponym,
            "Verb" => ::Pos::Verb,
            "Vorname" => ::Pos::FirstName,
            "Wortverbindung" => ::Pos::CompoundWord,
            _ => return None
        })));
        let language = unwrap!(context language_parameter ValueUnrecognized ::parse_text(&language_parameter.value).and_then(|text| ::Language::from_name(&text)));
        if Some(language) == context.language {
            return Some(pos);
        }
        ::add_warning(
            context,
            language_parameter,
            ::WarningMessage::ValueConflicting,
        );
    } else {
        ::add_warning(context, template_node, ::WarningMessage::ValueUnrecognized);
    }
    None
}
