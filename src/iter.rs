//! Iterators over [LanguageCode](crate::LanguageCode), various codes and families.

use crate::LanguageCode;

impl LanguageCode {
    /// Returns an iterator over all 2-letter country codes.
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::LanguageCode;
    ///
    /// assert!(LanguageCode::codes().find(|code| *code == "en").is_some());
    /// ```
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
    pub fn families() -> Families {
        Families::default()
    }
}

/// An iterator over all [LanguageCode](crate::LanguageCode).
#[derive(Debug, Default, Clone)]
pub struct Iter(usize);

impl Iterator for Iter {
    type Item = LanguageCode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == LANGUAGE_CODES.len() {
            return None;
        }
        let current = self.0;
        self.0 += 1;
        Some(LANGUAGE_CODES[current])
    }
}

#[allow(dead_code)]
impl Iter {
    /// Consumes [Iter](Iter) into [Codes](Codes).
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::{LanguageCode, Iter};
    ///
    /// let mut iter = Iter::default();
    /// assert_eq!(iter.next(), Some(LanguageCode::Ab)); // first entry
    ///
    /// let mut codes = iter.into_codes();
    /// assert_eq!(codes.next(), Some("aa")); // second entry, since we previously iterated once
    /// ```
    pub fn into_codes(self) -> Codes {
        Codes(self)
    }
    /// Consumes [Iter](Iter) into [CodesT](CodesT).
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::{LanguageCode, Iter};
    ///
    /// let mut iter = Iter::default();
    /// assert_eq!(iter.next(), Some(LanguageCode::Ab)); // first entry
    /// assert_eq!(iter.next(), Some(LanguageCode::Aa)); // second entry
    ///
    /// let mut codes = iter.into_codes_t();
    /// assert_eq!(codes.next(), Some("afr")); // third entry, since we previously iterated twice
    /// ```
    pub fn into_codes_t(self) -> CodesT {
        CodesT(self)
    }
    /// Consumes [Iter](Iter) into [CodesB](CodesB).
    ///
    /// # Example
    ///
    /// ```
    /// use isolanguage_1::{LanguageCode, Iter};
    ///
    /// let mut iter = Iter::default();
    /// assert_eq!(iter.next(), Some(LanguageCode::Ab)); // first entry
    /// assert_eq!(iter.next(), Some(LanguageCode::Aa)); // second entry
    /// assert_eq!(iter.next(), Some(LanguageCode::Af)); // third entry
    ///
    /// let mut codes = iter.into_codes_b();
    /// assert_eq!(codes.next(), Some("aka")); // fourth entry, since we previously iterated thrice
    /// ```
    pub fn into_codes_b(self) -> CodesB {
        CodesB(self)
    }
}

/// An iterator over all 2 letter codes.
#[derive(Debug, Default, Clone)]
pub struct Codes(Iter);

/// An iterator over all 3 letter ISO 639-2 T codes.
#[derive(Debug, Default, Clone)]
pub struct CodesT(Iter);

/// An iterator over all 3 letter ISO 639-2 B codes.
#[derive(Debug, Default, Clone)]
pub struct CodesB(Iter);

/// An iterator over all language families.
#[derive(Debug, Default, Clone)]
pub struct Families(usize);

impl Iterator for Codes {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|c| c.code())
    }
}

impl Iterator for CodesT {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|c| c.code_t())
    }
}

impl Iterator for CodesB {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|c| c.code_b())
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

/// All language codes.
pub const LANGUAGE_CODES: [LanguageCode; 184] = [
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

/// All language families, sorted by alphabetical order.
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

#[cfg(test)]
mod tests {
    use crate::iter;
    use crate::LanguageCode;

    #[test]
    fn language_codes() {
        let mut codes = iter::Iter::default();
        assert_eq!(codes.next(), Some(LanguageCode::Ab));
        assert_eq!(codes.next(), Some(LanguageCode::Aa));
    }

    #[test]
    fn codes() {
        let mut codes = iter::Codes::default();
        assert_eq!(codes.next(), Some("ab"));
        assert_eq!(codes.next(), Some("aa"));

        let mut codes = iter::Iter::default().into_codes();
        assert_eq!(codes.next(), Some("ab"));
        assert_eq!(codes.next(), Some("aa"));
    }

    #[test]
    fn codes_t() {
        let mut codes_t = iter::CodesT::default();
        assert_eq!(codes_t.next(), Some("abk"));
        assert_eq!(codes_t.next(), Some("aar"));

        let mut codes_t = iter::Iter::default().into_codes_t();
        assert_eq!(codes_t.next(), Some("abk"));
        assert_eq!(codes_t.next(), Some("aar"));
    }

    #[test]
    fn codes_b() {
        let mut codes_b = iter::CodesB::default();
        assert_eq!(codes_b.next(), Some("abk"));
        assert_eq!(codes_b.next(), Some("aar"));

        let mut codes_b = iter::Iter::default().into_codes_b();
        assert_eq!(codes_b.next(), Some("abk"));
        assert_eq!(codes_b.next(), Some("aar"));
    }

    #[test]
    fn families() {
        let mut families = iter::Families::default();
        assert_eq!(families.next(), Some("Afro-Asiatic"));
        assert_eq!(families.next(), Some("Algonquian"));
    }
}
