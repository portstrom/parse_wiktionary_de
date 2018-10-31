// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use parse_wiki_text::Positioned;

// XXX Check for duplicates.

pub fn parse_language<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    language_entries: &mut Vec<::LanguageEntry<'a>>,
) -> usize {
    let mut node_index = 0;
    let mut pos_entries = vec![];
    while let Some(node) = nodes.get(node_index) {
        match node {
            ::Node::Heading {
                level,
                nodes: heading_child_nodes,
                ..
            } => {
                if *level < 3 {
                    break;
                }
                if *level == 3 {
                    node_index += 1;
                    match heading_child_nodes.get(0) {
                        None => {
                            ::add_warning(context, node, ::WarningMessage::Empty);
                            continue;
                        }
                        Some(template_node) => if let ::Node::Template {
                            name, parameters, ..
                        } = template_node
                        {
                            if ::text_equals(name, "Wortart") {
                                if let Some(pos) = ::pos_template::parse_pos_template(
                                    context,
                                    template_node,
                                    parameters,
                                ) {
                                    let mut details =
                                        parse_details(context, &heading_child_nodes[1..]);
                                    node_index += ::pos_section::parse_pos_section(
                                        context,
                                        &nodes[node_index..],
                                        &mut pos_entries,
                                        pos,
                                        details,
                                    );
                                }
                                continue;
                            }
                        },
                    }
                    ::add_warning(context, node, ::WarningMessage::ValueUnrecognized);
                    continue;
                }
            }
            ::Node::Template { name, .. } => if ::text_equals(name, "erweitern") {
                node_index += 1;
                ::add_warning(context, node, ::WarningMessage::Supplementary);
                continue;
            },
            _ => {}
        }
        node_index += 1;
        ::add_warning(context, node, ::WarningMessage::Unrecognized);
    }
    if pos_entries.is_empty() {
        ::add_warning(context, heading_node, ::WarningMessage::SectionEmpty);
    } else {
        language_entries.push(::LanguageEntry {
            language: context.language.unwrap(),
            pos_entries,
        });
    }
    node_index
}

fn parse_details<'a>(context: &mut ::Context<'a>, nodes: &[::Node<'a>]) -> Vec<::Flowing<'a>> {
    let mut details = vec![];
    for node in nodes {
        match node {
            ::Node::Italic { .. } => details.push(::Flowing::Italic),
            ::Node::Link { target, text, .. } => {
                details.push(::parse_link(context, node, target, text))
            }
            ::Node::Template {
                name, parameters, ..
            } => match ::parse_text(name) {
                None => details.push(::create_unknown(
                    context,
                    node,
                    ::WarningMessage::Unrecognized,
                )),
                Some(text) => match &text as _ {
                    "Wortart" => details.push(match ::pos_template::parse_pos_template(
                        context, node, parameters,
                    ) {
                        None => ::Flowing::Unknown {
                            value: ::Cow::Borrowed(&context.wiki_text[node.start()..node.end()]),
                        },
                        Some(pos) => ::Flowing::Pos { pos },
                    }),
                    "f" => details.push(::parse_simple_template(
                        context,
                        node,
                        parameters,
                        ::Flowing::FeminineGender,
                    )),
                    "fm" => parse_gender_combination_template(
                        context,
                        node,
                        parameters,
                        &mut details,
                        ::Flowing::FeminineGender,
                        ::Flowing::MasculineGender,
                    ),
                    "fn" => parse_gender_combination_template(
                        context,
                        node,
                        parameters,
                        &mut details,
                        ::Flowing::FeminineGender,
                        ::Flowing::NeuterGender,
                    ),
                    "m" => details.push(::parse_simple_template(
                        context,
                        node,
                        parameters,
                        ::Flowing::MasculineGender,
                    )),
                    "mf" => parse_gender_combination_template(
                        context,
                        node,
                        parameters,
                        &mut details,
                        ::Flowing::MasculineGender,
                        ::Flowing::FeminineGender,
                    ),
                    "mn." => parse_gender_combination_template(
                        context,
                        node,
                        parameters,
                        &mut details,
                        ::Flowing::MasculineGender,
                        ::Flowing::NeuterGender,
                    ),
                    "n" => details.push(::parse_simple_template(
                        context,
                        node,
                        parameters,
                        ::Flowing::NeuterGender,
                    )),
                    "nf" => parse_gender_combination_template(
                        context,
                        node,
                        parameters,
                        &mut details,
                        ::Flowing::NeuterGender,
                        ::Flowing::FeminineGender,
                    ),
                    "nm" => parse_gender_combination_template(
                        context,
                        node,
                        parameters,
                        &mut details,
                        ::Flowing::NeuterGender,
                        ::Flowing::MasculineGender,
                    ),
                    "u" => details.push(::parse_simple_template(
                        context,
                        node,
                        parameters,
                        ::Flowing::CommonGender,
                    )),
                    _ => details.push(::create_unknown(
                        context,
                        node,
                        ::WarningMessage::Unrecognized,
                    )),
                },
            },
            ::Node::Text { mut value, .. } => {
                if details.is_empty() {
                    value = value.trim_left();
                    if value.starts_with(',') {
                        value = value[1..].trim_left();
                    }
                    if value.is_empty() {
                        continue;
                    }
                }
                details.push(::Flowing::Text {
                    value: ::Cow::Borrowed(value),
                });
            }
            _ => details.push(::create_unknown(
                context,
                node,
                ::WarningMessage::Unrecognized,
            )),
        }
    }
    details
}

fn parse_gender_combination_template<'a>(
    context: &mut ::Context<'a>,
    node: &::Node,
    parameters: &[::Parameter],
    output: &mut Vec<::Flowing<'a>>,
    gender1: ::Flowing<'a>,
    gender2: ::Flowing<'a>,
) {
    if parameters.is_empty() {
        output.push(gender1);
        output.push(::Flowing::Text {
            value: ::Cow::Borrowed(", "),
        });
        output.push(gender2);
    } else {
        output.push(::create_unknown(
            context,
            node,
            ::WarningMessage::ValueUnrecognized,
        ))
    }
}
