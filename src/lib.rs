// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

//! Parse dictionary pages from the German language edition of Wiktionary into structured data.
//!
//! For general information about Parse Wiktionary, see the readme file.
//!
//! # Examples
//!
//! This example prints all usage examples found in an article, together with the language and the first part of speech of the entry.
//!
//! ```
//! # extern crate parse_wiki_text;
//! # extern crate parse_wiktionary_de;
//! #
//! let title = "Ausstellungswohnen";
//! let wiki_text = concat!(
//!     "==Ausstellungswohnen ({{Sprache|Deutsch}})==\n",
//!     "==={{Wortart|Substantiv|Deutsch}}===\n",
//!     "{{Beispiele}}\n",
//!     r#":Das Haus sei geeignet bloß zum „Ausstellungswohnen", vermuteten Kritiker, "#,
//!     "die es meist nur in schwarz-weißen Zeitschriftenfotos betreten hatten."
//! );
//! let configuration = parse_wiktionary_de::create_configuration();
//! let parsed_wiki_text = configuration.parse(wiki_text);
//! let parsed_article = parse_wiktionary_de::parse(title, wiki_text, &parsed_wiki_text.nodes);
//! # let mut found = false;
//! for language_entry in parsed_article.language_entries {
//!     for pos_entry in language_entry.pos_entries {
//!         for example in pos_entry.examples {
//!             println!(
//!                 "The word '{title}' of language {language:?} and part of speech {pos:?} has the example: {example}",
//!                 title = title,
//!                 language = language_entry.language,
//!                 pos = pos_entry.pos,
//!                 example = &example.example.iter().map(|node| match node {
//!                     parse_wiktionary_de::Flowing::Text { value } => value,
//!                     _ => ""
//!                 }).collect::<String>()
//!             );
//! #           found = true;
//!         }
//!     }
//! }
//! # assert!(found);
//! ```
//!
//! # Limitations
//!
//! Parameters of overview templates are transferred to the output with minimal validation and processing. Due to the wide variety of overview templates that take parameters in highly complicated and inconsistent formats, fully validating and parsing these parameters is not feasible.
//!
//! The translations in the template [`Vorlage:Ü-Tabelle`](Vorlage:Ü-Tabelle) in the section [`Übersetzungen`](https://de.wiktionary.org/wiki/Vorlage:%C3%9Cbersetzungen) are not parsed. Due to the highly complicated format of translations, it's better not to even try parsing them than try and get an inconsistent result. Due to the common presence of translation tables that contain empty translations, it's not even indicated whether an entry has translations.
//!
//! The templates [`Ähnlichkeiten 1`](https://de.wiktionary.org/wiki/Vorlage:%C3%84hnlichkeiten_1) and [`Ähnlichkeiten 2`](https://de.wiktionary.org/wiki/Vorlage:%C3%84hnlichkeiten_2) are not parsed, because it's unclear what purpose they have and what format their parameters must have.

// XXX Consider going through all templates in https://de.wiktionary.org/wiki/Kategorie:Wiktionary:Markierung.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate parse_wiki_text;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod configuration;
mod examples;
mod language;
mod languages;
mod list;
mod overview;
mod pos_section;
mod pos_template;
mod pronunciation;
mod util;

pub use configuration::create_configuration;
pub use languages::Language;
use parse_wiki_text::{DefinitionListItem, DefinitionListItemType::Details, Node, Parameter};
use std::{borrow::Cow, collections::HashMap};
use util::*;

/// Usage example.
#[derive(Debug, Deserialize, Serialize)]
pub struct Example<'a> {
    /// The example in the language of the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub example: Vec<Flowing<'a>>,

    /// The German translation of the example.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub translation: Vec<Flowing<'a>>,
}

