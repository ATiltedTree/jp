use chumsky::prelude::*;

use kanji::Kanji;
use mora::Mora;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quote {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
    Comma,
    FullStop,
    Interpunct,
    Space,
    Ditto,
    Spaceholder,
    Quote(Quote),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Punctuation(Punctuation),

    Hiragana(Mora),
    Katakana(Mora),
    Kanji(Kanji),

    // Catch all for unknown chars
    Other(char),
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let quote =
        choice((just('「').to(Quote::Open), just('」').to(Quote::Close))).map(Punctuation::Quote);

    let punctuation = choice((
        just('、').to(Punctuation::Comma),
        just('。').to(Punctuation::FullStop),
        just('・').to(Punctuation::Interpunct),
        just('〃').to(Punctuation::Ditto),
        just('〇').to(Punctuation::Spaceholder),
        quote,
    ))
    .map(Token::Punctuation);

    let hiragana =
        filter_map(|span, x: char| Mora::try_from_hiragana(x).map_err(|x| Simple::custom(span, x)))
            .map(Token::Hiragana);

    let katakana =
        filter_map(|span, x: char| Mora::try_from_katakana(x).map_err(|x| Simple::custom(span, x)))
            .map(Token::Katakana);

    let kanji = filter_map(|span, x: char| Kanji::try_from(x).map_err(|x| Simple::custom(span, x)))
        .map(Token::Kanji);

    let token = choice((punctuation, hiragana, katakana, kanji)).or(any().map(Token::Other));

    token.repeated().then_ignore(end())
}
