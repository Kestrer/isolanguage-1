//! This crate implements the [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) standard in Rust.
//! It also has optional Serde support, by using the `serde` feature:
//!
//! ```toml
//! isolanguage-1 = { version = "0.2.1", features = ["serde"] }
//! ```
//!
//! The main type is the `LanguageCode` type, which is an enum for every single country in ISO
//! 639-1. It optionally implements Serialize and Deserialize too.

use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! languages_table {
    (
        $enum_name:ident, $enum_err_name:ident,
        $(($variant:ident, $code:literal, $code_t:literal, $code_b:literal, $name:literal, $family:literal),)+
    ) => {
        /// An enumeration of all ISO 639-1 language codes.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum $enum_name {
            $(
                #[doc=$name]
                #[cfg_attr(feature = "serde", serde(rename=$code))]
                $variant,
            )+
        }

        impl $enum_name {
            /// Returns the 2 letter code of the language.
            ///
            /// # Examples
            ///
            /// ```
            /// use isolanguage_1::LanguageCode;
            ///
            /// assert_eq!(LanguageCode::Vi.code(), "vi");
            /// ```
            #[must_use]
            pub const fn code(self) -> &'static str {
                match self {
                    $(Self::$variant => $code,)+
                }
            }

            /// Returns the 3 letter ISO 639-2 T code of the language (preferred over the B code).
            ///
            /// # Examples
            ///
            /// ```
            /// use isolanguage_1::LanguageCode;
            ///
            /// assert_eq!(LanguageCode::Nl.code_t(), "nld");
            /// ```
            #[must_use]
            pub const fn code_t(self) -> &'static str {
                match self {
                    $(Self::$variant => $code_t,)+
                }
            }

            /// Returns the 3 letter ISO 639-2 B code of the language (the T code is preferred).
            ///
            /// # Examples
            ///
            /// ```
            /// use isolanguage_1::LanguageCode;
            ///
            /// assert_eq!(LanguageCode::Nl.code_b(), "dut");
            /// ```
            #[must_use]
            pub const fn code_b(self) -> &'static str {
                match self {
                    $(Self::$variant => $code_b,)+
                }
            }

            /// Returns the ISO language name.
            ///
            /// # Examples
            ///
            /// ```
            /// use isolanguage_1::LanguageCode;
            ///
            /// assert_eq!(LanguageCode::Cs.name(), "Czech");
            /// ```
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)+
                }
            }

            /// Returns the ISO family of the language.
            ///
            /// # Examples
            ///
            /// ```
            /// use isolanguage_1::LanguageCode;
            ///
            /// assert_eq!(LanguageCode::Kk.family(), "Turkic");
            /// assert_eq!(LanguageCode::Vo.family(), "Constructed");
            /// ```
            #[must_use]
            pub const fn family(self) -> &'static str {
                match self {
                    $(Self::$variant => $family,)+
                }
            }
        }

        impl TryFrom<&str> for LanguageCode {
            type Error = $enum_err_name;

            /// Tries to convert from a two letter language code.
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    $($code => Ok(Self::$variant),)+
                    _ => Err($enum_err_name {
                        language: s.to_owned(),
                    }),
                }
            }
        }

        impl FromStr for $enum_name {
            type Err = $enum_err_name;

            /// Calls TryFrom.
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl Display for $enum_name {
            /// Writes the ISO language name.
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str(self.name())
            }
        }

        /// An error parsing a language from its two letter language code.
        #[derive(Debug, Clone)]
        pub struct $enum_err_name {
            /// The language that could not be parsed.
            pub language: String,
        }

        impl Display for $enum_err_name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{} is not a valid ISO 639-1 2 letter language code", self.language)
            }
        }
    }
}

#[cfg(feature = "iterable")]
impl LanguageCode {
    pub fn codes() -> iter::Codes {
        iter::Codes::default()
    }
    pub fn codes_t() -> iter::CodesT {
        iter::CodesT::default()
    }
    pub fn codes_b() -> iter::CodesB {
        iter::CodesB::default()
    }
    pub fn families() -> iter::Families {
        iter::Families::default()
    }
}

