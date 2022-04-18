use chumsky::Parser;
use mora::Mora;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lexer = jp_lexer::lexer();

    for line in std::env::args().skip(1) {
        let line: Vec<char> = line.chars().collect();
        let ts = lexer.parse(line);
        match ts {
            Ok(tokens) => {
                for token in tokens {
                    match token {
                        jp_lexer::Token::Comma => print!(","),
                        jp_lexer::Token::FullStop => print!("."),
                        jp_lexer::Token::Odoriji => print!("AA"),
                        jp_lexer::Token::QuoteOpen => print!("\""),
                        jp_lexer::Token::QuoteClose => print!("\""),
                        jp_lexer::Token::ParenOpen => print!("("),
                        jp_lexer::Token::ParenClose => print!(")"),
                        jp_lexer::Token::Space => print!(" "),
                        jp_lexer::Token::Interpunct => print!(":"),
                        jp_lexer::Token::Hiragana(hira) => print!("{hira:?}"),
                        jp_lexer::Token::Katakana(kata) => print!("{kata:?}"),
                        jp_lexer::Token::Kanji(kanji) => {
                            let reading = kanji.on_readings()[0];
                            for mora in reading {
                                if !matches!(mora, Mora::Yoon(_)) {
                                    print!("{mora:?}");
                                }
                            }
                        }
                    }
                }
                println!();
            }
            Err(err) => {
                for err in err {
                    println!("{:#?}", err);
                }
            }
        }
    }

    Ok(())
}
