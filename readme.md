<!--
Copyright 2018 Fredrik Portström <https://portstrom.com>
This is free software distributed under the terms specified in
the file LICENSE at the top-level directory of this distribution.
-->

# Parse Wiktionary for de.wiktionary.org

Parse dictionary pages from the German language edition of Wiktionary into structured data.

![Parse Wiki Text](https://portstrom.com/parse_wiki_text.svg)

This following information applies to all language editions of Parse Wiktionary. For information specific to each language edition, see its documentation.

## Introduction

Wiktionary is a dictionary with millions of entries containing a wide variety of data about words and phrases in many languages. The dictionary data is distributed under a free license, allowing it to be reused in other applications. Unfortunately it's written in a format that prevents using it in other applications. The format is designed only to be transformed into the exact HTML format displayed in Wiktionary itself, not to be parsed into semantically meaningful data that can be used for other purposes or displayed in other formats.

The format does however contain enough structurally meaningful data to allow most of it with great difficulty to be parsed into structured data. Parse Wiktionary does the challenging task of parsing entries from Wiktionary into a structured format that can easily be used to query details about entries and use them for different purposes and present them in different formats. Because each language edition of Wiktionary unfortunately has a completely different format, there is a different edition of Parse Wiktionary for each edition of Wiktionary and they all have a different output format. Currently Parse Wiktionary exists for the English (en.wiktionary.org), German (de.wiktionary.org) and Czech (cs.wiktionary.org) editions of Wiktionary.

## Limitations

Different parts of the information in Wiktionary are written in different formats that vary in regularity and complexity.

- Some parts have a highly regular format. These parts are parsed into semantic data directly representing the facts stated in the entry, not the format in which they are stated.
- Some parts have a format of manageable complexity but are not entirely consistent. Such parts are not parsed into semantic data, but parsed into a free form document. These documents are a sequence of elements where any of the supported elements can occur and can come in any order. There is also a kind of element that represents wiki text that could not be parsed. The document may be displayed by displaying all elements in order, or may be further parsed into semantic data in those cases the document complies with a regular enough format.
- Some parts don't have a consistent format or have a too complex and dynamic format that changes over time as templates are added or edited. These parts are not parsed at all. Parse Wiktionary may however put flags in the output indicating the existence of such parts.

In all editions of Wiktionary, the headings follow a regular format. Headings are therefore parsed semantically. The content of sections however may or may not be in a regular format depending on the section. Many sections are therefore parsed as a free form document. That free form document is however stored in a field with a semantic meaning. This means that even though the content of the documents is not semantic, they are organized in a semantic way allowing applications to choose what sections to take and what to do with each section.

The long term goal is to eliminate all these limitations and parse all information in Wiktionary as structured semantic data. This will however require cooperation from Wiktionary editors. A standard format could be created for each section, and authors encouraged to follow the standard format. Parse Wiktionary could be integrated in Wiktionary and validate entries as they are being edited, showing warnings about anything that doesn't conform to the standard format. More data can also be transferred to Wikidata which is already designed from the beginning to store semantic data.
