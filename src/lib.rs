//! This crate implements the [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) standard in Rust.
//! It also has optional Serde support, by using the `serde` feature:
//!
//! ```toml
//! isolanguage-1 = { version = "0.2.2", features = ["serde"] }
//! ```
//!
//! The main type is the `LanguageCode` type, which is an enum for every single language in ISO
//! 639-1. It optionally implements Serialize and Deserialize too.

use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::iter::FusedIterator;
use std::ops::Range;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! languages_table {
    ($(($variant:ident, $code:literal, $code_t:literal, $code_b:literal, $name:literal, $family:literal),)+) => {
        /// An enumeration of all ISO 639-1 language codes.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum LanguageCode {
            $(
                #[doc=$name]
                #[cfg_attr(feature = "serde", serde(rename=$code))]
                $variant,
            )+
        }

        impl LanguageCode {
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
            type Error = ParseError;

            /// Tries to convert from a two letter language code.
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    $($code => Ok(Self::$variant),)+
                    _ => Err(ParseError {
                        language: s.to_owned(),
                    }),
                }
            }
        }

        impl FromStr for LanguageCode {
            type Err = ParseError;

            /// Calls TryFrom.
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl Display for LanguageCode {
            /// Writes the ISO language name.
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str(self.name())
            }
        }

        /// An array of every ISO 639-1 language code.
        pub const LANGUAGE_CODES: [LanguageCode; 184] = [$(LanguageCode::$variant,)*];
    }
}

languages_table! {
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
}

impl LanguageCode {
    /// Returns an iterator over every ISO 639-1 language code.
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::LanguageCode;
    ///
    /// assert!(LanguageCode::iter().find(|&code| code == LanguageCode::Wo).is_some());
    /// ```
    #[inline]
    pub fn iter() -> Iter {
        Iter::default()
    }

    /// Returns an iterator over all 2-letter language codes.
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::LanguageCode;
    ///
    /// assert!(LanguageCode::codes().find(|code| *code == "en").is_some());
    /// ```
    #[inline]
    pub fn codes() -> Codes {
        Codes::default()
    }

    /// Returns an iterator over all 3 letter ISO 639-2 T codes.
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::LanguageCode;
    ///
    /// assert!(LanguageCode::codes_t().find(|code| *code == "ave").is_some());
    /// ```
    #[inline]
    pub fn codes_t() -> CodesT {
        CodesT::default()
    }

    /// Returns an iterator over all 3 letter ISO 639-2 B codes.
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::LanguageCode;
    ///
    /// assert!(LanguageCode::codes_b().find(|code| *code == "chi").is_some());
    /// ```
    #[inline]
    pub fn codes_b() -> CodesB {
        CodesB::default()
    }

    /// Returns an iterator over all language families.
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::LanguageCode;
    ///
    /// assert!(LanguageCode::families().find(|family| *family == "Algonquian").is_some());
    /// ```
    #[inline]
    pub fn families() -> Families {
        Families::default()
    }
}

/// All language families, sorted by alphabetical order.
pub const FAMILIES: [&str; 26] = [
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

macro_rules! static_array_iterators {
    ($($(#[doc = $doc:literal])* $name:ident($array:ident) -> $item:ty $({ $($mapper:tt)* })?,)*) => { $(
        $(#[doc = $doc])*
        #[derive(Debug, Clone)]
        pub struct $name(Range<u32>);

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self(0..$array.len() as u32)
            }
        }

        impl Iterator for $name {
            type Item = $item;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|i| $array[i as usize])$(.map($($mapper)*))?
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.0.nth(n).map(|i| $array[i as usize])$(.map($($mapper)*))?
            }

            #[inline]
            fn last(mut self) -> Option<Self::Item> {
                self.next_back()
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }

            #[inline]
            fn count(self) -> usize {
                self.len()
            }
        }

        impl DoubleEndedIterator for $name {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back().map(|i| $array[i as usize])$(.map($($mapper)*))?
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                self.0.nth_back(n).map(|i| $array[i as usize])$(.map($($mapper)*))?
            }
        }

        impl ExactSizeIterator for $name {
            #[inline]
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl FusedIterator for $name {}
    )* }
}
static_array_iterators! {
    /// An iterator over every [`LanguageCode`], created by [`LanguageCode::iter`].
    Iter(LANGUAGE_CODES) -> LanguageCode,

    /// An iterator over every 2-letter language code, created by [`LanguageCode::codes`]
    Codes(LANGUAGE_CODES) -> &'static str { LanguageCode::code },

    /// An iterator over ISO 639-2 T codes, created by [`LanguageCode::codes_t`].
    CodesT(LANGUAGE_CODES) -> &'static str { LanguageCode::code_t },

    /// An iterator over ISO 639-2 B codes, created by [`LanguageCode::codes_b`].
    CodesB(LANGUAGE_CODES) -> &'static str { LanguageCode::code_b },

    /// An iterator over all language families, created by [`LanguageCode::families`].
    Families(FAMILIES) -> &'static str,
}

/// An error parsing a language from its two letter language code.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The language that could not be parsed.
    pub language: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} is not a valid ISO 639-1 2 letter language code",
            self.language
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Families, LanguageCode};

    #[test]
    fn code_strings() {
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

    #[test]
    fn language_codes() {
        let mut codes = LanguageCode::iter();
        assert_eq!(codes.next(), Some(LanguageCode::Ab));
        assert_eq!(codes.next(), Some(LanguageCode::Aa));
    }

    #[test]
    fn codes() {
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
        let mut families = Families::default();
        assert_eq!(families.next(), Some("Afro-Asiatic"));
        assert_eq!(families.next(), Some("Algonquian"));
        assert_eq!(families.by_ref().count(), 24);
        assert_eq!(families.next(), None);
    }
}
