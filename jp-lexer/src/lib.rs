use chumsky::prelude::*;

use ast::Mora;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Comma,
    FullStop,
    Odoriji,
    QuoteOpen,
    QuoteClose,
    ParenOpen,
    ParenClose,
    Space,
    Interpunct,

    Hiragana(Mora),
    Katakana(Mora),
    Kanji(kanji::Kanji),
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let comma = just('、').to(Token::Comma);
    let space = just('　').to(Token::Space);
    let interpunct = just('・').to(Token::Interpunct);
    let full_stop = just('。').to(Token::FullStop);
    let quote_open = one_of("「『").to(Token::QuoteOpen);
    let quote_close = one_of("」』").to(Token::QuoteClose);
    let paren_open = just('（').to(Token::ParenOpen);
    let paren_close = just('）').to(Token::ParenClose);

    let hiragana =
        filter_map(|span, x: char| Mora::try_from_hiragana(x).map_err(|x| Simple::custom(span, x)))
            .map(Token::Hiragana);

    let katakana =
        filter_map(|span, x: char| Mora::try_from_katakana(x).map_err(|x| Simple::custom(span, x)))
            .map(Token::Katakana);

    let kanji =
        filter_map(|span, x: char| kanji::Kanji::try_from(x).map_err(|x| Simple::custom(span, x)))
            .map(Token::Kanji);

    let token = choice((
        comma,
        space,
        interpunct,
        full_stop,
        quote_open,
        quote_close,
        paren_open,
        paren_close,
        hiragana,
        katakana,
        kanji,
    ));

    token.repeated().then_ignore(end())
}
