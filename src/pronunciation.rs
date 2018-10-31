// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

#[derive(Default)]
pub struct Pronunciation<'a> {
    pub audio: Vec<::Flowing<'a>>,
    pub ipa: Vec<::Flowing<'a>>,
    pub rhymes: Vec<::Flowing<'a>>,
}

pub fn parse_pronunciation<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter],
    nodes: &[::Node<'a>],
    output: &mut Option<Pronunciation<'a>>,
) -> usize {
    ::parse_list_generic(
        context,
        template_node,
        parameters,
        nodes,
        output,
        |context, items| {
            let mut audio = None;
            let mut ipa = None;
            let mut rhymes = None;
            for item in items {
                if item.type_ != ::Details {
                    ::add_warning(context, item, ::WarningMessage::Unrecognized);
                    continue;
                }
                match item.nodes.get(0) {
                    None => {
                        ::add_warning(context, item, ::WarningMessage::Empty);
                        continue;
                    }
                    Some(node) => {
                        if let ::Node::Template {
                            name, parameters, ..
                        } = node
                        {
                            if let Some(text) = ::parse_text(name) {
                                match &text as _ {
                                    "Hörbeispiele" => {
                                        parse_audio(context, item, node, parameters, &mut audio);
                                        continue;
                                    }
                                    "IPA" => {
                                        parse_audio(context, item, node, parameters, &mut ipa);
                                        continue;
                                    }
                                    "Reime" => {
                                        parse_audio(context, item, node, parameters, &mut rhymes);
                                        continue;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        ::add_warning(context, node, ::WarningMessage::Unrecognized);
                    }
                }
            }
            Pronunciation {
                audio: audio.unwrap_or_default(),
                ipa: ipa.unwrap_or_default(),
                rhymes: rhymes.unwrap_or_default(),
            }
        },
    )
}

fn parse_audio<'a>(
    context: &mut ::Context<'a>,
    list_item: &::DefinitionListItem<'a>,
    template_node: &::Node,
    parameters: &[::Parameter],
    output: &mut Option<Vec<::Flowing<'a>>>,
) {
    if output.is_some() {
        *output = Some(vec![]);
        ::add_warning(context, template_node, ::WarningMessage::Duplicate);
        return;
    }
    if !parameters.is_empty() {
        *output = Some(vec![]);
        ::add_warning(context, template_node, ::WarningMessage::ValueUnrecognized);
        return;
    }
    let mut output_nodes = vec![];
    for node in &list_item.nodes[1..] {
        match node {
            ::Node::Template {
                name, parameters, ..
            } => if let Some(name) = ::parse_text(name) {
                match &name as _ {
                    "Audio" => {
                        output_nodes.push(parse_template_audio(context, node, parameters));
                        continue;
                    }
                    "Gen." => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::Genitive,
                        ));
                        continue;
                    }
                    "Lautschrift" => {
                        output_nodes.push(parse_template_ipa(context, node, parameters));
                        continue;
                    }
                    "Part." => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::PastParticiple,
                        ));
                        continue;
                    }
                    "Pl." => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::Plural,
                        ));
                        continue;
                    }
                    "Pl.1" => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::Plural1,
                        ));
                        continue;
                    }
                    "Pl.2" => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::Plural2,
                        ));
                        continue;
                    }
                    "Pl.3" => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::Plural3,
                        ));
                        continue;
                    }
                    "Pl.4" => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::Plural4,
                        ));
                        continue;
                    }
                    "Prät." => {
                        output_nodes.push(::parse_simple_template(
                            context,
                            node,
                            parameters,
                            ::Flowing::Preterite,
                        ));
                        continue;
                    }
                    "Reim" => {
                        output_nodes.push(parse_template_rhyme(context, node, parameters));
                        continue;
                    }
                    _ => {}
                }
            },
            ::Node::Text { mut value, .. } => {
                if output_nodes.is_empty() {
                    value = value.trim_left();
                    if value.is_empty() {
                        continue;
                    }
                }
                output_nodes.push(::Flowing::Text {
                    value: ::Cow::Borrowed(value),
                });
                continue;
            }
            _ => {}
        }
        output_nodes.push(::create_unknown(
            context,
            node,
            ::WarningMessage::Unrecognized,
        ));
    }
    if output_nodes.is_empty() {
        ::add_warning(context, template_node, ::WarningMessage::SectionEmpty);
    }
    *output = Some(output_nodes);
}

