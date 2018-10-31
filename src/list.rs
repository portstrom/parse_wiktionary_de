// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_list<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter],
    nodes: &[::Node<'a>],
    output: &mut Option<Vec<Vec<::Flowing<'a>>>>,
) -> usize {
    ::parse_list_items_generic(
        context,
        template_node,
        parameters,
        nodes,
        output,
        |context, list_item| {
            if list_item.nodes.is_empty() {
                ::add_warning(context, list_item, ::WarningMessage::Empty);
                None
            } else {
                Some(parse_list_item(context, &list_item.nodes, true))
            }
        },
    )
}

macro_rules! parse_list_item {
    {
        language_adjectives { $( $language_adjective:tt ),+ }
        languages { $( $language:tt ),+ }
        simple $( ( $simple_name:tt, $simple_variant:tt ) )+
    } => {
        fn parse_list_item<'a>(
            context: &mut ::Context<'a>,
            nodes: &[::Node<'a>],
            allow_list: bool
        ) -> Vec<::Flowing<'a>> {
            nodes.iter().filter_map(|node| match node {
                ::Node::Bold { .. } => Some(::Flowing::Bold),
                ::Node::CharacterEntity { character: '\u{a0}', .. } => Some(::Flowing::Text {
                    value: ::Cow::Borrowed("\u{a0}")
                }),
                ::Node::Comment { .. } => {
                    ::add_warning(context, node, ::WarningMessage::Supplementary);
                    Some(::Flowing::Comment)
                }
                ::Node::EndTag { name, .. } if name == "sup" => Some(::Flowing::SuperscriptEnd),
                ::Node::Italic { .. } => Some(::Flowing::Italic),
                ::Node::Link { target, text, .. } => Some(::parse_link(context, node, target, text)),
                ::Node::StartTag { name, .. } if name == "sup" => Some(::Flowing::SuperscriptStart),
                ::Node::Tag { name, .. } if name == "ref" => {
                    ::add_warning(context, node, ::WarningMessage::Supplementary);
                    Some(::Flowing::Reference)
                }
                ::Node::Template { name, parameters, .. } => Some(match ::parse_text(name) {
                    None => ::create_unknown(context, node, ::WarningMessage::Unrecognized),
                    Some(name) => match &name as _ {
                        "QS Herkunft" | "QS_Herkunft" => {
                            ::add_warning(context, node, ::WarningMessage::Supplementary);
                            ::Flowing::QualityControl
                        }
                        "Wortbildung" => parse_pos(context, node, parameters),
                        "Ü" => parse_term(context, node, parameters),
                        "Üt" => parse_term_transliteration(context, node, parameters),
                        $( concat!($language_adjective, ".") => ::parse_simple_template(context, node, parameters, ::Flowing::LanguageAdjective {
                            language: ::Cow::Borrowed($language_adjective)
                        }), )+
                        $( $language => ::parse_simple_template(context, node, parameters, ::Flowing::Language {
                            language: ::Cow::Borrowed($language)
                        }), )+
                        $( $simple_name => ::parse_simple_template(context, node, parameters, ::Flowing::$simple_variant), )+
                        _ => ::create_unknown(context, node, ::WarningMessage::Unrecognized)
                    }
                }),
                ::Node::Text { value, .. } => Some(::Flowing::Text { value: ::Cow::Borrowed(value) }),
                ::Node::UnorderedList { items, .. } if allow_list => {
                    let items: Vec<_> = items.iter().filter_map(|item| {
                        if item.nodes.is_empty() {
                            ::add_warning(context, item, ::WarningMessage::Empty);
                            None
                        } else {
                            Some(parse_list_item(context, &item.nodes, false))
                        }
                    }).collect();
                    if items.is_empty() {
                        None
                    } else {
                        Some(::Flowing::List { items })
                    }
                },
                _ => Some(::create_unknown(context, node, ::WarningMessage::Unrecognized))
            }).collect()
        }
    }
}

