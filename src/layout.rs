use core::fmt;

use crate::{
    chessman::{ChessDisplayMode, Chessman},
    constants::BOARD,
};

pub struct Layout {
    pub board: [[Option<Chessman>; 9]; 10],
    mode: ChessDisplayMode,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            board: [[None; 9]; 10],
            mode: ChessDisplayMode::Character,
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = 33;
        let height = 19;
        let board = BOARD.to_string();
        let board: Vec<&str> = board.split('\n').collect();
        writeln!(f, "1   2   3   4   5   6   7   8   9").unwrap();

        for j in 0..height {
            let mut has_chessman = false;
            for i in 0..width {
                let is_chessman = i % 4 == 0 && j % 2 == 0;
                if has_chessman {
                    has_chessman = false;
                    continue;
                }
                if is_chessman {
                    let x = i / 4;
                    let y = j / 2;
                    if let Some(c) = self.board[y][x] {
                        has_chessman = true;
                        write!(f, "{}", c.to_string(self.mode)).unwrap();
                        if self.mode == ChessDisplayMode::Unicode {
                            write!(f, " ").unwrap();
                        }
                    } else {
                        write!(f, "{}", board[j].chars().nth(i).unwrap()).unwrap();
                    }
                } else {
                    write!(f, "{}", board[j].chars().nth(i).unwrap()).unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f, "九  八  七  六  五  四  三  二  一").unwrap();
        Ok(())
    }
}
