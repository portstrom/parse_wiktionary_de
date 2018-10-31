// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_overview<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    name: ::Cow<'a, str>,
    parameters: &[::Parameter<'a>],
    output: &mut Option<Option<::Overview<'a>>>,
) -> bool {
    match (context.language.unwrap(), &name as _) {
        (::Language::De, "Bairisch Substantiv Übersicht m")
        | (::Language::De, "Bairisch Substantiv Übersicht n")
        | (::Language::De, "Bairisch Verb Übersicht")
        | (::Language::De, "Deutsch Adjektiv Übersicht")
        | (::Language::De, "Deutsch Adverb Übersicht")
        | (::Language::De, "Deutsch Eigenname Übersicht")
        | (::Language::De, "Deutsch Nachname Übersicht")
        | (::Language::De, "Deutsch Personalpronomen 1")
        | (::Language::De, "Deutsch Personalpronomen 2")
        | (::Language::De, "Deutsch Personalpronomen 3")
        | (::Language::De, "Deutsch Personalpronomen Berliner Dialekt")
        | (::Language::De, "Deutsch Pronomen Übersicht")
        | (::Language::De, "Deutsch Substantiv Dialekt")
        | (::Language::De, "Deutsch Substantiv Übersicht")
        | (::Language::De, "Deutsch Substantiv Übersicht -sch")
        | (::Language::De, "Deutsch Toponym Übersicht")
        | (::Language::De, "Deutsch Verb Übersicht")
        | (::Language::De, "Deutsch adjektivisch Übersicht")
        | (::Language::De, "Kardinalzahl 2-12")
        | (::Language::De, "Possessivpronomina-Tabelle")
        | (::Language::De, "Pronomina-Tabelle")
        | (::Language::En, "Englisch Adjektiv Übersicht")
        | (::Language::En, "Englisch Personalpronomen 2")
        | (::Language::En, "Englisch Personalpronomen")
        | (::Language::En, "Englisch Substantiv Übersicht")
        | (::Language::En, "Englisch Verb Übersicht") => {}
        _ => return false,
    }
    if output.is_some() {
        *output = Some(None);
        ::add_warning(context, template_node, ::WarningMessage::Duplicate);
        return true;
    }
    let mut named_parameters = ::HashMap::new();
    let mut unnamed_parameters = vec![];
    for parameter in parameters {
        match parameter.name {
            None => unnamed_parameters.push(
                parameter
                    .value
                    .iter()
                    .map(|node| match node {
                        ::Node::Italic { .. } => ::Flowing::Italic,
                        ::Node::Text { value, .. } => ::Flowing::Text {
                            value: ::Cow::Borrowed(value),
                        },
                        _ => ::create_unknown(context, node, ::WarningMessage::Unrecognized),
                    })
                    .collect(),
            ),
            Some(_) => match ::parse_parameter_name(parameter) {
                None => ::add_warning(context, parameter, ::WarningMessage::Unrecognized),
                Some(name) => {
                    if named_parameters.contains_key(name) {
                        ::add_warning(context, parameter, ::WarningMessage::Duplicate);
                    }
                    match ::parse_text(&parameter.value) {
                        None => {
                            ::add_warning(context, parameter, ::WarningMessage::ValueUnrecognized)
                        }
                        Some(value) => {
                            named_parameters.insert(::Cow::Borrowed(name), value);
                        }
                    }
                }
            },
        }
    }
    *output = Some(Some(::Overview {
        name,
        named_parameters,
        unnamed_parameters,
    }));
    true
}
