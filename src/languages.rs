// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

/// Identifier for a language.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Aa,
    Ab,
    Ae,
    Af,
    Ak,
    Am,
    An,
    Ar,
    Arc,
    As,
    Av,
    Ay,
    Az,
    Ba,
    Be,
    Bg,
    Bh,
    Bi,
    Bm,
    Bn,
    Bo,
    Br,
    Bs,
    By,
    Ca,
    Ce,
    Ch,
    Co,
    Cr,
    Cs,
    Cu,
    Cv,
    Cy,
    Da,
    De,
    Dv,
    Dz,
    Ee,
    El,
    En,
    Eo,
    Es,
    Et,
    Eu,
    Fa,
    Ff,
    Fi,
    Fj,
    Fo,
    Fr,
    Fy,
    Ga,
    Gd,
    Gl,
    Gn,
    Gu,
    Gv,
    Ha,
    He,
    Hi,
    Ho,
    Hr,
    Ht,
    Hu,
    Hy,
    Hz,
    Ia,
    Id,
    Ie,
    Ii,
    Ik,
    Is,
    It,
    Iu,
    Ja,
    Jv,
    Ka,
    Kg,
    Ki,
    Kj,
    Kk,
    Kl,
    Km,
    Kn,
    Ko,
    Kr,
    Ks,
    Ku,
    Kv,
    Kw,
    Ky,
    La,
    Lb,
    Lg,
    Li,
    Ln,
    Lo,
    Lt,
    Lu,
    Lv,
    Mg,
    Mh,
    Mi,
    Mk,
    Ml,
    Mn,
    Mr,
    Ms,
    Mt,
    My,
    Na,
    Nb,
    Nd,
    Ne,
    Ng,
    Nl,
    Nn,
    No,
    Nr,
    Nv,
    Ny,
    Oc,
    Oj,
    Om,
    Or,
    Os,
    Pa,
    Pi,
    Pl,
    Ps,
    Pt,
    Qu,
    Rm,
    Rn,
    Ro,
    Ru,
    Rw,
    Sa,
    Sc,
    Sd,
    Se,
    Sg,
    Si,
    Sk,
    Sl,
    Sm,
    Sn,
    So,
    Sq,
    Sr,
    Ss,
    St,
    Su,
    Sv,
    Sw,
    Ta,
    Te,
    Tg,
    Th,
    Ti,
    Tk,
    Tl,
    Tn,
    To,
    Tr,
    Ts,
    Tt,
    Tw,
    Ty,
    Ug,
    Uk,
    Ur,
    Uz,
    Ve,
    Vi,
    Vo,
    Wa,
    Wo,
    Xh,
    Yi,
    Yo,
    Za,
    Zh,
    Zu,
}

