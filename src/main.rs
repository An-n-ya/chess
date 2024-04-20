mod chessman;
mod constants;
mod input;
mod layout;

use input::Input;
use layout::Layout;
struct Chess {
    round: usize,
    peace_round: usize,
    turn: Turn,
    layout: Layout,
    input: Input,
}

enum Turn {
    Red,
    Black,
}

impl Chess {
    pub fn new() -> Self {
        let mut chess = Self {
            round: 0,
            peace_round: 0,
            turn: Turn::Red,
            layout: Layout::new(),
            input: Input::new(),
        };
        chess.parse_fen("rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1");
        chess
    }

    pub fn parse_fen(&mut self, input: &str) {
        let mut line_cnt = 0;
        let lines = input.split('/');
        let last_line = lines.clone().last().unwrap();
        for line in lines {
            let mut column_cnt = 0;
            for c in line.chars() {
                if c.is_numeric() {
                    let n = c as u8 - 0x30;
                    for _ in 0..n {
                        self.layout.board[line_cnt][column_cnt] = None;
                        column_cnt += 1;
                    }
                } else {
                    self.layout.board[line_cnt][column_cnt] = Some(c.into());
                    column_cnt += 1;
                }
                if column_cnt == 9 {
                    break;
                }
            }
            line_cnt += 1;
        }

        let commands: Vec<&str> = last_line.split(' ').into_iter().skip(1).collect();

        if commands[0] == "w" || commands[0] == "r" {
            self.turn = Turn::Red;
        } else if commands[0] == "b" {
            self.turn = Turn::Black;
        }

        self.peace_round = usize::from_str_radix(commands[3], 10).unwrap();
        self.round = usize::from_str_radix(commands[4], 10).unwrap();
    }

    pub fn render(&self) {
        println!("{}", self.layout);
    }

    pub fn run(&mut self) {
        loop {
            let m = self.input.get(&self.layout);
            self.layout.handle_move(&m);
            self.render();
        }
    }
}
fn main() {
    let mut chess = Chess::new();
    chess.render();
    chess.run();
}
