use crate::input::Move;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    BOARD,
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
                Chessman::BOARD => "",
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
                Chessman::BOARD => "",
            },
        };
        c.to_string()
    }

    pub fn into_red(&mut self) {
        *self = match self.clone() {
            Chessman::KingBlack => Chessman::KingRed,
            Chessman::AdvisorBlack => Chessman::AdvisorRed,
            Chessman::ElephantBlack => Chessman::ElephantRed,
            Chessman::HorseBlack => Chessman::HorseRed,
            Chessman::RookBlack => Chessman::RookRed,
            Chessman::CannonBlack => Chessman::CannonRed,
            Chessman::PawnBlack => Chessman::PawnRed,
            v @ _ => v,
        }
    }
    pub fn into_black(&mut self) {
        *self = match self.clone() {
            Chessman::KingRed => Chessman::KingBlack,
            Chessman::AdvisorRed => Chessman::AdvisorBlack,
            Chessman::ElephantRed => Chessman::ElephantBlack,
            Chessman::HorseRed => Chessman::HorseBlack,
            Chessman::RookRed => Chessman::RookBlack,
            Chessman::CannonRed => Chessman::CannonBlack,
            Chessman::PawnRed => Chessman::PawnBlack,
            v @ _ => v,
        }
    }
    pub fn to_red(&self) -> Self {
        match self {
            Chessman::KingBlack => Chessman::KingRed,
            Chessman::AdvisorBlack => Chessman::AdvisorRed,
            Chessman::ElephantBlack => Chessman::ElephantRed,
            Chessman::HorseBlack => Chessman::HorseRed,
            Chessman::RookBlack => Chessman::RookRed,
            Chessman::CannonBlack => Chessman::CannonRed,
            Chessman::PawnBlack => Chessman::PawnRed,
            v @ _ => *v,
        }
    }
    pub fn to_black(&self) -> Self {
        match self {
            Chessman::KingRed => Chessman::KingBlack,
            Chessman::AdvisorRed => Chessman::AdvisorBlack,
            Chessman::ElephantRed => Chessman::ElephantBlack,
            Chessman::HorseRed => Chessman::HorseBlack,
            Chessman::RookRed => Chessman::RookBlack,
            Chessman::CannonRed => Chessman::CannonBlack,
            Chessman::PawnRed => Chessman::PawnBlack,
            v @ _ => *v,
        }
    }
    pub fn change_color(&self) -> Self {
        if self.is_black() {
            self.to_red()
        } else {
            self.to_black()
        }
    }
    pub fn is_black(&self) -> bool {
        match self {
            Chessman::KingRed => false,
            Chessman::AdvisorRed => false,
            Chessman::ElephantRed => false,
            Chessman::HorseRed => false,
            Chessman::RookRed => false,
            Chessman::CannonRed => false,
            Chessman::PawnRed => false,
            _ => true,
        }
    }
    pub fn is_red(&self) -> bool {
        !self.is_black()
    }
    pub fn is_move_straight(&self) -> bool {
        self.is_king() || self.is_rook() || self.is_cannon() || self.is_pawn()
    }
    pub fn is_king(&self) -> bool {
        *self == Chessman::KingBlack || *self == Chessman::KingRed
    }
    pub fn is_rook(&self) -> bool {
        *self == Chessman::RookBlack || *self == Chessman::RookRed
    }
    pub fn is_cannon(&self) -> bool {
        *self == Chessman::CannonBlack || *self == Chessman::CannonRed
    }
    pub fn is_pawn(&self) -> bool {
        *self == Chessman::PawnBlack || *self == Chessman::PawnRed
    }
    pub fn is_elephant(&self) -> bool {
        *self == Chessman::ElephantBlack || *self == Chessman::ElephantRed
    }
    pub fn is_horse(&self) -> bool {
        *self == Chessman::HorseBlack || *self == Chessman::HorseRed
    }
    pub fn is_advisor(&self) -> bool {
        *self == Chessman::AdvisorBlack || *self == Chessman::AdvisorRed
    }
    pub fn is_different_color(&self, other: &Chessman) -> bool {
        self.is_red() && other.is_black() || self.is_black() && other.is_red()
    }
    pub fn is_cross_river(&self, coordinate: &(usize, usize)) -> bool {
        if self.is_red() {
            coordinate.1 < 5
        } else {
            coordinate.1 >= 5
        }
    }
    pub fn is_move_forward(&self, m: &Move) -> bool {
        if self.is_red() {
            m.to.1 < m.from.1
        } else {
            m.to.1 > m.from.1
        }
    }
    pub fn is_move_backward(&self, m: &Move) -> bool {
        if self.is_red() {
            m.to.1 > m.from.1
        } else {
            m.to.1 < m.from.1
        }
    }
    pub fn is_move_right(&self, m: &Move) -> bool {
        if self.is_red() {
            m.to.0 > m.from.0
        } else {
            m.to.0 < m.from.0
        }
    }
    pub fn is_move_left(&self, m: &Move) -> bool {
        if self.is_red() {
            m.to.0 < m.from.0
        } else {
            m.to.0 > m.from.0
        }
    }
    pub fn is_move_horizontally(&self, m: &Move) -> bool {
        self.is_move_left(m) || self.is_move_right(m)
    }
    pub fn is_move_vertically(&self, m: &Move) -> bool {
        self.is_move_backward(m) || self.is_move_forward(m)
    }
    pub fn is_move_one_step(&self, m: &Move) -> bool {
        let (from, to) = (m.from, m.to);
        from.0.abs_diff(to.0) == 1 && from.1 == to.1 || from.1.abs_diff(to.1) == 1 && from.0 == to.0
    }
    pub fn is_in_palace(&self, coordinate: &(usize, usize)) -> bool {
        if self.is_black() {
            coordinate.0 >= 3 && coordinate.0 <= 5 && coordinate.1 <= 2
        } else {
            coordinate.0 >= 3 && coordinate.0 <= 5 && coordinate.1 >= 7 && coordinate.1 <= 9
        }
    }
}

impl From<char> for Chessman {
    fn from(value: char) -> Self {
        match value {
            'k' => Chessman::KingBlack,
            'K' => Chessman::KingRed,
            '将' => Chessman::KingRed,
            '帅' => Chessman::KingRed,
            'a' => Chessman::AdvisorBlack,
            'A' => Chessman::AdvisorRed,
            '士' => Chessman::AdvisorRed,
            '仕' => Chessman::AdvisorRed,
            'e' => Chessman::ElephantBlack,
            'E' => Chessman::ElephantRed,
            'b' => Chessman::ElephantBlack,
            'B' => Chessman::ElephantRed,
            '相' => Chessman::ElephantRed,
            '象' => Chessman::ElephantRed,
            'n' => Chessman::HorseBlack,
            'N' => Chessman::HorseRed,
            'h' => Chessman::HorseBlack,
            'H' => Chessman::HorseRed,
            '马' => Chessman::HorseRed,
            'r' => Chessman::RookBlack,
            'R' => Chessman::RookRed,
            '车' => Chessman::RookRed,
            'c' => Chessman::CannonBlack,
            'C' => Chessman::CannonRed,
            '炮' => Chessman::CannonRed,
            '砲' => Chessman::CannonRed,
            'p' => Chessman::PawnBlack,
            'P' => Chessman::PawnRed,
            '卒' => Chessman::PawnRed,
            '兵' => Chessman::PawnRed,
            _ => panic!("unsupported chess character {value}"),
        }
    }
}