parse_list_item! {
    language_adjectives {
        "ahd",
        "amer",
        "ang",
        "ar",
        "brit",
        "cy",
        "da",
        "de",
        "dum",
        "el",
        "en",
        "es",
        "fa",
        "fr",
        "fro",
        "frühnhd",
        "ga",
        "gem",
        "gmh",
        "gml",
        "goh",
        "got",
        "grc",
        "hy",
        "ine",
        "is",
        "it",
        "lat",
        "lt",
        "md",
        "mhd",
        "mlat",
        "mlg",
        "nds",
        "nhd",
        "nl",
        "no",
        "non",
        "nordd",
        "ofs",
        "osx",
        "owen",
        "pt",
        "ru",
        "schweiz",
        "spätlat",
        "sv",
        "süddt",
        "th",
        "tr",
        "wen",
        "österr"
    }
    languages {
        "KA",
        "MHA",
        "aa",
        "ab",
        "abq",
        "ace",
        "ady",
        "ae",
        "aeb",
        "af",
        "agf",
        "agj",
        "aie",
        "ain",
        "ajp",
        "ak",
        "akg",
        "akk",
        "akz",
        "ale",
        "alp",
        "als",
        "alt",
        "am",
        "amk",
        "amu",
        "an",
        "ang",
        "apc",
        "apw",
        "ar",
        "arc",
        "arn",
        "arq",
        "arw",
        "ary",
        "arz",
        "as",
        "ast",
        "aua",
        "av",
        "ay",
        "ayl",
        "az",
        "azb",
        "azj",
        "ba",
        "baa",
        "bal",
        "ban",
        "bar",
        "bat",
        "bbc",
        "bci",
        "bcl",
        "bcm",
        "be",
        "bem",
        "ber",
        "bg",
        "bgc",
        "bh",
        "bhw",
        "bi",
        "bjn",
        "bla",
        "bm",
        "bmg",
        "bn",
        "bnd",
        "bo",
        "bpy",
        "br",
        "bs",
        "bty",
        "bua",
        "bug",
        "bxr",
        "bzg",
        "ca",
        "ccc",
        "cdo",
        "ce",
        "ceb",
        "cel",
        "ch",
        "chc",
        "chm",
        "cho",
        "chp",
        "chr",
        "chy",
        "cjs",
        "ckb",
        "ckt",
        "cnx",
        "co",
        "com",
        "cop",
        "cr",
        "crh",
        "cro",
        "crs",
        "cs",
        "csb",
        "cu",
        "cv",
        "cy",
        "da",
        "ddn",
        "de",
        "dhv",
        "diq",
        "dje",
        "dlm",
        "dng",
        "dob",
        "dsb",
        "dum",
        "dv",
        "dz",
        "ee",
        "egy",
        "el",
        "eml",
        "en",
        "enm",
        "eo",
        "erk",
        "es",
        "et",
        "eu",
        "ext",
        "fa",
        "ff",
        "fi",
        "fj",
        "fng",
        "fo",
        "fon",
        "fr",
        "frk",
        "frm",
        "fro",
        "frp",
        "frr",
        "frs",
        "fry",
        "fur",
        "fy",
        "ga",
        "gag",
        "gan",
        "gay",
        "gcf",
        "gd",
        "gdq",
        "gem",
        "gez",
        "gha",
        "gil",
        "gl",
        "glk",
        "gmh",
        "gml",
        "gmw",
        "gmy",
        "gn",
        "gnc",
        "goh",
        "got",
        "gr",
        "grc",
        "gsw",
        "gu",
        "gv",
        "ha",
        "hac",
        "hak",
        "haw",
        "he",
        "hi",
        "hif",
        "hit",
        "ho",
        "hop",
        "hr",
        "hsb",
        "ht",
        "hu",
        "hy",
        "hz",
        "ia",
        "iba",
        "id",
        "ie",
        "ig",
        "ii",
        "ik",
        "ilo",
        "ine",
        "inh",
        "io",
        "is",
        "ist",
        "it",
        "itk",
        "itl",
        "iu",
        "izh",
        "ja",
        "jbo",
        "jrb",
        "jv",
        "ka",
        "kaa",
        "kab",
        "kam",
        "kaw",
        "kbd",
        "kca",
        "kdr",
        "kg",
        "khb",
        "ki",
        "kj",
        "kjh",
        "kk",
        "kl",
        "kla",
        "km",
        "kmr",
        "kn",
        "ko",
        "koi",
        "kok",
        "kos",
        "kr",
        "krc",
        "krl",
        "ks",
        "ksh",
        "ku",
        "kum",
        "kv",
        "kw",
        "ky",
        "kyh",
        "la",
        "lad",
        "lb",
        "lbe",
        "ldn",
        "lep",
        "lez",
        "lg",
        "li",
        "lij",
        "liv",
        "lld",
        "llp",
        "lmo",
        "ln",
        "lo",
        "lou",
        "loz",
        "lt",
        "ltg",
        "lud",
        "lus",
        "lv",
        "lzz",
        "mad",
        "mak",
        "mas",
        "mdf",
        "mfe",
        "mg",
        "mga",
        "mh",
        "mhr",
        "mi",
        "mic",
        "min",
        "mk",
        "ml",
        "mn",
        "mnc",
        "mnk",
        "mns",
        "mo",
        "moh",
        "mr",
        "mrj",
        "ms",
        "mt",
        "mus",
        "mwl",
        "my",
        "myn",
        "myv",
        "mzn",
        "na",
        "nah",
        "nan",
        "nap",
        "naq",
        "nb",
        "nde",
        "nds",
        "ne",
        "new",
        "nez",
        "ng",
        "ngo",
        "nhn",
        "nic",
        "niu",
        "nl",
        "nld",
        "nmn",
        "nn",
        "no",
        "nog",
        "non",
        "nov",
        "nr",
        "nrf",
        "nso",
        "nv",
        "ny",
        "nyn",
        "obt",
        "oc",
        "oco",
        "odt",
        "ofs",
        "oj",
        "om",
        "ood",
        "or",
        "orv",
        "os",
        "osc",
        "osx",
        "ota",
        "owl",
        "pa",
        "pag",
        "pal",
        "pam",
        "pap",
        "pcd",
        "pdc",
        "pdt",
        "peo",
        "pfl",
        "pgn",
        "phn",
        "pi",
        "pih",
        "pis",
        "pl",
        "pms",
        "pnb",
        "pnt",
        "pox",
        "pra",
        "prg",
        "pro",
        "prs",
        "ps",
        "pt",
        "qka",
        "qts",
        "qu",
        "raj",
        "rap",
        "rhg",
        "rif",
        "rm",
        "rmq",
        "rmr",
        "rmy",
        "rn",
        "ro",
        "rom",
        "ru",
        "rue",
        "rup",
        "rw",
        "sa",
        "sah",
        "sas",
        "sc",
        "scn",
        "sco",
        "sd",
        "se",
        "sg",
        "sga",
        "sgs",
        "sgw",
        "sh",
        "shh",
        "shi",
        "shv",
        "si",
        "simple",
        "sjn",
        "sk",
        "sl",
        "sla",
        "sli",
        "sm",
        "smi",
        "smn",
        "sn",
        "snk",
        "so",
        "spx",
        "sq",
        "sqr",
        "sqt",
        "sr",
        "src",
        "srn",
        "sro",
        "srr",
        "ss",
        "st",
        "stq",
        "su",
        "suw",
        "sux",
        "sv",
        "sva",
        "sw",
        "swb",
        "swg",
        "syr",
        "szl",
        "ta",
        "tay",
        "te",
        "tet",
        "tg",
        "th",
        "ti",
        "tig",
        "tk",
        "tkl",
        "tl",
        "tlh",
        "tmh",
        "tn",
        "tnq",
        "to",
        "tokipona",
        "tox",
        "tpi",
        "tpn",
        "tpw",
        "tr",
        "trv",
        "ts",
        "tt",
        "tum",
        "tvk",
        "tvl",
        "tw",
        "txb",
        "txh",
        "ty",
        "tyv",
        "tzl",
        "tzm",
        "udm",
        "ug",
        "uga",
        "uk",
        "umc",
        "ur",
        "uum",
        "uz",
        "ve",
        "vec",
        "vep",
        "vi",
        "vls",
        "vmf",
        "vo",
        "vot",
        "vro",
        "wa",
        "war",
        "wen",
        "wep",
        "wlm",
        "wo",
        "wuu",
        "wym",
        "xaa",
        "xal",
        "xcl",
        "xfa",
        "xh",
        "xhu",
        "xlc",
        "xld",
        "xlu",
        "xmf",
        "xmn",
        "xno",
        "xum",
        "xur",
        "xve",
        "yi",
        "yo",
        "yua",
        "yue",
        "za",
        "zbw",
        "zea",
        "zen",
        "zh",
        "zh-cn",
        "zh-tw",
        "zu",
        "zza"
    }
    simple
        ("Komp.", Comparative)
        ("Part.", PastParticiple)
        ("Pl.", Plural)
        ("Pl.1", Plural1)
        ("Pl.2", Plural2)
        ("Pl.3", Plural3)
        ("Pl.4", Plural4)
        ("Prät.", Preterite)
        ("Sup.", Superlative)
        ("kPl.", NoPlural)
}

