use jp_kanji::Kanji;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for c in std::env::args().nth(1).unwrap().chars() {
        let kanji = Kanji::try_from(c)?;
        println!("{}:", kanji);
        println!("  Grade: {:?}", kanji.grade());
        println!("  Stroke Count: {}", kanji.stroke_count());
        println!("  On Readings:");
        for reading in kanji.on_readings() {
            println!("    - {:?}", reading);
        }
        println!("  Kun Readings:");
        for reading in kanji.kun_readings() {
            println!("    - {}", reading);
        }
    }

    Ok(())
}
