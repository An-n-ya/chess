use std::io;

use crate::{
    chessman::{self, Chessman},
    layout::{self, Layout},
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

impl Input {
    const C_NUMBER: [char; 9] = ['一', '二', '三', '四', '五', '六', '七', '八', '九'];
    pub fn new() -> Self {
        Input {
            mode: InputMode::Classic,
        }
    }

    pub fn get(&self, layout: &Layout) -> Move {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if let Some(m) = self.parse_input(&buffer, layout) {
                return m;
            }
            // wrong input, wait another input
        }
    }

    fn parse_input(&self, input: &str, layout: &Layout) -> Option<Move> {
        match self.mode {
            InputMode::Classic => {
                let chars: Vec<char> = input.trim().chars().collect();
                if chars.len() != 4 {
                    eprintln!("input len is not equal to 4, got {}", chars.len());
                    return None;
                }
                let mut chessman: Chessman = chars[0].into();
                if chars[1].is_numeric() {
                    chessman.into_black();
                } else if Self::C_NUMBER.contains(&chars[1]) {
                    chessman.into_red();
                }
                let column = Self::classic_to_coordinate(&chars[1]);
                if let Some(from) = layout.find_chessman_at_column(&chessman, &column) {
                    let n = Self::classic_to_number(&chars[3]);
                    let to = if chars[2] == '平' {
                        (n, from.1)
                    } else if chars[2] == '进' {
                        if n > from.1 {
                            eprintln!("step exceed board");
                            return None;
                        }
                        (from.0, from.1 - n)
                    } else if chars[2] == '退' {
                        if n + from.1 > 9 {
                            eprintln!("step exceed board");
                            return None;
                        }
                        (from.0, from.1 + n)
                    } else {
                        eprintln!("unsupported movement {}", chars[2]);
                        return None;
                    };
                    return Some(Move { from, to });
                } else {
                    eprintln!("cannot find chessman {:?} at column {}", chessman, column);
                    return None;
                }
            }
        }
    }

    fn classic_to_number(c: &char) -> usize {
        if c.is_numeric() {
            *c as usize - 0x30
        } else if Self::C_NUMBER.contains(c) {
            Self::character_to_number(c)
        } else {
            panic!("unsupported number {c}");
        }
    }
    fn classic_to_coordinate(c: &char) -> usize {
        if c.is_numeric() {
            *c as usize - 0x30 - 1
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