/// An element in a sequence that allows different kinds of elements.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Flowing<'a> {
    /// Audio sample.
    ///
    /// Parsed from the template [`Audio`](https://de.wiktionary.org/wiki/Vorlage:Audio).
    Audio {
        /// The file name referred to.
        file_name: Cow<'a, str>,

        /// The label to display for the audio sample.
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<Cow<'a, str>>,

        /// The language of the audio.
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<Cow<'a, str>>,
    },

    /// Toggle bold text.
    ///
    /// Parsed from the wiki text `'''`.
    Bold,

    /// Comment tag.
    ///
    /// Parsed code starting with `<!--`.
    Comment,

    /// Indication that something is of common gender.
    ///
    /// Parsed from the template [`u`](https://de.wiktionary.org/wiki/Vorlage:u).
    CommonGender,

    /// Indication that something is a comparative.
    ///
    /// Parsed from the template [`Komp.`](https://de.wiktionary.org/wiki/Vorlage:Komp.).
    Comparative,

    /// Placeholder for an audio sample that has not been filled in.
    ///
    /// Parsed from the template [`Audio`](https://de.wiktionary.org/wiki/Vorlage:Audio).
    EmptyAudio,

    /// Indication that something is of feminine gender.
    ///
    /// Parsed from the template [`u`](https://de.wiktionary.org/wiki/Vorlage:f).
    FeminineGender,

    /// Indication that something is a genitive.
    ///
    /// Parsed from the template [`Gen.`](https://de.wiktionary.org/wiki/Vorlage:Gen.).
    Genitive,

    /// Pronunciation written in IPA.
    ///
    /// Parsed from the template [`Lautschrift`](https://de.wiktionary.org/wiki/Vorlage:Lautschrift).
    Ipa {
        /// The pronunciation written in IPA.
        ipa: Cow<'a, str>,
    },

    /// Toggle italic text.
    ///
    /// Parsed from the wiki text `''`.
    Italic,

    /// Language as a noun.
    ///
    /// Parsed from the templates found in the category [`Sprachkürzel`](https://de.wiktionary.org/wiki/Kategorie:Wiktionary:Sprachk%C3%BCrzel).
    Language {
        /// The language referred to.
        language: Cow<'a, str>,
    },

    /// Language as an adjective.
    ///
    /// Parsed from the templates found in the category [`Sprachadjektive`](https://de.wiktionary.org/wiki/Kategorie:Wiktionary:Sprachadjektive).
    LanguageAdjective {
        /// The language referred to.
        language: Cow<'a, str>,
    },

    /// Link.
    ///
    /// Parsed from wiki text starting with `[[`.
    Link {
        /// The target the link refers to.
        target: Cow<'a, str>,

        /// The text to display for the link.
        text: Cow<'a, str>,
    },

    /// Unordered list.
    List {
        /// The items of the list.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        items: Vec<Vec<Flowing<'a>>>,
    },

    /// Indication that something is of masculine gender.
    ///
    /// Parsed from the template [`m`](https://de.wiktionary.org/wiki/Vorlage:m).
    MasculineGender,

    /// Indication that something is of neuter gender.
    ///
    /// Parsed from the template [`n`](https://de.wiktionary.org/wiki/Vorlage:n).
    NeuterGender,

    /// Indication that something has no plural.
    ///
    /// Parsed from the template [`kPl.`](https://de.wiktionary.org/wiki/Vorlage:kPl.).
    NoPlural,

    /// Indication that something is a past participle.
    ///
    /// Parsed from the template [`Part.`](https://de.wiktionary.org/wiki/Vorlage:Part.).
    PastParticiple,

    /// Indication that something is a plural.
    ///
    /// Parsed from the template [`Pl.`](https://de.wiktionary.org/wiki/Vorlage:Pl.).
    Plural,

    /// Indication that something is a plural.
    ///
    /// Parsed from the template [`Pl.1`](https://de.wiktionary.org/wiki/Vorlage:Pl.1).
    Plural1,

    /// Indication that something is a plural.
    ///
    /// Parsed from the template [`Pl.2`](https://de.wiktionary.org/wiki/Vorlage:Pl.2).
    Plural2,

    /// Indication that something is a plural.
    ///
    /// Parsed from the template [`Pl.3`](https://de.wiktionary.org/wiki/Vorlage:Pl.3).
    Plural3,

    /// Indication that something is a plural.
    ///
    /// Parsed from the template [`Pl.4`](https://de.wiktionary.org/wiki/Vorlage:Pl.4).
    Plural4,

    /// Part of speech.
    ///
    /// Parsed from the template [`Wortbildung`](https://de.wiktionary.org/wiki/Vorlage:Wortbildung).
    Pos {
        /// The part of speech.
        pos: Pos,
    },

    /// Indication that something is a preterite.
    ///
    /// Parsed from the template [`Prät.`](https://de.wiktionary.org/wiki/Vorlage:Pr%C3%A4t.).
    Preterite,

    /// Quality control marker.
    ///
    /// Parsed from the template [`QS Herkunft`](https://de.wiktionary.org/wiki/Vorlage:QS_Herkunft).
    QualityControl,

    /// Indication of a reference.
    ///
    /// Parsed from the extension tag `ref`. The content if the reference is not parsed. This element is added to the output just to indicate the existence of a reference.
    Reference,

    /// Rhyme.
    ///
    /// Parsed from the template [Reim](https://de.wiktionary.org/wiki/Vorlage:Reim).
    Rhyme {
        /// The rhyme.
        rhyme: Cow<'a, str>,
    },

    /// Indication that something is a superlative.
    ///
    /// Parsed from the template [`Sup.`](https://de.wiktionary.org/wiki/Vorlage:Sup.).
    Superlative,

    /// End of superscript.
    ///
    /// Parsed from the tag `</sup>`.
    SuperscriptEnd,

    /// Start of superscript.
    ///
    /// Parsed from the tag `<sup>`.
    SuperscriptStart,

    /// Link to a dictionary entry.
    ///
    /// Parsed from the template [`Ü`](https://de.wiktionary.org/wiki/Vorlage:%C3%9C).
    Term {
        /// The language the link refers to.
        language: Cow<'a, str>,

        /// The term the link refers to.
        term: Cow<'a, str>,

        /// Transliteration of the term.
        #[serde(skip_serializing_if = "Option::is_none")]
        transliteration: Option<Cow<'a, str>>,
    },

    /// Chunk of plain text.
    Text {
        /// The text to display.
        value: Cow<'a, str>,
    },

    /// Element that could not be recognized.
    Unknown {
        /// The wiki text of the element.
        value: Cow<'a, str>,
    },
}

