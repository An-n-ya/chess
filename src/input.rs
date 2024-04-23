use core::fmt;
use std::io;

use crate::{
    chessman::{self, Chessman},
    layout::{self, Layout},
    Chess,
};

pub enum InputMode {
    Classic,
}
pub struct Input {
    mode: InputMode,
}

pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_col = ('a' as u8 + self.from.0 as u8) as char;
        let from_line = 9 - self.from.1;
        let to_col = ('a' as u8 + self.to.0 as u8) as char;
        let to_line = 9 - self.to.1;
        write!(f, "{from_col}{from_line}{to_col}{to_line}")
    }
}

impl Input {
    const C_NUMBER: [char; 9] = ['一', '二', '三', '四', '五', '六', '七', '八', '九'];
    pub fn new() -> Self {
        Input {
            mode: InputMode::Classic,
        }
    }

    pub fn get_move(&self, layout: &Layout) -> Move {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if let Some(m) = self.parse_input(&buffer, layout) {
                return m;
            }
            // wrong input, wait another input
        }
    }

    fn find_chessman(chars: &[char], layout: &Layout) -> Option<(Chessman, (usize, usize))> {
        assert!(chars.len() == 2);
        if chars[0] == '前' || chars[0] == '后' || chars[0] == '中' {
            let chessman: Chessman = chars[1].into();
            if let Some(coordinates) = layout.find_chessman(&chessman) {
                let size = coordinates.len();
                if !(size == 2 || size == 3) {
                    return None;
                }
                let index = if size == 2 {
                    if chars[0] == '前' {
                        0
                    } else if chars[0] == '后' {
                        1
                    } else {
                        unreachable!()
                    }
                } else if size == 3 {
                    if chars[0] == '前' {
                        0
                    } else if chars[0] == '中' {
                        1
                    } else if chars[0] == '后' {
                        2
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                };
                Some((chessman, coordinates[index]))
            } else {
                None
            }
        } else {
            let mut chessman: Chessman = chars[0].into();
            if chars[1].is_numeric() {
                chessman.into_black();
            } else if Self::C_NUMBER.contains(&chars[1]) {
                chessman.into_red();
            }
            let column = Self::classic_to_coordinate(&chars[1]);
            if let Some(from) = layout.find_chessman_at_column(&chessman, &column) {
                Some((chessman, from))
            } else {
                None
            }
        }
    }

    pub fn parse_input(&self, input: &str, layout: &Layout) -> Option<Move> {
        match self.mode {
            InputMode::Classic => {
                let chars: Vec<char> = input.trim().chars().collect();
                if chars.len() != 4 {
                    eprintln!("input len is not equal to 4, got {}", chars.len());
                    return None;
                }
                if let Some((chessman, from)) = Self::find_chessman(&chars[..2], layout) {
                    let n = Self::classic_to_number(&chars[3]);
                    let to = if chars[2] == '平' {
                        let n = Self::classic_to_coordinate(&chars[3]);
                        (n, from.1)
                    } else if chars[2] == '进' && chessman.is_red()
                        || chars[2] == '退' && chessman.is_black()
                    {
                        if chessman.is_move_straight() {
                            (from.0, from.1 - n)
                        } else {
                            let n = Self::classic_to_coordinate(&chars[3]);
                            if chessman.is_horse() {
                                (n, from.1 - 2)
                            } else if chessman.is_advisor() {
                                (n, from.1 - 1)
                            } else if chessman.is_elephant() {
                                (n, from.1 - 2)
                            } else {
                                unreachable!()
                            }
                        }
                    } else if chars[2] == '退' && chessman.is_red()
                        || chars[2] == '进' && chessman.is_black()
                    {
                        if chessman.is_move_straight() {
                            (from.0, from.1 + n)
                        } else {
                            let n = Self::classic_to_coordinate(&chars[3]);
                            if chessman.is_horse() {
                                if n.abs_diff(from.0) == 1 {
                                    (n, from.1 + 2)
                                } else if n.abs_diff(from.0) == 2 {
                                    (n, from.1 + 1)
                                } else {
                                    eprintln!("invalid movement, move to far");
                                    return None;
                                }
                            } else if chessman.is_advisor() {
                                (n, from.1 + 1)
                            } else if chessman.is_elephant() {
                                (n, from.1 + 2)
                            } else {
                                unreachable!()
                            }
                        }
                    } else {
                        eprintln!("unsupported movement {}", chars[2]);
                        return None;
                    };
                    let m = Move { from, to };
                    if layout.is_valid_move(&m) {
                        return Some(m);
                    } else {
                        eprintln!("invalid movement from {:?} to {:?}", from, to);
                        return None;
                    }
                } else {
                    eprintln!("cannot find chessman {:?}", &chars[..2]);
                    return None;
                }
            }
        }
    }

    fn classic_to_number(c: &char) -> usize {
        if c.is_numeric() {
            usize::from_str_radix(&format!("{c}"), 10).unwrap()
        } else if Self::C_NUMBER.contains(c) {
            Self::character_to_number(c)
        } else {
            panic!("unsupported number {c}");
        }
    }
    fn classic_to_coordinate(c: &char) -> usize {
        if c.is_numeric() {
            usize::from_str_radix(&format!("{c}"), 10).unwrap() - 1
        } else if Self::C_NUMBER.contains(c) {
            let n = Self::character_to_number(c);
            9 - n
        } else {
            panic!("unsupported number {c}");
        }
    }

    fn character_to_number(c: &char) -> usize {
        match c {
            '一' => 1,
            '二' => 2,
            '三' => 3,
            '四' => 4,
            '五' => 5,
            '六' => 6,
            '七' => 7,
            '八' => 8,
            '九' => 9,
            _ => panic!("unsupported number {c}"),
        }
    }
}
