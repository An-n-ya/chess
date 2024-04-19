#[derive(Clone, Copy)]
pub enum Chessman {
    KingRed,
    AdvisorRed,
    ElephantRed,
    HorseRed,
    RookRed,
    CannonRed,
    PawnRed,
    KingBlack,
    AdvisorBlack,
    ElephantBlack,
    HorseBlack,
    RookBlack,
    CannonBlack,
    PawnBlack,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ChessDisplayMode {
    Unicode,
    Character,
}
impl Chessman {
    pub fn to_string(&self, mode: ChessDisplayMode) -> String {
        let c = match mode {
            ChessDisplayMode::Unicode => match self {
                Chessman::KingBlack => "🩧",
                Chessman::AdvisorBlack => "🩨",
                Chessman::ElephantBlack => "🩩",
                Chessman::HorseBlack => "🩪",
                Chessman::RookBlack => "🩫",
                Chessman::CannonBlack => "🩬",
                Chessman::PawnBlack => "🩭",
                Chessman::KingRed => "🩠",
                Chessman::AdvisorRed => "🩡",
                Chessman::ElephantRed => "🩢",
                Chessman::HorseRed => "🩣",
                Chessman::RookRed => "🩤",
                Chessman::CannonRed => "🩥",
                Chessman::PawnRed => "🩦",
            },
            ChessDisplayMode::Character => match self {
                Chessman::KingBlack => "将",
                Chessman::AdvisorBlack => "士",
                Chessman::ElephantBlack => "象",
                Chessman::HorseBlack => "马",
                Chessman::RookBlack => "车",
                Chessman::CannonBlack => "炮",
                Chessman::PawnBlack => "卒",
                Chessman::KingRed => "\x1b[31;1m帅\x1b[0m",
                Chessman::AdvisorRed => "\x1b[31;1m仕\x1b[0m",
                Chessman::ElephantRed => "\x1b[31;1m相\x1b[0m",
                Chessman::HorseRed => "\x1b[31;1m马\x1b[0m",
                Chessman::RookRed => "\x1b[31;1m车\x1b[0m",
                Chessman::CannonRed => "\x1b[31;1m炮\x1b[0m",
                Chessman::PawnRed => "\x1b[31;1m兵\x1b[0m",
            },
        };
        c.to_string()
    }
}

impl From<char> for Chessman {
    fn from(value: char) -> Self {
        match value {
            'k' => Chessman::KingBlack,
            'K' => Chessman::KingRed,
            'a' => Chessman::AdvisorBlack,
            'A' => Chessman::AdvisorRed,
            'e' => Chessman::ElephantBlack,
            'E' => Chessman::ElephantRed,
            'b' => Chessman::ElephantBlack,
            'B' => Chessman::ElephantRed,
            'n' => Chessman::HorseBlack,
            'N' => Chessman::HorseRed,
            'h' => Chessman::HorseBlack,
            'H' => Chessman::HorseRed,
            'r' => Chessman::RookBlack,
            'R' => Chessman::RookRed,
            'c' => Chessman::CannonBlack,
            'C' => Chessman::CannonRed,
            'p' => Chessman::PawnBlack,
            'P' => Chessman::PawnRed,
            _ => panic!("unsupported chess character {value}"),
        }
    }
}