languages_table!(
    LanguageCode,
    ParseError,
    (Ab, "ab", "abk", "abk", "Abkhazian", "Northwest Caucasian"),
    (Aa, "aa", "aar", "aar", "Afar", "Afro-Asiatic"),
    (Af, "af", "afr", "afr", "Afrikaans", "Indo-European"),
    (Ak, "ak", "aka", "aka", "Akan", "Niger–Congo"),
    (Sq, "sq", "sqi", "alb", "Albanian", "Indo-European"),
    (Am, "am", "amh", "amh", "Amharic", "Afro-Asiatic"),
    (Ar, "ar", "ara", "ara", "Arabic", "Afro-Asiatic"),
    (An, "an", "arg", "arg", "Aragonese", "Indo-European"),
    (Hy, "hy", "hye", "arm", "Armenian", "Indo-European"),
    (As, "as", "asm", "asm", "Assamese", "Indo-European"),
    (Av, "av", "ava", "ava", "Avaric", "Northeast Caucasian"),
    (Ae, "ae", "ave", "ave", "Avestan", "Indo-European"),
    (Ay, "ay", "aym", "aym", "Aymara", "Aymaran"),
    (Az, "az", "aze", "aze", "Azerbaijani", "Turkic"),
    (Bm, "bm", "bam", "bam", "Bambara", "Niger–Congo"),
    (Ba, "ba", "bak", "bak", "Bashkir", "Turkic"),
    (Eu, "eu", "eus", "baq", "Basque", "Language isolate"),
    (Be, "be", "bel", "bel", "Belarusian", "Indo-European"),
    (Bn, "bn", "ben", "ben", "Bengali", "Indo-European"),
    (Bh, "bh", "bih", "bih", "Bihari languages", "Indo-European"),
    (Bi, "bi", "bis", "bis", "Bislama", "Creole"),
    (Bs, "bs", "bos", "bos", "Bosnian", "Indo-European"),
    (Br, "br", "bre", "bre", "Breton", "Indo-European"),
    (Bg, "bg", "bul", "bul", "Bulgarian", "Indo-European"),
    (My, "my", "mya", "bur", "Burmese", "Sino-Tibetan"),
    (Ca, "ca", "cat", "cat", "Catalan", "Indo-European"),
    (Ch, "ch", "cha", "cha", "Chamorro", "Austronesian"),
    (Ce, "ce", "che", "che", "Chechen", "Northeast Caucasian"),
    (Ny, "ny", "nya", "nya", "Chichewa", "Niger–Congo"),
    (Zh, "zh", "zho", "chi", "Chinese", "Sino-Tibetan"),
    (Cv, "cv", "chv", "chv", "Chuvash", "Turkic"),
    (Kw, "kw", "cor", "cor", "Cornish", "Indo-European"),
    (Co, "co", "cos", "cos", "Corsican", "Indo-European"),
    (Cr, "cr", "cre", "cre", "Cree", "Algonquian"),
    (Hr, "hr", "hrv", "hrv", "Croatian", "Indo-European"),
    (Cs, "cs", "ces", "cze", "Czech", "Indo-European"),
    (Da, "da", "dan", "dan", "Danish", "Indo-European"),
    (Dv, "dv", "div", "div", "Divehi", "Indo-European"),
    (Nl, "nl", "nld", "dut", "Dutch", "Indo-European"),
    (Dz, "dz", "dzo", "dzo", "Dzongkha", "Sino-Tibetan"),
    (En, "en", "eng", "eng", "English", "Indo-European"),
    (Eo, "eo", "epo", "epo", "Esperanto", "Constructed"),
    (Et, "et", "est", "est", "Estonian", "Uralic"),
    (Ee, "ee", "ewe", "ewe", "Ewe", "Niger–Congo"),
    (Fo, "fo", "fao", "fao", "Faroese", "Indo-European"),
    (Fj, "fj", "fij", "fij", "Fijian", "Austronesian"),
    (Fi, "fi", "fin", "fin", "Finnish", "Uralic"),
    (Fr, "fr", "fra", "fre", "French", "Indo-European"),
    (Ff, "ff", "ful", "ful", "Fulah", "Niger–Congo"),
    (Gl, "gl", "glg", "glg", "Galician", "Indo-European"),
    (Ka, "ka", "kat", "geo", "Georgian", "Kartvelian"),
    (De, "de", "deu", "ger", "German", "Indo-European"),
    (El, "el", "ell", "gre", "Greek", "Indo-European"),
    (Gn, "gn", "grn", "grn", "Guarani", "Tupian"),
    (Gu, "gu", "guj", "guj", "Gujarati", "Indo-European"),
    (Ht, "ht", "hat", "hat", "Haitian", "Creole"),
    (Ha, "ha", "hau", "hau", "Hausa", "Afro-Asiatic"),
    (He, "he", "heb", "heb", "Hebrew", "Afro-Asiatic"),
    (Hz, "hz", "her", "her", "Herero", "Niger–Congo"),
    (Hi, "hi", "hin", "hin", "Hindi", "Indo-European"),
    (Ho, "ho", "hmo", "hmo", "Hiri Motu", "Austronesian"),
    (Hu, "hu", "hun", "hun", "Hungarian", "Uralic"),
    (Ia, "ia", "ina", "ina", "Interlingua", "Constructed"),
    (Id, "id", "ind", "ind", "Indonesian", "Austronesian"),
    (Ie, "ie", "ile", "ile", "Interlingue", "Constructed"),
    (Ga, "ga", "gle", "gle", "Irish", "Indo-European"),
    (Ig, "ig", "ibo", "ibo", "Igbo", "Niger–Congo"),
    (Ik, "ik", "ipk", "ipk", "Inupiaq", "Eskimo–Aleut"),
    (Io, "io", "ido", "ido", "Ido", "Constructed"),
    (Is, "is", "isl", "ice", "Icelandic", "Indo-European"),
    (It, "it", "ita", "ita", "Italian", "Indo-European"),
    (Iu, "iu", "iku", "iku", "Inuktitut", "Eskimo–Aleut"),
    (Ja, "ja", "jpn", "jpn", "Japanese", "Japonic"),
    (Jv, "jv", "jav", "jav", "Javanese", "Austronesian"),
    (Kl, "kl", "kal", "kal", "Kalaallisut", "Eskimo–Aleut"),
    (Kn, "kn", "kan", "kan", "Kannada", "Dravidian"),
    (Kr, "kr", "kau", "kau", "Kanuri", "Nilo-Saharan"),
    (Ks, "ks", "kas", "kas", "Kashmiri", "Indo-European"),
    (Kk, "kk", "kaz", "kaz", "Kazakh", "Turkic"),
    (Km, "km", "khm", "khm", "Central Khmer", "Austroasiatic"),
    (Ki, "ki", "kik", "kik", "Kikuyu", "Niger–Congo"),
    (Rw, "rw", "kin", "kin", "Kinyarwanda", "Niger–Congo"),
    (Ky, "ky", "kir", "kir", "Kirghiz", "Turkic"),
    (Kv, "kv", "kom", "kom", "Komi", "Uralic"),
    (Kg, "kg", "kon", "kon", "Kongo", "Niger–Congo"),
    (Ko, "ko", "kor", "kor", "Korean", "Koreanic"),
    (Ku, "ku", "kur", "kur", "Kurdish", "Indo-European"),
    (Kj, "kj", "kua", "kua", "Kuanyama", "Niger–Congo"),
    (La, "la", "lat", "lat", "Latin", "Indo-European"),
    (Lb, "lb", "ltz", "ltz", "Luxembourgish", "Indo-European"),
    (Lg, "lg", "lug", "lug", "Ganda", "Niger–Congo"),
    (Li, "li", "lim", "lim", "Limburgan", "Indo-European"),
    (Ln, "ln", "lin", "lin", "Lingala", "Niger–Congo"),
    (Lo, "lo", "lao", "lao", "Lao", "Tai–Kadai"),
    (Lt, "lt", "lit", "lit", "Lithuanian", "Indo-European"),
    (Lu, "lu", "lub", "lub", "Luba-Katanga", "Niger–Congo"),
    (Lv, "lv", "lav", "lav", "Latvian", "Indo-European"),
    (Gv, "gv", "glv", "glv", "Manx", "Indo-European"),
    (Mk, "mk", "mkd", "mac", "Macedonian", "Indo-European"),
    (Mg, "mg", "mlg", "mlg", "Malagasy", "Austronesian"),
    (Ms, "ms", "msa", "may", "Malay", "Austronesian"),
    (Ml, "ml", "mal", "mal", "Malayalam", "Dravidian"),
    (Mt, "mt", "mlt", "mlt", "Maltese", "Afro-Asiatic"),
    (Mi, "mi", "mri", "mao", "Maori", "Austronesian"),
    (Mr, "mr", "mar", "mar", "Marathi", "Indo-European"),
    (Mh, "mh", "mah", "mah", "Marshallese", "Austronesian"),
    (Mn, "mn", "mon", "mon", "Mongolian", "Mongolic"),
    (Na, "na", "nau", "nau", "Nauru", "Austronesian"),
    (Nv, "nv", "nav", "nav", "Navajo", "Dené–Yeniseian"),
    (Nd, "nd", "nde", "nde", "North Ndebele", "Niger–Congo"),
    (Ne, "ne", "nep", "nep", "Nepali", "Indo-European"),
    (Ng, "ng", "ndo", "ndo", "Ndonga", "Niger–Congo"),
    (Nb, "nb", "nob", "nob", "Norwegian Bokmål", "Indo-European"),
    (Nn, "nn", "nno", "nno", "Norwegian Nynorsk", "Indo-European"),
    (No, "no", "nor", "nor", "Norwegian", "Indo-European"),
    (Ii, "ii", "iii", "iii", "Sichuan Yi", "Sino-Tibetan"),
    (Nr, "nr", "nbl", "nbl", "South Ndebele", "Niger–Congo"),
    (Oc, "oc", "oci", "oci", "Occitan", "Indo-European"),
    (Oj, "oj", "oji", "oji", "Ojibwa", "Algonquian"),
    (Cu, "cu", "chu", "chu", "Church Slavic", "Indo-European"),
    (Om, "om", "orm", "orm", "Oromo", "Afro-Asiatic"),
    (Or, "or", "ori", "ori", "Oriya", "Indo-European"),
    (Os, "os", "oss", "oss", "Ossetian", "Indo-European"),
    (Pa, "pa", "pan", "pan", "Punjabi", "Indo-European"),
    (Pi, "pi", "pli", "pli", "Pali", "Indo-European"),
    (Fa, "fa", "fas", "per", "Persian", "Indo-European"),
    (Pl, "pl", "pol", "pol", "Polish", "Indo-European"),
    (Ps, "ps", "pus", "pus", "Pashto", "Indo-European"),
    (Pt, "pt", "por", "por", "Portuguese", "Indo-European"),
    (Qu, "qu", "que", "que", "Quechua", "Quechuan"),
    (Rm, "rm", "roh", "roh", "Romansh", "Indo-European"),
    (Rn, "rn", "run", "run", "Rundi", "Niger–Congo"),
    (Ro, "ro", "ron", "rum", "Romanian", "Indo-European"),
    (Ru, "ru", "rus", "rus", "Russian", "Indo-European"),
    (Sa, "sa", "san", "san", "Sanskrit", "Indo-European"),
    (Sc, "sc", "srd", "srd", "Sardinian", "Indo-European"),
    (Sd, "sd", "snd", "snd", "Sindhi", "Indo-European"),
    (Se, "se", "sme", "sme", "Northern Sami", "Uralic"),
    (Sm, "sm", "smo", "smo", "Samoan", "Austronesian"),
    (Sg, "sg", "sag", "sag", "Sango", "Creole"),
    (Sr, "sr", "srp", "srp", "Serbian", "Indo-European"),
    (Gd, "gd", "gla", "gla", "Gaelic", "Indo-European"),
    (Sn, "sn", "sna", "sna", "Shona", "Niger–Congo"),
    (Si, "si", "sin", "sin", "Sinhala", "Indo-European"),
    (Sk, "sk", "slk", "slo", "Slovak", "Indo-European"),
    (Sl, "sl", "slv", "slv", "Slovenian", "Indo-European"),
    (So, "so", "som", "som", "Somali", "Afro-Asiatic"),
    (St, "st", "sot", "sot", "Southern Sotho", "Niger–Congo"),
    (Es, "es", "spa", "spa", "Spanish", "Indo-European"),
    (Su, "su", "sun", "sun", "Sundanese", "Austronesian"),
    (Sw, "sw", "swa", "swa", "Swahili", "Niger–Congo"),
    (Ss, "ss", "ssw", "ssw", "Swati", "Niger–Congo"),
    (Sv, "sv", "swe", "swe", "Swedish", "Indo-European"),
    (Ta, "ta", "tam", "tam", "Tamil", "Dravidian"),
    (Te, "te", "tel", "tel", "Telugu", "Dravidian"),
    (Tg, "tg", "tgk", "tgk", "Tajik", "Indo-European"),
    (Th, "th", "tha", "tha", "Thai", "Tai–Kadai"),
    (Ti, "ti", "tir", "tir", "Tigrinya", "Afro-Asiatic"),
    (Bo, "bo", "bod", "tib", "Tibetan", "Sino-Tibetan"),
    (Tk, "tk", "tuk", "tuk", "Turkmen", "Turkic"),
    (Tl, "tl", "tgl", "tgl", "Tagalog", "Austronesian"),
    (Tn, "tn", "tsn", "tsn", "Tswana", "Niger–Congo"),
    (To, "to", "ton", "ton", "Tonga", "Austronesian"),
    (Tr, "tr", "tur", "tur", "Turkish", "Turkic"),
    (Ts, "ts", "tso", "tso", "Tsonga", "Niger–Congo"),
    (Tt, "tt", "tat", "tat", "Tatar", "Turkic"),
    (Tw, "tw", "twi", "twi", "Twi", "Niger–Congo"),
    (Ty, "ty", "tah", "tah", "Tahitian", "Austronesian"),
    (Ug, "ug", "uig", "uig", "Uighur", "Turkic"),
    (Uk, "uk", "ukr", "ukr", "Ukrainian", "Indo-European"),
    (Ur, "ur", "urd", "urd", "Urdu", "Indo-European"),
    (Uz, "uz", "uzb", "uzb", "Uzbek", "Turkic"),
    (Ve, "ve", "ven", "ven", "Venda", "Niger–Congo"),
    (Vi, "vi", "vie", "vie", "Vietnamese", "Austroasiatic"),
    (Vo, "vo", "vol", "vol", "Volapük", "Constructed"),
    (Wa, "wa", "wln", "wln", "Walloon", "Indo-European"),
    (Cy, "cy", "cym", "wel", "Welsh", "Indo-European"),
    (Wo, "wo", "wol", "wol", "Wolof", "Niger–Congo"),
    (Fy, "fy", "fry", "fry", "Western Frisian", "Indo-European"),
    (Xh, "xh", "xho", "xho", "Xhosa", "Niger–Congo"),
    (Yi, "yi", "yid", "yid", "Yiddish", "Indo-European"),
    (Yo, "yo", "yor", "yor", "Yoruba", "Niger–Congo"),
    (Za, "za", "zha", "zha", "Zhuang", "Tai–Kadai"),
    (Zu, "zu", "zul", "zul", "Zulu", "Niger–Congo"),
);

