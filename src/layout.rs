use core::fmt;

use crate::{
    chessman::{ChessDisplayMode, Chessman},
    constants::BOARD,
    input::Move,
};

pub struct Layout {
    pub board: [[Option<Chessman>; 9]; 10],
    mode: ChessDisplayMode,
}

impl Layout {
    const HEIGHT: usize = 10;
    const WIDTH: usize = 9;
    pub fn new() -> Self {
        Self {
            board: [[None; 9]; 10],
            mode: ChessDisplayMode::Character,
        }
    }
    pub fn find_chessman_at_column(
        &self,
        chessman: &Chessman,
        column: &usize,
    ) -> Option<(usize, usize)> {
        for line in 0..10 {
            if let Some(c) = self.board[line][*column] {
                if c == *chessman {
                    return Some((*column, line));
                }
            }
        }
        None
    }
    pub fn find_chessman(&self, chessman: &Chessman) -> Option<Vec<(usize, usize)>> {
        let mut res = vec![];
        for line in 0..10 {
            for col in 0..9 {
                if let Some(c) = self.board[line][col] {
                    if c == *chessman {
                        res.push((col, line));
                    }
                }
            }
        }
        Some(res)
    }

    pub fn to_fen_string(&self) -> String {
        let mut res = "".to_string();
        let mut cnt = 0u8;
        for line in 0..10 {
            for col in 0..9 {
                if let Some(c) = self.board[line][col] {
                    if cnt != 0 {
                        res.push((0x30 + cnt) as char);
                        cnt = 0;
                    }
                    res.push(c.to_fen_char());
                } else {
                    cnt += 1;
                }
            }
            if cnt != 0 {
                res.push((0x30 + cnt) as char);
                cnt = 0;
            }
            if line != 9 {
                res.push('/');
            }
        }
        res
    }

    pub fn handle_move(&mut self, m: &Move) {
        let position = self.get_mut(m.from.1, m.from.0);
        if position.is_none() {
            // FIXME: find a better way to print coordinate
            panic!("cannot find chessman on position {:?}", m.from);
        }
        let chessman = position.clone().unwrap();
        *position = None;
        let position = self.get_mut(m.to.1, m.to.0);
        *position = Some(chessman);
    }

    pub fn is_valid_move(&self, m: &Move) -> bool {
        let chessman = self
            .get_at(m.from)
            .expect(&format!("cannot find a chessman at position {:?}", m.from));
        let from = m.from;
        let to = m.to;
        let position = self.get_at(m.to);
        if let Some(c) = position {
            if !chessman.is_different_color(&c) {
                return false;
            }
        }
        if chessman.is_pawn() {
            if chessman.is_cross_river(&from) {
                if chessman.is_move_backward(m) {
                    return false;
                }
            } else {
                if chessman.is_move_horizontally(m) || chessman.is_move_backward(m) {
                    return false;
                }
            }
            if !chessman.is_move_one_step(m) {
                return false;
            }
        } else if chessman.is_cannon() {
            let move_in_horizontal = from.1.abs_diff(to.1);
            let move_in_vertical = from.0.abs_diff(to.0);
            if move_in_horizontal != 0 && move_in_vertical != 0 {
                return false;
            }
            let bypass_num = self.get_bypass_chessman_num(m);
            if bypass_num != 0 && bypass_num != 2 {
                return false;
            }
        } else if chessman.is_rook() {
            let move_in_horizontal = from.1.abs_diff(to.1);
            let move_in_vertical = from.0.abs_diff(to.0);
            if move_in_horizontal != 0 && move_in_vertical != 0 {
                return false;
            }
            if self.get_bypass_chessman_num(m) > 1 {
                return false;
            }
        } else if chessman.is_horse() {
            let move_in_horizontal = from.1.abs_diff(to.1);
            let move_in_vertical = from.0.abs_diff(to.0);
            if !((move_in_horizontal == 1 && move_in_vertical == 2)
                || (move_in_horizontal == 2 && move_in_vertical == 1))
            {
                return false;
            }
            if move_in_horizontal == 2 {
                if chessman.is_move_backward(m) {
                    let back_position = self.get_back_of_chessman(from, &chessman);
                    if back_position.is_some() {
                        return false;
                    }
                } else if chessman.is_move_forward(m) {
                    let front_position = self.get_front_of_chessman(from, &chessman);
                    if front_position.is_some() {
                        return false;
                    }
                }
            } else if move_in_horizontal == 1 {
                if chessman.is_move_left(m) {
                    let position = self.get_left_of_chessman(from, &chessman);
                    if position.is_some() {
                        return false;
                    }
                } else if chessman.is_move_right(m) {
                    let position = self.get_right_of_chessman(from, &chessman);
                    if position.is_some() {
                        return false;
                    }
                }
            }
        } else if chessman.is_elephant() {
            let move_in_horizontal = from.1 as i8 - to.1 as i8;
            let move_in_vertical = from.0 as i8 - to.0 as i8;
            if move_in_horizontal.abs() != 2 || move_in_vertical.abs() != 2 {
                return false;
            }
            let n_cor = (
                from.0 as i8 + move_in_horizontal / 2,
                from.1 as i8 + move_in_vertical / 2,
            );
            if !Self::is_valid_coordinate(&n_cor) {
                return false;
            }
            let n_cor = (n_cor.0 as usize, n_cor.1 as usize);
            let position = self.get_at(n_cor);
            if position.is_some() {
                return false;
            }
        } else if chessman.is_advisor() {
            if !chessman.is_in_palace(&to) {
                return false;
            }
            let move_in_horizontal = from.1.abs_diff(to.1);
            let move_in_vertical = from.0.abs_diff(to.0);
            if move_in_horizontal != 1 || move_in_vertical != 1 {
                return false;
            }
        } else if chessman.is_king() {
            if !chessman.is_in_palace(&to) {
                return false;
            }
            if !chessman.is_move_one_step(m) {
                return false;
            }
        }
        true
    }

