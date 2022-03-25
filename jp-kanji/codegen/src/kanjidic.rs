use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Kanjidic {
    pub character: Vec<Character>,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub literal: char,
    pub misc: Misc,
    pub reading_meaning: Option<ReadingMeaning>,
}

#[derive(Debug, Deserialize)]
pub struct Misc {
    pub grade: Option<u8>,
    pub stroke_count: Vec<u8>,
    #[serde(default)]
    pub freq: u32,
}

#[derive(Debug, Deserialize)]
pub struct ReadingMeaning {
    pub rmgroup: RMGroup,
}

#[derive(Debug, Deserialize)]
pub struct RMGroup {
    #[serde(default)]
    pub reading: Vec<Reading>,
    #[serde(default)]
    pub meaning: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
pub struct Reading {
    pub r_type: ReadingType,

    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadingType {
    Pinyin,
    KoreanR,
    KoreanH,
    Vietnam,
    JaOn,
    JaKun,
}

#[derive(Debug, Deserialize)]
pub struct Meaning {
    #[serde(default)]
    pub m_lang: Lang,

    #[serde(rename = "$value")]
    pub body: String,
}


#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Fr,
    Es,
    Pt,
    En,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::En
    }
}