/// Dictionary entry for a single language.
#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageEntry<'a> {
    /// The language of the entry.
    pub language: Language,

    /// Entries for parts of speech for this language.
    ///
    /// Parsed from the sections with the template [`Wortart`](https://de.wiktionary.org/wiki/Vorlage:Wortart) in their heading.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pos_entries: Vec<PosEntry<'a>>,
}

/// Output of parsing a page.
#[derive(Debug, Deserialize, Serialize)]
pub struct Output<'a> {
    /// The dictionary entries by language.
    ///
    /// Parsed from the section with the template [`Sprache`](https://de.wiktionary.org/wiki/Vorlage:Sprache) in its heading.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_entries: Vec<LanguageEntry<'a>>,

    /// Warnings from the parser telling that something is not well-formed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Warning>,
}

/// Information from the overview template in the POS entry.
///
/// There are many different overview templates for different languages and different patterns of inflection. These are constructed in a way that makes it difficult to parse their meaning. Therefore any parameters are accepted and included in the output.
#[derive(Debug, Deserialize, Serialize)]
pub struct Overview<'a> {
    /// The name of the overview template.
    pub name: Cow<'a, str>,

    /// The named parameters to the template by name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub named_parameters: HashMap<Cow<'a, str>, Cow<'a, str>>,

    /// The unnamed parameters to the template in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unnamed_parameters: Vec<Vec<::Flowing<'a>>>,
}

