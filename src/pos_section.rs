// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_pos_section<'a>(
    context: &mut ::Context<'a>,
    nodes: &[::Node<'a>],
    pos_entries: &mut Vec<::PosEntry<'a>>,
    pos: ::Pos,
    details: Vec<::Flowing<'a>>,
) -> usize {
    let mut abbreviations = None;
    let mut affectionate_forms = None;
    let mut antonyms = None;
    let mut compound_words = None;
    let mut definitions = None;
    let mut diminutives = None;
    let mut etymology = None;
    let mut examples = None;
    let mut feminine_forms = None;
    let mut hypernyms = None;
    let mut hyphenation = None;
    let mut hyponyms = None;
    let mut idioms = None;
    let mut masculine_forms = None;
    let mut no_longer_valid_spellings = None;
    let mut node_index = 0;
    let mut overview = None;
    let mut pronunciation = None;
    let mut proverbs = None;
    let mut related_words = None;
    let mut short_forms = None;
    let mut similar_words = None;
    let mut symbols = None;
    let mut synonyms = None;
    let mut translations = false;
    let mut typical_word_combinations = None;
    let mut variants = None;
    while let Some(node) = nodes.get(node_index) {
        match node {
            ::Node::Heading {
                level,
                nodes: heading_child_nodes,
                ..
            } => {
                if *level < 4 {
                    break;
                }
                node_index += 1;
                if *level == 4 {
                    if let [::Node::Template {
                        name, parameters, ..
                    }] = heading_child_nodes.as_slice()
                    {
                        if ::text_equals(name, "Übersetzungen") {
                            if translations {
                                ::add_warning(context, node, ::WarningMessage::Duplicate);
                                return 0;
                            }
                            translations = true;
                            if !parameters.is_empty() {
                                ::add_warning(context, node, ::WarningMessage::ValueUnrecognized);
                                continue;
                            }
                            if let Some(node) = nodes.get(node_index) {
                                if let ::Node::Template { name, .. } = node {
                                    if ::text_equals(name, "Ü-Tabelle") {
                                        node_index += 1;
                                        ::add_warning(
                                            context,
                                            node,
                                            ::WarningMessage::Supplementary,
                                        );
                                        continue;
                                    }
                                }
                            }
                            ::add_warning(context, node, ::WarningMessage::SectionEmpty);
                            continue;
                        }
                    }
                }
            }
            ::Node::Template {
                name, parameters, ..
            } => {
                macro_rules! section {
                    ($output:tt $function:path) => {{
                        node_index += $function(
                            context,
                            node,
                            parameters,
                            &nodes[node_index..],
                            &mut $output,
                        );
                        continue;
                    }};
                }
                node_index += 1;
                if let Some(name) = ::parse_text(name) {
                    match &name as _ {
                        "Abkürzungen" => section!(abbreviations::list::parse_list),
                        "Abschnitte fehlen" | "Quellen" | "Referenzen prüfen"
                        | "Ähnlichkeiten 1" | "Ähnlichkeiten 2" => {
                            ::add_warning(context, node, ::WarningMessage::Supplementary);
                            continue;
                        }
                        "Aussprache" => section!(pronunciation::pronunciation::parse_pronunciation),
                        "Bedeutungen" => section!(definitions::list::parse_list),
                        "Beispiele" => section!(examples::examples::parse_examples),
                        "Charakteristische Wortkombinationen" => {
                            section!(typical_word_combinations::list::parse_list)
                        }
                        "Gegenwörter" => section!(antonyms::list::parse_list),
                        "Herkunft" => section!(etymology::list::parse_list),
                        "Koseformen" => section!(affectionate_forms::list::parse_list),
                        "Kurzformen" => section!(short_forms::list::parse_list),
                        "Männliche Wortformen" => section!(masculine_forms::list::parse_list),
                        "Nebenformen" => section!(variants::list::parse_list),
                        "Nicht mehr gültige Schreibweisen" => {
                            section!(no_longer_valid_spellings::list::parse_list)
                        }
                        "Oberbegriffe" => section!(hypernyms::list::parse_list),
                        "Redewendungen" => section!(idioms::list::parse_list),
                        "Referenzen" => {
                            match nodes.get(node_index) {
                                Some(node @ ::Node::DefinitionList { .. }) => {
                                    node_index += 1;
                                    ::add_warning(context, node, ::WarningMessage::Supplementary);
                                }
                                _ => ::add_warning(context, node, ::WarningMessage::SectionEmpty),
                            }
                            continue;
                        }
                        "Sinnverwandte Wörter" => section!(related_words::list::parse_list),
                        "Sprichwörter" => section!(proverbs::list::parse_list),
                        "Symbole" => section!(symbols::list::parse_list),
                        "Synonyme" => section!(synonyms::list::parse_list),
                        "Unterbegriffe" => section!(hyponyms::list::parse_list),
                        "Verkleinerungsformen" => section!(diminutives::list::parse_list),
                        "Weibliche Wortformen" => section!(feminine_forms::list::parse_list),
                        "Wortbildungen" => section!(compound_words::list::parse_list),
                        "Worttrennung" => section!(hyphenation::list::parse_list),
                        "Ähnlichkeiten" => section!(similar_words::list::parse_list),
                        _ => if ::overview::parse_overview(
                            context,
                            node,
                            name,
                            parameters,
                            &mut overview,
                        ) {
                            node_index += 1;
                            continue;
                        },
                    }
                }
            }
            _ => node_index += 1,
        }
        ::add_warning(context, node, ::WarningMessage::Unrecognized);
    }
    let pronunciation = pronunciation.unwrap_or_default();
    pos_entries.push(::PosEntry {
        abbreviations: abbreviations.unwrap_or_default(),
        affectionate_forms: affectionate_forms.unwrap_or_default(),
        antonyms: antonyms.unwrap_or_default(),
        audio: pronunciation.audio,
        compound_words: compound_words.unwrap_or_default(),
        definitions: definitions.unwrap_or_default(),
        details,
        diminutives: diminutives.unwrap_or_default(),
        etymology: etymology.unwrap_or_default(),
        examples: examples.unwrap_or_default(),
        hypernyms: hypernyms.unwrap_or_default(),
        hyphenation: hyphenation.unwrap_or_default(),
        hyponyms: hyponyms.unwrap_or_default(),
        idioms: idioms.unwrap_or_default(),
        ipa: pronunciation.ipa,
        feminine_forms: feminine_forms.unwrap_or_default(),
        masculine_forms: masculine_forms.unwrap_or_default(),
        no_longer_valid_spellings: no_longer_valid_spellings.unwrap_or_default(),
        overview: overview.unwrap_or_default(),
        pos,
        proverbs: proverbs.unwrap_or_default(),
        related_words: related_words.unwrap_or_default(),
        rhymes: pronunciation.rhymes,
        short_forms: short_forms.unwrap_or_default(),
        similar_words: similar_words.unwrap_or_default(),
        symbols: symbols.unwrap_or_default(),
        synonyms: synonyms.unwrap_or_default(),
        typical_word_combinations: typical_word_combinations.unwrap_or_default(),
        variants: variants.unwrap_or_default(),
    });
    node_index
}
