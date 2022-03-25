use chumsky::prelude::*;

#[derive(Debug)]
pub enum Vowel {
    A,
    I,
    U,
    E,
    O,
}

macro_rules! mora {
    ({$hira:expr => $name:ident}) => {
        just($hira).to(Mora::$name)
    };
    ({$hira:expr => $name:ident} $({$H:expr => $N:ident})*) => {
        just($hira).to(Mora::$name).or(mora!($({$H => $N})*))
    }
}

macro_rules! moras {
    ($($hira:literal | $kata:literal: $name:ident => { $vowel:expr, $consonant:expr }),+) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy)]
        pub enum Mora {
            $($name),+
        }

        impl Mora {
            pub fn vowel(&self) -> Option<Vowel> {
                match self {
                    $(Self::$name => $vowel),+
                }
            }

            pub fn hiragana_letter(&self) -> char {
                match self {
                    $(Self::$name => $hira),+
                }
            }

            pub fn katakana_letter(&self) -> char {
                match self {
                    $(Self::$name => $kata),+
                }
            }
        }

        pub(crate) fn hiragana() -> impl Parser<char, Mora, Error = Simple<char>> {
            mora!($({$hira => $name})*)
        }

        pub(crate) fn katakana() -> impl Parser<char, Mora, Error = Simple<char>> {
            mora!($({$kata => $name})*)
        }

    };
}