/// Part of speech.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Pos {
    /// Abbreviation (“Abkürzung”)
    Abbreviation,

    /// Adjective (“Adjektiv”)
    Adjective,

    /// Adverb (“Adverb”)
    Adverb,

    /// Compound word (“Wortverbindung”)
    CompoundWord,

    /// Conjugated form (“Konjugierte Form”)
    ConjugatedForm,

    /// Conjunction (“Konjunktion”)
    Conjunction,

    /// Declined form (“Deklinierte Form”)
    DeclinedForm,

    /// First name (“Vorname”)
    FirstName,

    /// Idiom (“Redewendung”)
    Idiom,

    /// Interjection (“Interjektion”)
    Interjection,

    /// Last name (“Nachname”)
    LastName,

    /// Local adverb (“Lokaladverb”)
    LocalAdverb,

    /// Noun (“Substantiv”)
    Noun,

    /// Numeral (“Numerale”)
    Numeral,

    /// Past participle (“Partizip II”)
    PastParticiple,

    /// Postposition (“Postposition”)
    Postposition,

    /// Preposition (“Präposition”)
    Preposition,

    /// Proper noun (“Eigenname”)
    ProperNoun,

    /// Proverb (“Sprichwort”)
    Proverb,

    /// Symbol (“Symbol”)
    Symbol,

    /// Toponym (“Toponym”)
    Toponym,

    /// Verb (“Verb”)
    Verb,
}