#[cfg(feature = "iterable")]
pub mod iter {
    use super::LanguageCode;

    pub struct Codes(usize);
    pub struct CodesT(usize);
    pub struct CodesB(usize);
    pub struct Families(usize);

    impl Default for Codes {
        fn default() -> Self {
            Self(0)
        }
    }

    impl Default for CodesT {
        fn default() -> Self {
            Self(0)
        }
    }

    impl Default for CodesB {
        fn default() -> Self {
            Self(0)
        }
    }

    impl Default for Families {
        fn default() -> Self {
            Self(0)
        }
    }

    impl Iterator for Codes {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == CODES.len() {
                return None;
            }
            let current = self.0;
            self.0 += 1;
            Some(CODES[current].code())
        }
    }

    impl Iterator for CodesT {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == CODES.len() {
                return None;
            }
            let current = self.0;
            self.0 += 1;
            Some(CODES[current].code_t())
        }
    }

    impl Iterator for CodesB {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == CODES.len() {
                return None;
            }
            let current = self.0;
            self.0 += 1;
            Some(CODES[current].code_b())
        }
    }

    impl Iterator for Families {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 == FAMILIES.len() {
                return None;
            }
            let current = self.0;
            self.0 += 1;
            Some(FAMILIES[current])
        }
    }
    pub const CODES: [LanguageCode; 184] = [
        LanguageCode::Ab,
        LanguageCode::Aa,
        LanguageCode::Af,
        LanguageCode::Ak,
        LanguageCode::Sq,
        LanguageCode::Am,
        LanguageCode::Ar,
        LanguageCode::An,
        LanguageCode::Hy,
        LanguageCode::As,
        LanguageCode::Av,
        LanguageCode::Ae,
        LanguageCode::Ay,
        LanguageCode::Az,
        LanguageCode::Bm,
        LanguageCode::Ba,
        LanguageCode::Eu,
        LanguageCode::Be,
        LanguageCode::Bn,
        LanguageCode::Bh,
        LanguageCode::Bi,
        LanguageCode::Bs,
        LanguageCode::Br,
        LanguageCode::Bg,
        LanguageCode::My,
        LanguageCode::Ca,
        LanguageCode::Ch,
        LanguageCode::Ce,
        LanguageCode::Ny,
        LanguageCode::Zh,
        LanguageCode::Cv,
        LanguageCode::Kw,
        LanguageCode::Co,
        LanguageCode::Cr,
        LanguageCode::Hr,
        LanguageCode::Cs,
        LanguageCode::Da,
        LanguageCode::Dv,
        LanguageCode::Nl,
        LanguageCode::Dz,
        LanguageCode::En,
        LanguageCode::Eo,
        LanguageCode::Et,
        LanguageCode::Ee,
        LanguageCode::Fo,
        LanguageCode::Fj,
        LanguageCode::Fi,
        LanguageCode::Fr,
        LanguageCode::Ff,
        LanguageCode::Gl,
        LanguageCode::Ka,
        LanguageCode::De,
        LanguageCode::El,
        LanguageCode::Gn,
        LanguageCode::Gu,
        LanguageCode::Ht,
        LanguageCode::Ha,
        LanguageCode::He,
        LanguageCode::Hz,
        LanguageCode::Hi,
        LanguageCode::Ho,
        LanguageCode::Hu,
        LanguageCode::Ia,
        LanguageCode::Id,
        LanguageCode::Ie,
        LanguageCode::Ga,
        LanguageCode::Ig,
        LanguageCode::Ik,
        LanguageCode::Io,
        LanguageCode::Is,
        LanguageCode::It,
        LanguageCode::Iu,
        LanguageCode::Ja,
        LanguageCode::Jv,
        LanguageCode::Kl,
        LanguageCode::Kn,
        LanguageCode::Kr,
        LanguageCode::Ks,
        LanguageCode::Kk,
        LanguageCode::Km,
        LanguageCode::Ki,
        LanguageCode::Rw,
        LanguageCode::Ky,
        LanguageCode::Kv,
        LanguageCode::Kg,
        LanguageCode::Ko,
        LanguageCode::Ku,
        LanguageCode::Kj,
        LanguageCode::La,
        LanguageCode::Lb,
        LanguageCode::Lg,
        LanguageCode::Li,
        LanguageCode::Ln,
        LanguageCode::Lo,
        LanguageCode::Lt,
        LanguageCode::Lu,
        LanguageCode::Lv,
        LanguageCode::Gv,
        LanguageCode::Mk,
        LanguageCode::Mg,
        LanguageCode::Ms,
        LanguageCode::Ml,
        LanguageCode::Mt,
        LanguageCode::Mi,
        LanguageCode::Mr,
        LanguageCode::Mh,
        LanguageCode::Mn,
        LanguageCode::Na,
        LanguageCode::Nv,
        LanguageCode::Nd,
        LanguageCode::Ne,
        LanguageCode::Ng,
        LanguageCode::Nb,
        LanguageCode::Nn,
        LanguageCode::No,
        LanguageCode::Ii,
        LanguageCode::Nr,
        LanguageCode::Oc,
        LanguageCode::Oj,
        LanguageCode::Cu,
        LanguageCode::Om,
        LanguageCode::Or,
        LanguageCode::Os,
        LanguageCode::Pa,
        LanguageCode::Pi,
        LanguageCode::Fa,
        LanguageCode::Pl,
        LanguageCode::Ps,
        LanguageCode::Pt,
        LanguageCode::Qu,
        LanguageCode::Rm,
        LanguageCode::Rn,
        LanguageCode::Ro,
        LanguageCode::Ru,
        LanguageCode::Sa,
        LanguageCode::Sc,
        LanguageCode::Sd,
        LanguageCode::Se,
        LanguageCode::Sm,
        LanguageCode::Sg,
        LanguageCode::Sr,
        LanguageCode::Gd,
        LanguageCode::Sn,
        LanguageCode::Si,
        LanguageCode::Sk,
        LanguageCode::Sl,
        LanguageCode::So,
        LanguageCode::St,
        LanguageCode::Es,
        LanguageCode::Su,
        LanguageCode::Sw,
        LanguageCode::Ss,
        LanguageCode::Sv,
        LanguageCode::Ta,
        LanguageCode::Te,
        LanguageCode::Tg,
        LanguageCode::Th,
        LanguageCode::Ti,
        LanguageCode::Bo,
        LanguageCode::Tk,
        LanguageCode::Tl,
        LanguageCode::Tn,
        LanguageCode::To,
        LanguageCode::Tr,
        LanguageCode::Ts,
        LanguageCode::Tt,
        LanguageCode::Tw,
        LanguageCode::Ty,
        LanguageCode::Ug,
        LanguageCode::Uk,
        LanguageCode::Ur,
        LanguageCode::Uz,
        LanguageCode::Ve,
        LanguageCode::Vi,
        LanguageCode::Vo,
        LanguageCode::Wa,
        LanguageCode::Cy,
        LanguageCode::Wo,
        LanguageCode::Fy,
        LanguageCode::Xh,
        LanguageCode::Yi,
        LanguageCode::Yo,
        LanguageCode::Za,
        LanguageCode::Zu,
    ];

    pub const FAMILIES: [&'static str; 26] = [
        "Afro-Asiatic",
        "Algonquian",
        "Austroasiatic",
        "Austronesian",
        "Aymaran",
        "Constructed",
        "Creole",
        "Dené–Yeniseian",
        "Dravidian",
        "Eskimo–Aleut",
        "Indo-European",
        "Japonic",
        "Kartvelian",
        "Koreanic",
        "Language isolate",
        "Mongolic",
        "Niger–Congo",
        "Nilo-Saharan",
        "Northeast Caucasian",
        "Northwest Caucasian",
        "Quechuan",
        "Sino-Tibetan",
        "Tai–Kadai",
        "Tupian",
        "Turkic",
        "Uralic",
    ];
}