fn parse_pos<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter],
) -> ::Flowing<'a> {
    if let [parameter @ ::Parameter { name: None, .. }] = parameters {
        if let Some(text) = ::parse_text(&parameter.value) {
            if let Some(pos) = match &text as _ {
                "Adj" | "Adje" | "Adjektiv" | "Adjektive" => Some(::Pos::Adjective),
                "Adv" | "Adve" | "Adverb" | "Adverbien" => Some(::Pos::Adverb),
                "Sub" | "Subs" | "Substantiv" | "Substantive" => Some(::Pos::Noun),
                "Ver" | "Verb" | "Verben" => Some(::Pos::Verb),
                _ => None,
            } {
                return ::Flowing::Pos { pos };
            }
        }
    }
    ::create_unknown(context, template_node, ::WarningMessage::ValueUnrecognized)
}

fn parse_term<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> ::Flowing<'a> {
    if let [language_parameter @ ::Parameter { name: None, .. }, term_parameter @ ::Parameter { name: None, .. }] =
        parameters
    {
        match ::parse_text_not_empty(&language_parameter.value) {
            None => ::create_unknown2(
                context,
                template_node,
                language_parameter,
                ::WarningMessage::ValueUnrecognized,
            ),
            Some(language) => match ::parse_text_not_empty(&term_parameter.value) {
                None => ::create_unknown2(
                    context,
                    template_node,
                    term_parameter,
                    ::WarningMessage::ValueUnrecognized,
                ),
                Some(term) => ::Flowing::Term {
                    language,
                    term,
                    transliteration: None,
                },
            },
        }
    } else {
        ::create_unknown(context, template_node, ::WarningMessage::ValueUnrecognized)
    }
}

fn parse_term_transliteration<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> ::Flowing<'a> {
    if let [language_parameter @ ::Parameter { name: None, .. }, term_parameter @ ::Parameter { name: None, .. }, transliteration_parameter @ ::Parameter { name: None, .. }] =
        parameters
    {
        match ::parse_text_not_empty(&language_parameter.value) {
            None => ::create_unknown2(
                context,
                template_node,
                language_parameter,
                ::WarningMessage::ValueUnrecognized,
            ),
            Some(language) => match ::parse_text_not_empty(&term_parameter.value) {
                None => ::create_unknown2(
                    context,
                    template_node,
                    term_parameter,
                    ::WarningMessage::ValueUnrecognized,
                ),
                Some(term) => match ::parse_text_not_empty(&transliteration_parameter.value) {
                    None => ::create_unknown2(
                        context,
                        template_node,
                        transliteration_parameter,
                        ::WarningMessage::ValueUnrecognized,
                    ),
                    transliteration @ Some(_) => ::Flowing::Term {
                        language,
                        term,
                        transliteration,
                    },
                },
            },
        }
    } else {
        ::create_unknown(context, template_node, ::WarningMessage::ValueUnrecognized)
    }
}
