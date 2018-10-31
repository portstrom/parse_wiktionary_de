// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_examples<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter],
    nodes: &[::Node<'a>],
    output: &mut Option<Vec<::Example<'a>>>,
) -> usize {
    ::parse_list_items_generic(
        context,
        template_node,
        parameters,
        nodes,
        output,
        |context, list_item| parse_example(context, list_item),
    )
}

fn parse_example<'a>(
    context: &mut ::Context<'a>,
    list_item: &::DefinitionListItem<'a>,
) -> Option<::Example<'a>> {
    let mut example = vec![];
    let mut has_text = false;
    let mut translation = vec![];
    let mut iterator = list_item.nodes.iter();
    while let Some(node) = iterator.next() {
        match node {
            ::Node::Italic { .. } => example.push(::Flowing::Italic),
            ::Node::Tag { name, .. } if name == "ref" => {
                ::add_warning(context, node, ::WarningMessage::Supplementary);
                example.push(::Flowing::Reference);
            }
            ::Node::Text { value, .. } => {
                has_text = true;
                example.push(::Flowing::Text {
                    value: ::Cow::Borrowed(value),
                });
            }
            ::Node::DefinitionList { items, .. } => {
                if let [list_item @ ::DefinitionListItem {
                    type_: ::Details, ..
                }] = items.as_slice()
                {
                    if list_item.nodes.is_empty() {
                        ::add_warning(context, list_item, ::WarningMessage::Empty);
                    } else {
                        translation = list_item
                            .nodes
                            .iter()
                            .map(|node| match node {
                                ::Node::Italic { .. } => ::Flowing::Italic,
                                ::Node::Text { value, .. } => ::Flowing::Text {
                                    value: ::Cow::Borrowed(value),
                                },
                                _ => {
                                    ::create_unknown(context, node, ::WarningMessage::Unrecognized)
                                }
                            })
                            .collect();
                    }
                } else {
                    ::add_warning(context, node, ::WarningMessage::ValueUnrecognized);
                }
                while let Some(node) = iterator.next() {
                    ::add_warning(context, node, ::WarningMessage::Unrecognized);
                }
                break;
            }
            _ => example.push(::create_unknown(
                context,
                node,
                ::WarningMessage::Unrecognized,
            )),
        }
    }
    if has_text {
        Some(::Example {
            example,
            translation,
        })
    } else {
        ::add_warning(context, list_item, ::WarningMessage::Empty);
        None
    }
}
