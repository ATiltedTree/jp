//! Parser and codegen for Kanji characters.
//!
//! Dataset from [KANJIDIC](https://www.edrdg.org/wiki/index.php/KANJIDIC_Project).
//!
//! Download [`kanjidic2.xml`](http://www.edrdg.org/kanjidic/kanjidic2.xml.gz),
//! gunzip it and pass it to this program.

mod kanjidic;

use std::io;

use quote::quote;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parsing error: {0}")]
    XML(#[from] quick_xml::DeError),

    #[error(transparent)]
    IO(#[from] io::Error),
}

pub fn gen(reader: &mut impl io::Read, writer: &mut impl io::Write) -> Result<(), Error> {
    let reader = io::BufReader::new(reader);

    eprint!("READING...");

    let it = std::time::Instant::now();

    let parsed: kanjidic::Kanjidic = quick_xml::de::from_reader(reader)?;
    eprintln!(
        "  DONE! Took {:#?}",
        std::time::Instant::now().duration_since(it)
    );

    let chrs: Vec<_> = parsed
        .character
        .into_iter()
        .filter(|x| x.misc.grade != None)
        .filter(|x| x.literal < '欄') // Rustc seems to normalize chars and doesn't like these. we lose 57 chars :(
        .collect();
    eprintln!("{} chars", chrs.len());

    let literals: Vec<_> = chrs
        .iter()
        .map(|x| {
            let lit = quote::format_ident!("{}", x.literal);
            quote::quote! {#lit}
        })
        .collect();

    let s: Vec<_> = chrs
        .iter()
        .map(|x| {
            let s = x.literal;
            quote! {#s}
        })
        .collect();

    let grade = chrs.iter().map(|x| match x.misc.grade.unwrap() {
        x @ 1..=6 => quote! {Grade::Kyoiku(#x)},
        8 => quote! {Grade::Joyo},
        9 => quote! {Grade::Jimeiyo},
        10 => quote! {Grade::JoyoJimeiyo},
        _ => unreachable!("lel"),
    });

    let stroke_count = chrs.iter().map(|x| {
        let c = x.misc.stroke_count[0];
        quote! {#c}
    });

    let kun_reading = chrs.iter().map(|x| {
        x.reading_meaning
            .as_ref()
            .unwrap()
            .rmgroup
            .reading
            .iter()
            .filter_map(|x| match x.r_type {
                kanjidic::ReadingType::JaKun => {
                    let s = &x.body;
                    Some(quote! {#s})
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    });

    let on_reading = chrs.iter().map(|x| {
        x.reading_meaning
            .as_ref()
            .unwrap()
            .rmgroup
            .reading
            .iter()
            .filter_map(|x| match x.r_type {
                kanjidic::ReadingType::JaOn => {
                    let ret = x.body.chars().filter(|x| *x != '-').map(|c| {
                        let c = mora::Mora::try_from_katakana(c).unwrap();
                        match c {
                            mora::Mora::Yoon(yoon) => {
                                let i = quote::format_ident!("{}", format!("{:#?}", yoon));
                                quote! { Mora::Yoon(Yoon::#i) }
                            }
                            _ => {
                                let i = quote::format_ident!("{}", format!("{:#?}", c));
                                quote! { Mora::#i }
                            }
                        }
                    });
                    Some(quote! {&[#(#ret),*]})
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    });

    let def = quote::quote! {
        use mora::{Mora, Yoon};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Kanji {
            #(#literals),*
        }

        impl Kanji {
            pub fn grade(&self) -> Grade {
                match self {
                    #(Self::#literals => #grade),*
                }
            }

            pub fn stroke_count(&self) -> u8 {
                match self {
                    #(Self::#literals => #stroke_count),*
                }
            }

            pub fn kun_readings(&self) -> &[&'static str] {
                match self {
                    #(Self::#literals => &[#(#kun_reading),*]),*
                }
            }

            pub fn on_readings(&self) -> &[&[Mora]] {
                match self {
                    #(Self::#literals => &[#(#on_reading),*]),*
                }
            }
        }


        impl TryFrom<char> for Kanji {
            type Error = KanjiError;

            fn try_from(s: char) -> Result<Kanji, KanjiError> {
                match s {
                    #(#s => Ok(Self::#literals),)*
                    _ => Err(KanjiError(s)),
                }
            }
        }

        impl Into<char> for Kanji {
            fn into(self) -> char {
                match self {
                    #(Self::#literals => #s),*
                }
            }
        }
    };

    writeln!(writer, "{}", def)?;

    Ok(())
}