moras! {
    'あ' | 'ア': A => { Some(Vowel::A), None },
    'ぁ' | 'ァ': a => { Some(Vowel::A), None },
    'い' | 'イ': I => { Some(Vowel::I), None },
    'ぃ' | 'ィ': i => { Some(Vowel::I), None },
    'う' | 'ウ': U => { Some(Vowel::U), None },
    'ぅ' | 'ゥ': u => { Some(Vowel::U), None },
    'え' | 'エ': E => { Some(Vowel::E), None },
    'ぇ' | 'ェ': e => { Some(Vowel::E), None },
    'お' | 'オ': O => { Some(Vowel::O), None },
    'ぉ' | 'ォ': o => { Some(Vowel::O), None },
    'か' | 'カ': KA => { Some(Vowel::A), Some(Consonant::K) },
    'き' | 'キ': KI => { Some(Vowel::I), Some(Consonant::K) },
    'く' | 'ク': KU => { Some(Vowel::U), Some(Consonant::K) },
    'け' | 'ケ': KE => { Some(Vowel::E), Some(Consonant::K) },
    'こ' | 'コ': KO => { Some(Vowel::O), Some(Consonant::K) },
    'さ' | 'サ': SA => { Some(Vowel::A), Some(Consonant::S) },
    'し' | 'シ': SHI => { Some(Vowel::I), Some(Consonant::S) },
    'す' | 'ス': SU => { Some(Vowel::U), Some(Consonant::S) },
    'せ' | 'セ': SE => { Some(Vowel::E), Some(Consonant::S) },
    'そ' | 'ソ': SO => { Some(Vowel::O), Some(Consonant::S) },
    'た' | 'タ': TA => { Some(Vowel::A), Some(Consonant::T) },
    'ち' | 'チ': CHI => { Some(Vowel::I), Some(Consonant::T) },
    'つ' | 'ツ': TSU => { Some(Vowel::U), Some(Consonant::T) },
    'て' | 'テ': TE => { Some(Vowel::E), Some(Consonant::T) },
    'と' | 'ト': TO => { Some(Vowel::O), Some(Consonant::T) },
    'な' | 'ナ': NA => { Some(Vowel::A), Some(Consonant::N) },
    'に' | 'ニ': NI => { Some(Vowel::I), Some(Consonant::N) },
    'ぬ' | 'ヌ': NU => { Some(Vowel::U), Some(Consonant::N) },
    'ね' | 'ネ': NE => { Some(Vowel::E), Some(Consonant::N) },
    'の' | 'ノ': NO => { Some(Vowel::O), Some(Consonant::N) },
    'は' | 'ハ': HA => { Some(Vowel::A), Some(Consonant::H) },
    'ひ' | 'ヒ': HI => { Some(Vowel::I), Some(Consonant::H) },
    'ふ' | 'フ': FU => { Some(Vowel::U), Some(Consonant::H) },
    'へ' | 'ヘ': HE => { Some(Vowel::E), Some(Consonant::H) },
    'ほ' | 'ホ': HO => { Some(Vowel::O), Some(Consonant::H) },
    'ま' | 'マ': MA => { Some(Vowel::A), Some(Consonant::M) },
    'み' | 'ミ': MI => { Some(Vowel::I), Some(Consonant::M) },
    'む' | 'ム': MU => { Some(Vowel::U), Some(Consonant::M) },
    'め' | 'メ': ME => { Some(Vowel::E), Some(Consonant::M) },
    'も' | 'モ': MO => { Some(Vowel::O), Some(Consonant::M) },
    'や' | 'ヤ': YA => { Some(Vowel::A), Some(Consonant::Y) },
    'ゆ' | 'ユ': YU => { Some(Vowel::U), Some(Consonant::Y) },
    'よ' | 'ヨ': YO => { Some(Vowel::O), Some(Consonant::Y) },
    'ら' | 'ラ': RA => { Some(Vowel::A), Some(Consonant::R) },
    'り' | 'リ': RI => { Some(Vowel::I), Some(Consonant::R) },
    'る' | 'ル': RU => { Some(Vowel::U), Some(Consonant::R) },
    'れ' | 'レ': RE => { Some(Vowel::E), Some(Consonant::R) },
    'ろ' | 'ロ': RO => { Some(Vowel::O), Some(Consonant::R) },
    'わ' | 'ワ': WA => { Some(Vowel::A), Some(Consonant::W) },
    'ゐ' | 'ヰ': WI => { Some(Vowel::I), Some(Consonant::W) },
    'ゑ' | 'ヱ': WE => { Some(Vowel::E), Some(Consonant::W) },
    'を' | 'ヲ': WO => { Some(Vowel::O), Some(Consonant::W) },
    'が' | 'ガ': GA => { Some(Vowel::A), Some(Consonant::G) },
    'ぎ' | 'ギ': GI => { Some(Vowel::I), Some(Consonant::G) },
    'ぐ' | 'グ': GU => { Some(Vowel::U), Some(Consonant::G) },
    'げ' | 'ゲ': GE => { Some(Vowel::E), Some(Consonant::G) },
    'ご' | 'ゴ': GO => { Some(Vowel::O), Some(Consonant::G) },
    'ざ' | 'ザ': ZA => { Some(Vowel::A), Some(Consonant::Z) },
    'じ' | 'ジ': JI => { Some(Vowel::I), Some(Consonant::Z) },
    'ず' | 'ズ': ZU => { Some(Vowel::U), Some(Consonant::Z) },
    'ぜ' | 'ゼ': ZE => { Some(Vowel::E), Some(Consonant::Z) },
    'ぞ' | 'ゾ': ZO => { Some(Vowel::O), Some(Consonant::Z) },
    'だ' | 'ダ': DA => { Some(Vowel::A), Some(Consonant::D) },
    'ぢ' | 'ヂ': DJI => { Some(Vowel::I), Some(Consonant::D) },
    'づ' | 'ヅ': DZU => { Some(Vowel::U), Some(Consonant::D) },
    'で' | 'デ': DE => { Some(Vowel::E), Some(Consonant::D) },
    'ど' | 'ド': DO => { Some(Vowel::O), Some(Consonant::D) },
    'ば' | 'バ': BA => { Some(Vowel::A), Some(Consonant::B) },
    'び' | 'ビ': BI => { Some(Vowel::I), Some(Consonant::B) },
    'ぶ' | 'ブ': BU => { Some(Vowel::U), Some(Consonant::B) },
    'べ' | 'ベ': BE => { Some(Vowel::E), Some(Consonant::B) },
    'ぼ' | 'ボ': BO => { Some(Vowel::O), Some(Consonant::B) },
    'ぱ' | 'パ': PA => { Some(Vowel::A), Some(Consonant::P) },
    'ぴ' | 'ピ': PI => { Some(Vowel::I), Some(Consonant::P) },
    'ぷ' | 'プ': PU => { Some(Vowel::U), Some(Consonant::P) },
    'ぺ' | 'ペ': PE => { Some(Vowel::E), Some(Consonant::P) },
    'ぽ' | 'ポ': PO => { Some(Vowel::O), Some(Consonant::P) },
    'ん' | 'ン': N => { None, Some(Consonant::N) }
}
