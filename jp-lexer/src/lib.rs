#![feature(if_let_guard)]
#![recursion_limit = "256"]

use chumsky::prelude::*;

mod mora;

pub use mora::Mora;

#[derive(Debug, Clone, Copy)]
pub enum Yoon {
    Ya,
    Yu,
    Yo,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Sokuon,
    Choonpu,
    Comma,
    FullStop,
    Yoon(Yoon),
    Odoriji,
    QuoteOpen,
    QuoteClose,
    Space,

    Hiragana(Mora),
    Katakana(Mora),
    Kanji(char),
    Other(char),
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let sokuon = one_of("ッっ").to(Token::Sokuon);
    let odoriji = one_of("ヽゝ").to(Token::Sokuon);
    let choonpu = just("ー").to(Token::Choonpu);
    let comma = just('、').to(Token::Comma);
    let space = just('　').to(Token::Space);
    let full_stop = just('。').to(Token::FullStop);
    let quote_open = one_of("「『").to(Token::QuoteOpen);
    let quote_close = one_of("」』").to(Token::QuoteClose);
    let yoon = choice((
        one_of("ャゃ").to(Yoon::Ya),
        one_of("ュゅ").to(Yoon::Yu),
        one_of("ョょ").to(Yoon::Yo),
    )).map(Token::Yoon);
    let hiragana = mora::hiragana().map(Token::Hiragana);
    let katakana = mora::katakana().map(Token::Katakana);

    let kanji = filter(|x| {
        unicode_script::Script::from(*x) == unicode_script::Script::Han
    }).map(Token::Kanji);

    let token = choice((
        sokuon,
        odoriji,
        choonpu,
        comma,
        space,
        full_stop,
        quote_open,
        quote_close,
        yoon,
        hiragana,
        katakana,
        kanji,
    )).or(any().map(Token::Other));

    token.repeated().then_ignore(end())
}

