use std::io::BufRead;

use chumsky::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin().lock();
    let lexer = jp_lexer::lexer();

    for line in stdin.lines() {
        let line = line?;
        let line: Vec<char> = line.chars().collect();
        let ts = lexer.parse(line);
        println!("{:#?}", ts);
        // match ts {
        //     Ok(tokens) => {
        //         for token in tokens {
        //             if matches!(token, jp_lexer::Token::Other(..)) {
        //                 println!("{:#?}", token);
        //             }
        //         }
        //     },
        //     Err(err) => {
        //         for err in  err {
        //             println!("{:#?}", err);
        //         }
        //     },
        // }
    }

    Ok(())
}
