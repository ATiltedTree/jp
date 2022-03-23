//! Parser and codegen for Kanji characters.
//!
//! Dataset from [KANJIDIC](https://www.edrdg.org/wiki/index.php/KANJIDIC_Project).
//!
//! Download [`kanjidic2.xml`](http://www.edrdg.org/kanjidic/kanjidic2.xml.gz),
//! gunzip it and pass it to this program.

mod kanjidic;

use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args().nth(1).expect("No arg");
    let file = PathBuf::from(file);

    eprint!("READING...");

    let it = std::time::Instant::now();

    let read = std::fs::read_to_string(file)?;

    let parsed: kanjidic::Kanjidic = quick_xml::de::from_str(&read)?;
    eprintln!("  DONE! Took {:#?}", std::time::Instant::now().duration_since(it));

    for chr in parsed.character {
        println!("{}", chr.literal);
    }

    Ok(())
}