/// The entry for a part of speech within the entry for a language.
#[derive(Debug, Deserialize, Serialize)]
pub struct PosEntry<'a> {
    /// Abbreviations, from the section [`Abkürzungen`](https://de.wiktionary.org/wiki/Vorlage:Abk%C3%BCrzungen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abbreviations: Vec<Vec<Flowing<'a>>>,

    /// Affectionate forms, from the section [`Koseformen`](https://de.wiktionary.org/wiki/Vorlage:Koseformen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affectionate_forms: Vec<Vec<Flowing<'a>>>,

    /// Antonyms, from the section [`Gegenwörter`](https://de.wiktionary.org/wiki/Vorlage:Gegenw%C3%B6rter).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub antonyms: Vec<Vec<Flowing<'a>>>,

    /// Audio, from the subsection [`Hörbeispiele`](https://de.wiktionary.org/wiki/Vorlage:H%C3%B6rbeispiele) in the section [`Aussprache`](https://de.wiktionary.org/wiki/Vorlage:Aussprache).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<Flowing<'a>>,

    /// Compound words, from the section [`Wortbildungen`](https://de.wiktionary.org/wiki/Vorlage:Wortbildungen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compound_words: Vec<Vec<Flowing<'a>>>,

    /// Definition, from the section [`Bedeutungen`](https://de.wiktionary.org/wiki/Vorlage:Bedeutungen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<Vec<Flowing<'a>>>,

    /// Various details from the POS heading.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Flowing<'a>>,

    /// Diminutives, from the section [`Verkleinerungsformen`](https://de.wiktionary.org/wiki/Vorlage:Verkleinerungsformen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diminutives: Vec<Vec<Flowing<'a>>>,

    /// Etymology, from the section [`Herkunft`](https://de.wiktionary.org/wiki/Vorlage:Herkunft).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub etymology: Vec<Vec<Flowing<'a>>>,

    /// Examples, from the section [`Beispiele`](https://de.wiktionary.org/wiki/Vorlage:Beispiele).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Example<'a>>,

    /// Feminine forms, from the section [`Weibliche Wortformen`](https://de.wiktionary.org/wiki/Vorlage:Weibliche_Wortformen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feminine_forms: Vec<Vec<Flowing<'a>>>,

    /// Hypernyms, from the section [`Oberbegriffe`](https://de.wiktionary.org/wiki/Vorlage:Oberbegriffe).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hypernyms: Vec<Vec<Flowing<'a>>>,

    /// Hyphenation, from the section [`Worttrennung`](https://de.wiktionary.org/wiki/Vorlage:Worttrennung).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hyphenation: Vec<Vec<Flowing<'a>>>,

    /// Hyponyms, from the section [`Unterbegriffe`](https://de.wiktionary.org/wiki/Vorlage:Unterbegriffe).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hyponyms: Vec<Vec<Flowing<'a>>>,

    /// Idioms, from the section [`Redewendungen`](https://de.wiktionary.org/wiki/Vorlage:Redewendungen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub idioms: Vec<Vec<Flowing<'a>>>,

    /// IPA, from the subsection [`IPA`](https://de.wiktionary.org/wiki/Vorlage:IPA) in the section [`Aussprache`](https://de.wiktionary.org/wiki/Vorlage:Aussprache).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipa: Vec<Flowing<'a>>,

    /// Masculine forms, from the section [`Männliche Wortformen`](https://de.wiktionary.org/wiki/Vorlage:M%C3%A4nnliche_Wortformen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub masculine_forms: Vec<Vec<Flowing<'a>>>,

    /// No longer valid spellings, from the section [`Nicht mehr gültige Schreibweisen`](https://de.wiktionary.org/wiki/Vorlage:Nicht_mehr_g%C3%BCltige_Schreibweisen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub no_longer_valid_spellings: Vec<Vec<Flowing<'a>>>,

    /// Various information about the entry, from any of the many overview templates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<Overview<'a>>,

    /// The first part of speech of the entry, from the heading of the POS entry.
    pub pos: Pos,

    /// Proverbs, fro mthe section [`Sprichwörter`](https://de.wiktionary.org/wiki/Vorlage:Sprichw%C3%B6rter).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub proverbs: Vec<Vec<Flowing<'a>>>,

    /// Related words, from the section [`Sinnverwandte Wörter`](https://de.wiktionary.org/wiki/Vorlage:Sinnverwandte_W%C3%B6rter).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_words: Vec<Vec<Flowing<'a>>>,

    /// Rhymes, from the subsection [`Reime`](https://de.wiktionary.org/wiki/Vorlage:Reime) in the section [`Aussprache`](https://de.wiktionary.org/wiki/Vorlage:Aussprache).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rhymes: Vec<Flowing<'a>>,

    /// Short forms, from the section [`Kurzformen`](https://de.wiktionary.org/wiki/Vorlage:Kurzformen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub short_forms: Vec<Vec<Flowing<'a>>>,

    /// Similar words, from the section [`Ähnlichkeiten`](https://de.wiktionary.org/wiki/Vorlage:%C3%84hnlichkeiten).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub similar_words: Vec<Vec<Flowing<'a>>>,

    /// Symbols, from the section [`Symbole`](https://de.wiktionary.org/wiki/Vorlage:Symbole).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<Vec<Flowing<'a>>>,

    /// Synonyms, from the section [`Synonyme`](https://de.wiktionary.org/wiki/Vorlage:Synonyme).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<Vec<Flowing<'a>>>,

    /// Typical word combinations, from the section [`Charakteristische Wortkombinationen`](https://de.wiktionary.org/wiki/Vorlage:Charakteristische_Wortkombinationen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub typical_word_combinations: Vec<Vec<Flowing<'a>>>,

    /// Variants, from the section [`Nebenformen`](https://de.wiktionary.org/wiki/Vorlage:Nebenformen).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<Vec<Flowing<'a>>>,
}

/// Warning from the parser telling that something is not well-formed.
///
/// When a warning occurs, it's not guaranteed that the text near the warning is parsed correctly. Usually the data that could not be unambiguously parsed due to the warning is excluded from the output, to make sure the output doesn't contain incorrectly parsed data.
#[derive(Debug, Deserialize, Serialize)]
pub struct Warning {
    /// The byte position in the wiki text where the warning ends.
    pub end: usize,

    /// The language of the language section in which the warning occurred, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// An identifier for the kind of warning.
    pub message: WarningMessage,

    /// The byte position in the wiki text where the warning starts.
    pub start: usize,
}