impl Language {
    /// Returns the language corresponding to the given language name if any.
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "(Neu-)Awarisch" => Language::Av,
            "(Neu-)Griechisch" => Language::El,
            "(Schottisch-)Gälisch" => Language::Gd,
            "Abchasisch" => Language::Ab,
            "Afar" => Language::Aa,
            "Afrikaans" => Language::Af,
            "Akan" => Language::Ak,
            "Albanisch" => Language::Sq,
            "Altkirchenslawisch" => Language::Cu,
            "Amharisch" => Language::Am,
            "Anishinabe" => Language::Oj,
            "Arabisch" => Language::Ar,
            "Aragonesisch" => Language::An,
            "Aramäisch" => Language::Arc,
            "Armenisch" => Language::Hy,
            "Aserbaidschanisch" => Language::Az,
            "Assamesisch/Assami" => Language::As,
            "Avestisch" => Language::Ae,
            "Aymara" => Language::Ay,
            "Bambara" => Language::Bm,
            "Banyumasan" => Language::By,
            "Baschkirisch" => Language::Ba,
            "Baskisch" => Language::Eu,
            "Bengalisch" => Language::Bn,
            "Bihari" => Language::Bh,
            "Birmanisch" => Language::My,
            "Bislama" => Language::Bi,
            "Bokmål" => Language::Nb,
            "Bosnisch" => Language::Bs,
            "Bretonisch" => Language::Br,
            "Bulgarisch" => Language::Bg,
            "Chamorro" => Language::Ch,
            "Chichewa" => Language::Ny,
            "Chinesisch" => Language::Zh,
            "Cree" => Language::Cr,
            "Deutsch" => Language::De,
            "Dhivehi" => Language::Dv,
            "Dzongkha" => Language::Dz,
            "Dänisch" => Language::Da,
            "Englisch" => Language::En,
            "Esperanto" => Language::Eo,
            "Estnisch" => Language::Et,
            "Ewe" => Language::Ee,
            "Fidschi" => Language::Fj,
            "Finnisch" => Language::Fi,
            "Französisch" => Language::Fr,
            "Friesisch" => Language::Fy,
            "Fula" => Language::Ff,
            "Färöisch" => Language::Fo,
            "Galicisch" => Language::Gl,
            "Ganda" => Language::Lg,
            "Georgisch" => Language::Ka,
            "Guaraní" => Language::Gn,
            "Gujarati" => Language::Gu,
            "Haitianisch" => Language::Ht,
            "Hausa" => Language::Ha,
            "Hebräisch" => Language::He,
            "Herero" => Language::Hz,
            "Hindi" => Language::Hi,
            "Hiri Motu" => Language::Ho,
            "Indonesisch" => Language::Id,
            "Interlingua" => Language::Ia,
            "Interlingue" => Language::Ie,
            "Inuktitut" => Language::Iu,
            "Inupiaq" => Language::Ik,
            "Irisch" => Language::Ga,
            "Isländisch" => Language::Is,
            "Italienisch" => Language::It,
            "Japanisch" => Language::Ja,
            "Javanisch" => Language::Jv,
            "Jiddisch" => Language::Yi,
            "Kalaallisut; Grönländisch" => Language::Kl,
            "Kannada" => Language::Kn,
            "Kanuri" => Language::Kr,
            "Kasachisch" => Language::Kk,
            "Kaschmirisch" => Language::Ks,
            "Katalanisch" => Language::Ca,
            "Khmer" => Language::Km,
            "Kikuyu" => Language::Ki,
            "Kiluba (Luba-Katanga)" => Language::Lu,
            "Kinyarwanda" => Language::Rw,
            "Kirgisisch" => Language::Ky,
            "Kirundi" => Language::Rn,
            "Komi" => Language::Kv,
            "Kongo, Kikongo" => Language::Kg,
            "Koreanisch" => Language::Ko,
            "Kornisch" => Language::Kw,
            "Korsisch" => Language::Co,
            "Kroatisch" => Language::Hr,
            "Kuanyama" => Language::Kj,
            "Kurdisch" => Language::Ku,
            "Laotisch" => Language::Lo,
            "Lateinisch" => Language::La,
            "Lettisch" => Language::Lv,
            "Limburgisch" => Language::Li,
            "Lingala" => Language::Ln,
            "Litauisch" => Language::Lt,
            "Luxemburgisch" => Language::Lb,
            "Madagassisch" => Language::Mg,
            "Malaiisch" => Language::Ms,
            "Malayalam" => Language::Ml,
            "Maltesisch" => Language::Mt,
            "Manx" => Language::Gv,
            "Maori" => Language::Mi,
            "Marathi" => Language::Mr,
            "Marshallesisch" => Language::Mh,
            "Mazedonisch" => Language::Mk,
            "Mongolisch" => Language::Mn,
            "Nauruisch" => Language::Na,
            "Navajo" => Language::Nv,
            "Ndonga" => Language::Ng,
            "Nepalesisch" => Language::Ne,
            "Niederländisch" => Language::Nl,
            "Nord-Ndebele" => Language::Nd,
            "Norwegisch" => Language::No,
            "Nynorsk (Neunorwegisch)" => Language::Nn,
            "Okzitanisch" => Language::Oc,
            "Oriya" => Language::Or,
            "Oromo" => Language::Om,
            "Ossetisch" => Language::Os,
            "Pali" => Language::Pi,
            "Pandschabi" => Language::Pa,
            "Paschtu" => Language::Ps,
            "Persisch" => Language::Fa,
            "Polnisch" => Language::Pl,
            "Portugiesisch" => Language::Pt,
            "Quechua" => Language::Qu,
            "Rumänisch" => Language::Ro,
            "Russisch" => Language::Ru,
            "Rätoromanisch" => Language::Rm,
            "Samisch" => Language::Se,
            "Samoanisch" => Language::Sm,
            "Sango" => Language::Sg,
            "Sanskrit" => Language::Sa,
            "Sardisch" => Language::Sc,
            "Schwedisch" => Language::Sv,
            "Serbisch" => Language::Sr,
            "Sesotho" => Language::St,
            "Setswana" => Language::Tn,
            "Shona" => Language::Sn,
            "Sindhi" => Language::Sd,
            "Singhalesisch" => Language::Si,
            "Siswati" => Language::Ss,
            "Slowakisch" => Language::Sk,
            "Slowenisch" => Language::Sl,
            "Somali" => Language::So,
            "Spanisch" => Language::Es,
            "Sundanesisch" => Language::Su,
            "Swahili" => Language::Sw,
            "Süd-Ndebele" => Language::Nr,
            "Tadschikisch" => Language::Tg,
            "Tagalog" => Language::Tl,
            "Tahitianisch" => Language::Ty,
            "Tamilisch" => Language::Ta,
            "Tatarisch" => Language::Tt,
            "Telugu" => Language::Te,
            "Thailändisch" => Language::Th,
            "Tibetisch" => Language::Bo,
            "Tigrinya" => Language::Ti,
            "Tongaisch" => Language::To,
            "Tschechisch" => Language::Cs,
            "Tschetschenisch" => Language::Ce,
            "Tschuwaschisch" => Language::Cv,
            "Tsonga" => Language::Ts,
            "Turkmenisch" => Language::Tk,
            "Twi" => Language::Tw,
            "Türkisch" => Language::Tr,
            "Uigurisch" => Language::Ug,
            "Ukrainisch" => Language::Uk,
            "Ungarisch" => Language::Hu,
            "Urdu" => Language::Ur,
            "Usbekisch" => Language::Uz,
            "Venda" => Language::Ve,
            "Vietnamesisch" => Language::Vi,
            "Volapük" => Language::Vo,
            "Walisisch" => Language::Cy,
            "Wallonisch" => Language::Wa,
            "Weißrussisch" => Language::Be,
            "Wolof" => Language::Wo,
            "Yi" => Language::Ii,
            "Yoruba" => Language::Yo,
            "Zhuang" => Language::Za,
            "isiXhosa" => Language::Xh,
            "isiZulu" => Language::Zu,
            _ => return None,
        })
    }
}
