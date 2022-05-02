use std::{collections::VecDeque, time::Instant};

use chumsky::Parser;
use jp_lexer::Token;
use mora::Mora;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lexer = jp_lexer::lexer();

    for line in std::env::args().skip(1) {
        let line: Vec<char> = line.chars().collect();
        let it = Instant::now();
        let ts = lexer.parse(line);
        eprintln!("TOOK: {:#?}", it.elapsed());
        match ts {
            Ok(tokens) => {
                let r = tokens
                    .iter()
                    .filter_map(|x| {
                        if let Token::Kanji(kanji) = x {
                            Some(kanji)
                        } else {
                            None
                        }
                    })
                    .fold(1, |i: u128, x| {
                        let on = x.on_readings().len() as u128;
                        let kun = x.kun_readings().len() as u128;
                        let i = if on != 0 { i * on } else { i };
                        let i = if kun != 0 { i * kun } else { i };
                        i
                    });
                eprintln!("POS: {}", r);
                let it = Instant::now();
                println!("{:#?}", tokens);
                let res = process(VecDeque::from(tokens));
                eprintln!("TOOK: {:#?}", it.elapsed());
                println!("{:#?}", res);
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Res {
    Token(Token, Box<Res>),
    Kanji(Vec<Vec<Mora>>, Box<Res>),
    End,
}

fn process(mut it: VecDeque<Token>) -> Res {
    if let Some(i) = it.pop_front() {
        if let Token::Kanji(kanji) = i {
            Res::Kanji(
                kanji
                    .on_readings()
                    .iter()
                    .chain(kanji.kun_readings().iter())
                    .map(|x| Vec::from(*x))
                    .collect::<Vec<_>>(),
                Box::new(process(it.clone())),
            )
        } else {
            Res::Token(i, Box::new(process(it)))
        }
    } else {
        Res::End
    }
}