macro_rules! parse_parameter {
    ($output:tt $context:tt $template_node:tt $parameter:tt) => {{
        $output = ::parse_text_not_empty(&$parameter.value);
        if $output.is_none() {
            return ::create_unknown2(
                $context,
                $template_node,
                $parameter,
                ::WarningMessage::ValueUnrecognized,
            );
        }
        continue;
    }};
}

fn parse_template_audio<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> ::Flowing<'a> {
    if let [::Parameter {
        name: None, value, ..
    }] = parameters
    {
        if value.is_empty() {
            return ::Flowing::EmptyAudio;
        }
    }
    let mut file_name = None;
    let mut label = None;
    let mut language = None;
    let mut parameter_index = 0;
    for parameter in parameters {
        match &parameter.name {
            None => {
                parameter_index += 1;
                match parameter_index {
                    1 => parse_parameter!(file_name context template_node parameter),
                    2 => parse_parameter!(label context template_node parameter),
                    _ => {}
                }
            }
            Some(_) => if ::parse_parameter_name(parameter) == Some("spr") {
                if language.is_some() {
                    return ::create_unknown2(
                        context,
                        template_node,
                        parameter,
                        ::WarningMessage::Duplicate,
                    );
                }
                parse_parameter!(language context template_node parameter)
            },
        }
        return ::create_unknown2(
            context,
            template_node,
            parameter,
            ::WarningMessage::Unrecognized,
        );
    }
    match file_name {
        None => ::create_unknown(context, template_node, ::WarningMessage::Empty),
        Some(file_name) => ::Flowing::Audio {
            file_name,
            label,
            language,
        },
    }
}

fn parse_template_ipa<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> ::Flowing<'a> {
    if let [parameter @ ::Parameter { name: None, .. }] = parameters {
        if let Some(ipa) = ::parse_text_not_empty(&parameter.value) {
            return ::Flowing::Ipa { ipa };
        }
        match ::parse_text_not_empty(&parameter.value) {
            None => ::create_unknown2(
                context,
                template_node,
                parameter,
                ::WarningMessage::ValueUnrecognized,
            ),
            Some(ipa) => ::Flowing::Ipa { ipa },
        }
    } else {
        ::create_unknown(context, template_node, ::WarningMessage::ValueUnrecognized)
    }
}

fn parse_template_rhyme<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> ::Flowing<'a> {
    if let [rhyme_parameter @ ::Parameter { name: None, .. }, language_parameter @ ::Parameter { name: None, .. }] =
        parameters
    {
        match ::parse_text_not_empty(&rhyme_parameter.value) {
            None => ::create_unknown2(
                context,
                template_node,
                rhyme_parameter,
                ::WarningMessage::ValueUnrecognized,
            ),
            Some(rhyme) => match ::parse_text(&language_parameter.value)
                .and_then(|text| ::Language::from_name(&text))
            {
                None => ::create_unknown2(
                    context,
                    template_node,
                    language_parameter,
                    ::WarningMessage::ValueUnrecognized,
                ),
                Some(language) => if Some(language) == context.language {
                    ::Flowing::Rhyme { rhyme }
                } else {
                    ::create_unknown2(
                        context,
                        template_node,
                        language_parameter,
                        ::WarningMessage::ValueConflicting,
                    )
                },
            },
        }
    } else {
        ::create_unknown(context, template_node, ::WarningMessage::ValueUnrecognized)
    }
}