/// Identifier for a kind of warning from the parser.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WarningMessage {
    /// The element is a duplicate of something that comes before it.
    ///
    /// This may be a heading that contains the same text as a previous heading in the same section, or a parameter that has the same name as a previous parameter to the same template.
    Duplicate,

    /// The element is missing some required content.
    Empty,

    /// The section following the heading is missing some required content.
    SectionEmpty,

    /// The element is recognized but not represented in the output.
    ///
    /// The element conveys meaningful information, but this information has not been parsed and is not represented in the output. In contrast to other warnings, this warning does not indicate there is anything wrong with the wiki text. It just indicates that the wiki text contains additional information that is not represented in the output. The element is recognized as valid in the position it occurs, but its content is not parsed, and nothing can be said about whether the content is valid.
    ///
    /// This applies for example to the section [`Referenzen`](https://de.wiktionary.org/wiki/Vorlage:Referenzen), the templates [`Ü-Tabelle`](https://de.wiktionary.org/wiki/Vorlage:%C3%9C-Tabelle) and [`erweitern`](https://de.wiktionary.org/wiki/Vorlage:erweitern) and the extension tag `ref`.
    Supplementary,

    /// The element is not recognized.
    ///
    /// This may be because of the type of the element itself or because of anything inside it.
    Unrecognized,

    /// The value of the element conflicts with information occurring before it.
    ///
    /// This can mean for example that a specified language within a section doesn't match the language specified for the section as a whole.
    ValueConflicting,

    /// The element is recognized, but its value is not.
    ///
    /// On lists it means that a list with this kind is valid in this position, but something about the list items contained in the list is not recognized.
    ///
    /// On templates it means that a template with this name is valid in this position, but something about the parameters of the template is not recognized.
    ///
    /// On template parameters it means that a parameter with this name (or lack of name) is valid in this position, but something about the value of the parameter is not recognized.
    ValueUnrecognized,
}

/// Parses an article from the German language version of Wiktionary into structured data.
///
/// `title` is the title of the article. `wiki_text` is the wiki text of the article. `nodes` is the sequence of nodes obtained by parsing the wiki text with the crate [Parse Wiki Text](https://github.com/portstrom/parse_wiki_text).
#[must_use]
pub fn parse<'a>(title: &str, wiki_text: &'a str, nodes: &[Node<'a>]) -> Output<'a> {
    let mut context = Context {
        language: None,
        warnings: vec![],
        wiki_text,
    };
    let mut language_entries = vec![];
    let mut node_index = 0;
    while let Some(node) = nodes.get(node_index) {
        if let Node::Heading {
            level,
            nodes: heading_child_nodes,
            ..
        } = node
        {
            if *level < 2 {
                add_warning(&mut context, node, WarningMessage::Unrecognized);
                break;
            }
            if *level == 2 {
                node_index += 1;
                if let [Node::Text { value, .. }, Node::Template {
                    name, parameters, ..
                }, Node::Text { value: ")", .. }] = heading_child_nodes.as_slice()
                {
                    if value.len() == title.len() + 2
                        && value.starts_with(title)
                        && value.ends_with(" (")
                        && text_equals(name, "Sprache")
                    {
                        if let [parameter @ Parameter { name: None, .. }] = parameters.as_slice() {
                            if let Some(language) = parse_text(&parameter.value) {
                                match Language::from_name(&language) {
                                    None => add_warning(
                                        &mut context,
                                        parameter,
                                        WarningMessage::ValueUnrecognized,
                                    ),
                                    Some(language) => {
                                        context.language = Some(language);
                                        node_index += language::parse_language(
                                            &mut context,
                                            node,
                                            &nodes[node_index..],
                                            &mut language_entries,
                                        );
                                        context.language = None;
                                    }
                                }
                                continue;
                            }
                        }
                    }
                }
                add_warning(&mut context, node, WarningMessage::ValueUnrecognized);
                continue;
            }
        }
        node_index += 1;
        add_warning(&mut context, node, WarningMessage::Unrecognized);
    }
    Output {
        language_entries,
        warnings: context.warnings,
    }
}
