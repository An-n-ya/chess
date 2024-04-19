use core::fmt;

#[derive(Clone, Copy)]
pub enum Chessman {
    King_RED,
    Advisor_RED,
    Elephant_RED,
    Horse_RED,
    Rook_RED,
    Cannon_RED,
    Pawn_RED,
    King_BLACK,
    Advisor_BLACK,
    Elephant_BLACK,
    Horse_BLACK,
    Rook_BLACK,
    Cannon_BLACK,
    Pawn_BLACK,
}

pub enum ChessDisplayMode {
    Unicode,
    Character,
}
impl Chessman {
    pub fn to_string(&self, mode: ChessDisplayMode) -> String {
        let c = match mode {
            ChessDisplayMode::Unicode => match self {
                Chessman::King_BLACK => "🩧",
                Chessman::Advisor_BLACK => "🩨",
                Chessman::Elephant_BLACK => "🩩",
                Chessman::Horse_BLACK => "🩪",
                Chessman::Rook_BLACK => "🩫",
                Chessman::Cannon_BLACK => "🩬",
                Chessman::Pawn_BLACK => "🩭",
                Chessman::King_RED => "🩠",
                Chessman::Advisor_RED => "🩡",
                Chessman::Elephant_RED => "🩢",
                Chessman::Horse_RED => "🩣",
                Chessman::Rook_RED => "🩤",
                Chessman::Cannon_RED => "🩥",
                Chessman::Pawn_RED => "🩦",
            },
            ChessDisplayMode::Character => match self {
                Chessman::King_BLACK => "将",
                Chessman::Advisor_BLACK => "士",
                Chessman::Elephant_BLACK => "象",
                Chessman::Horse_BLACK => "马",
                Chessman::Rook_BLACK => "车",
                Chessman::Cannon_BLACK => "炮",
                Chessman::Pawn_BLACK => "卒",
                Chessman::King_RED => "\x1b[31;1m帅\x1b[0m",
                Chessman::Advisor_RED => "\x1b[31;1m仕\x1b[0m",
                Chessman::Elephant_RED => "\x1b[31;1m相\x1b[0m",
                Chessman::Horse_RED => "\x1b[31;1m马\x1b[0m",
                Chessman::Rook_RED => "\x1b[31;1m车\x1b[0m",
                Chessman::Cannon_RED => "\x1b[31;1m炮\x1b[0m",
                Chessman::Pawn_RED => "\x1b[31;1m兵\x1b[0m",
            },
        };
        c.to_string()
    }
}

impl From<char> for Chessman {
    fn from(value: char) -> Self {
        match value {
            'k' => Chessman::King_BLACK,
            'K' => Chessman::King_RED,
            'a' => Chessman::Advisor_BLACK,
            'A' => Chessman::Advisor_RED,
            'e' => Chessman::Elephant_BLACK,
            'E' => Chessman::Elephant_RED,
            'b' => Chessman::Elephant_BLACK,
            'B' => Chessman::Elephant_RED,
            'n' => Chessman::Horse_BLACK,
            'N' => Chessman::Horse_RED,
            'h' => Chessman::Horse_BLACK,
            'H' => Chessman::Horse_RED,
            'r' => Chessman::Rook_BLACK,
            'R' => Chessman::Rook_RED,
            'c' => Chessman::Cannon_BLACK,
            'C' => Chessman::Cannon_RED,
            'p' => Chessman::Pawn_BLACK,
            'P' => Chessman::Pawn_RED,
            _ => panic!("unsupported chess character {value}"),
        }
    }
}
