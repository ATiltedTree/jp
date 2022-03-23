use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Kanjidic {
    pub character: Vec<Character>,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub literal: String,
    pub misc: Misc,
    pub reading_meaning: Option<ReadingMeaning>,
}

#[derive(Debug, Deserialize)]
pub struct Misc {
    pub grade: Option<u8>,
    pub stroke_count: Vec<u8>,
    pub freq: Option<u32>,
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
    pub r_type: String,

    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct Meaning {
    pub m_lang: Option<String>,

    #[serde(rename = "$value")]
    pub body: String,
}
