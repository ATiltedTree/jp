#![allow(confusable_idents)]

use thiserror::Error;

#[derive(Debug)]
pub enum Grade {
    Kyoiku(u8),
    Joyo,
    Jimeiyo,
    JoyoJimeiyo,
}

#[derive(Debug, Error)]
#[error("'{0}' is not a known kanji character")]
pub struct KanjiError(char);

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

impl std::fmt::Display for Kanji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char((*self).into())
    }
}