    fn is_valid_coordinate(coordinate: &(i8, i8)) -> bool {
        coordinate.0 >= 0
            && coordinate.0 < Self::WIDTH as i8
            && coordinate.1 >= 0
            && coordinate.1 < Self::HEIGHT as i8
    }

    fn get_bypass_chessman_num(&self, m: &Move) -> usize {
        let mut res = 0;
        let i = m.from.0;
        for j in m.from.1 + 1..=m.to.1 {
            if self.get(j, i).is_some() {
                res += 1;
            }
        }
        let j = m.from.1;
        for i in m.from.0 + 1..=m.to.0 {
            if self.get(j, i).is_some() {
                res += 1;
            }
        }
        res
    }

    fn get_right_of_chessman(
        &self,
        coordinate: (usize, usize),
        chessman: &Chessman,
    ) -> Option<Chessman> {
        self.get_left_of_chessman(coordinate, &chessman.change_color())
    }
    fn get_left_of_chessman(
        &self,
        coordinate: (usize, usize),
        chessman: &Chessman,
    ) -> Option<Chessman> {
        let col = coordinate.0;
        if chessman.is_red() {
            if col == 0 {
                Some(Chessman::BOARD)
            } else {
                self.get_at((coordinate.0 - 1, coordinate.1))
            }
        } else {
            if col == Self::WIDTH - 1 {
                Some(Chessman::BOARD)
            } else {
                self.get_at((coordinate.0 + 1, coordinate.1))
            }
        }
    }
    fn get_back_of_chessman(
        &self,
        coordinate: (usize, usize),
        chessman: &Chessman,
    ) -> Option<Chessman> {
        self.get_front_of_chessman(coordinate, &chessman.change_color())
    }
    fn get_front_of_chessman(
        &self,
        coordinate: (usize, usize),
        chessman: &Chessman,
    ) -> Option<Chessman> {
        let line = coordinate.1;
        if chessman.is_red() {
            if line == 0 {
                Some(Chessman::BOARD)
            } else {
                self.get_at((coordinate.0, line - 1))
            }
        } else {
            if line == Self::HEIGHT - 1 {
                Some(Chessman::BOARD)
            } else {
                self.get_at((coordinate.0, line + 1))
            }
        }
    }

    pub fn get_mut_at(&mut self, coordinate: (usize, usize)) -> &mut Option<Chessman> {
        self.get_mut(coordinate.1, coordinate.0)
    }
    pub fn get_at(&self, coordinate: (usize, usize)) -> Option<Chessman> {
        self.get(coordinate.1, coordinate.0)
    }
    pub fn get(&self, line: usize, column: usize) -> Option<Chessman> {
        self.board[line][column]
    }
    pub fn get_mut(&mut self, line: usize, column: usize) -> &mut Option<Chessman> {
        &mut self.board[line][column]
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
