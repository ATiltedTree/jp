#![allow(confusable_idents)]

#[derive(Debug)]
pub enum Grade {
    Kyoiku(u8),
    Joyo,
    Jimeiyo,
    JoyoJimeiyo,
}

#[derive(Debug)]
pub struct KanjiError(char);

impl std::fmt::Display for KanjiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a known kanji character", self.0)
    }
}

impl std::error::Error for KanjiError {}

include!("gen.rs");

impl std::fmt::Display for Kanji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char((*self).into())
    }
}