#[cfg(test)]
mod tests {
    use crate::LanguageCode;

    #[test]
    fn codes() {
        assert_eq!(LanguageCode::Ae.code(), "ae");
        assert_eq!(LanguageCode::Ae.code_t(), "ave");
        assert_eq!(LanguageCode::Ae.code_b(), "ave");

        assert_eq!(LanguageCode::Zh.code(), "zh");
        assert_eq!(LanguageCode::Zh.code_t(), "zho");
        assert_eq!(LanguageCode::Zh.code_b(), "chi");

        assert_eq!(LanguageCode::Sg.code(), "sg");
        assert_eq!(LanguageCode::Sg.code_t(), "sag");
        assert_eq!(LanguageCode::Sg.code_b(), "sag");
    }

    #[test]
    fn names_families() {
        assert_eq!(LanguageCode::Ae.name(), "Avestan");
        assert_eq!(LanguageCode::Ae.family(), "Indo-European");

        assert_eq!(LanguageCode::Zh.name(), "Chinese");
        assert_eq!(LanguageCode::Zh.family(), "Sino-Tibetan");

        assert_eq!(LanguageCode::Sg.name(), "Sango");
        assert_eq!(LanguageCode::Sg.family(), "Creole");
    }

    #[test]
    fn parse() {
        assert_eq!("ae".parse::<LanguageCode>().unwrap(), LanguageCode::Ae);
        assert_eq!("zh".parse::<LanguageCode>().unwrap(), LanguageCode::Zh);
        assert_eq!("sg".parse::<LanguageCode>().unwrap(), LanguageCode::Sg);

        assert!("aE".parse::<LanguageCode>().is_err());
        assert!("Zh".parse::<LanguageCode>().is_err());
        assert!("sag".parse::<LanguageCode>().is_err());
    }

    #[test]
    fn format() {
        assert_eq!(LanguageCode::Ae.to_string(), "Avestan");
        assert_eq!(LanguageCode::Zh.to_string(), "Chinese");
        assert_eq!(LanguageCode::Sg.to_string(), "Sango");
    }

    mod iter {
        use super::LanguageCode;

        #[test]
        fn all_codes() {
            let mut codes = LanguageCode::codes();
            assert_eq!(codes.next(), Some("ab"));
            assert_eq!(codes.next(), Some("aa"));
        }

        #[test]
        fn codes_t() {
            let mut codes_t = LanguageCode::codes_t();
            assert_eq!(codes_t.next(), Some("abk"));
            assert_eq!(codes_t.next(), Some("aar"));
        }

        #[test]
        fn codes_b() {
            let mut codes_b = LanguageCode::codes_b();
            assert_eq!(codes_b.next(), Some("abk"));
            assert_eq!(codes_b.next(), Some("aar"));
        }

        #[test]
        fn families() {
            let mut families = LanguageCode::families();
            assert_eq!(families.next(), Some("Afro-Asiatic"));
            assert_eq!(families.next(), Some("Algonquian"));
        }
    }
}
