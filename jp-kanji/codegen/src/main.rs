use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(std::env::args().nth(1).unwrap())?;
    let mut output = std::io::stdout();

    jp_kanji_codegen::gen(&mut input, &mut output)?;

    Ok(())
}
