use std::{fs::File, path::PathBuf};

const URL: &str = "http://www.edrdg.org/kanjidic/kanjidic2.xml.gz";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::var("OUT_DIR").unwrap();
    let out = PathBuf::from(out).join("gen.rs");
    if let Ok(mut file) = File::create(out) {
        let res = ureq::get(URL).call()?.into_reader();
        let mut decomp = flate2::read::GzDecoder::new(res);

        codegen::gen(&mut decomp, &mut file)?;
    }

    Ok(())
}
