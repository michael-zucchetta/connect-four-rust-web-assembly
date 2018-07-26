extern crate ansi_term;

use self::ansi_term::Colour::{Red, Blue, White};
use std::ops::Range;
use std::prelude::v1::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ConnectFourMove {
    XPosition,
    XIgnoredPosition,
    OPosition,
    OIgnoredPosition,
    UnplayablePosition,
    EmptyIgnoredPosition,
    EmptyPosition,
}

impl ToString for ConnectFourMove {
    fn to_string(&self) -> String {
        match *self {
            ConnectFourMove::XPosition => Red.bold().paint("  X  ").to_string(),
            ConnectFourMove::XIgnoredPosition => Red.bold().paint(" .x. ").to_string(),
            ConnectFourMove::OPosition => Blue.bold().paint("  O  ").to_string(),
            ConnectFourMove::OIgnoredPosition => Blue.bold().paint(" .o. ").to_string(),
            ConnectFourMove::UnplayablePosition => White.bold().paint("#####").to_string(),
            ConnectFourMove::EmptyIgnoredPosition => " ... ".to_string(),
            ConnectFourMove::EmptyPosition => "     ".to_string(),
        }
    }
}

pub struct Position {
    column: u32,
    row: u32,
}

pub struct ConnectFourBoard {
    pub board: Vec<Vec<ConnectFourMove>>,
    pub width: usize,
    pub height: usize,
}

impl ConnectFourBoard {
    pub fn new(column_sizes: &Vec<u32>, ignored_positions: &Vec<Position>) -> ConnectFourBoard {
        let board_width = column_sizes.len();
        let highest_column = *column_sizes.iter().max().unwrap_or(&0u32);
        let board = column_sizes.iter().enumerate().map(|(x, xth_column_size)|
            (0..highest_column).map(move |y| { // move resolves the borrowing problem by moving the lifecycle of x and xth_column_size to this scope
                if (y as u32) < (highest_column - xth_column_size) { // position x is higher than column actual size
                    ConnectFourMove::UnplayablePosition
                } else {
                    ignored_positions.iter().find(|ignored_position| {
                        ignored_position.row == (highest_column - y as u32) && ignored_position.column == x as u32
                    })
                    .map(|_| ConnectFourMove::EmptyIgnoredPosition)
                    .unwrap_or(ConnectFourMove::EmptyPosition)
                }
            }).collect()
        ).collect();
        ConnectFourBoard {
            board: board,
            height: highest_column as usize,
            width: board_width,
        }
    }

    pub fn create_another(board: Vec<Vec<ConnectFourMove>>, height: usize, width: usize) -> ConnectFourBoard {
        ConnectFourBoard {
            board: board,
            height: height,
            width: width, 
        }
    }

    // to be changed to return Self
    pub fn make_move(&mut self, move_type: ConnectFourMove, move_column: usize) -> () {
		let column = &mut self.board[move_column];
        // println!("column {} =  {:?}", move_column, column);
        let empty_position = column.iter_mut().take_while(|position| {
            match *position {
                &mut ConnectFourMove::UnplayablePosition | 
                &mut ConnectFourMove::EmptyPosition | 
                &mut ConnectFourMove::EmptyIgnoredPosition => true,
                _ => false
            }
        }).last().unwrap();
        *empty_position = match *empty_position {
            ConnectFourMove::EmptyIgnoredPosition if move_type == ConnectFourMove::XPosition => ConnectFourMove::XIgnoredPosition,
            ConnectFourMove::EmptyIgnoredPosition if move_type == ConnectFourMove::OPosition => ConnectFourMove::OIgnoredPosition,
            ConnectFourMove::EmptyPosition => move_type,
            _ => panic!("Move insertion is broken {}", move_type.to_string()),
        };
    }

    pub fn empty_moves_by_column(&self) -> Vec<usize> {
        self.board.iter().map(|column| {
            column.iter().filter(|element| 
                element == &&ConnectFourMove::EmptyPosition || element == &&ConnectFourMove::EmptyIgnoredPosition
            ).count()
        }).collect::<Vec<_>>()
    }

    fn valid_position(&self, cell: ConnectFourMove) -> bool {
        match cell {
            ConnectFourMove::XPosition |
                ConnectFourMove::OPosition => {
                    true
                },
            _ => { 
                false
            },
        }
    }

    fn is_winning_position(&self, y: usize, x: usize) -> Option< Vec<(usize, usize)> > {
        let candidate = self.board[x][y];
        if !self.valid_position(candidate) {
            None
        } else {
            let upper_limit = 3;
            let mut horizontal_win = true; 
            let mut vertical_win = true; 
            let mut diagonal_up_win = true; 
            let mut diagonal_down_win = true; 
            for index in 1..4 {
				horizontal_win = horizontal_win && x + upper_limit < self.width && self.board[x + index][y] == candidate;
                vertical_win = vertical_win && y + upper_limit < self.height && candidate == self.board[x][y + index];
                diagonal_up_win = diagonal_up_win && (y + upper_limit) < self.height && (x + upper_limit) < self.width && candidate == self.board[x + index][y + index];
                diagonal_down_win = diagonal_down_win && (y as i32 - index as i32) >= 0 && x + index < self.width && candidate == self.board[x + index][y - index];
            }
            // println!("{}, {}, {}, {}, {}, {}, {:?}", x, y, horizontal_win, vertical_win, diagonal_up_win, diagonal_down_win, candidate);
            match (horizontal_win, vertical_win, diagonal_up_win, diagonal_down_win) {
                (true, _, _, _) => Some(vec![(y, x), (y, x + 1), (y, x + 2), (y, x + 3)]),
                (_, true, _, _) => Some(vec![(y, x), (y + 1, x), (y + 2, x), (y + 3, x)]),
                (_, _, true, _) => Some(vec![(y, x), (y + 1, x + 1), (y + 2, x + 2), (y + 3, x + 3)]),
                (_, _, _, true) => Some(vec![(y, x), (y - 1, x + 1), (y - 2, x + 2), (y - 3, x + 3)]),
                _ => None,
            }
        }
    }

    pub fn return_winner(&self, y: usize, x: usize) -> Option< (ConnectFourMove, Vec<(usize, usize)>) > {
        let winning_move_and_sequence_opt = self.is_winning_position(y, x);
        if winning_move_and_sequence_opt.is_some() {
            Some( ( self.board[x][y], winning_move_and_sequence_opt.unwrap()) )
        } else if x == self.width - 1 &&  y == self.height - 1 {
            None
        } else {
            let (new_y, new_x) = if x < self.width - 1 {
                (y, x + 1)
            } else {
                (y + 1, 0)
            };
            self.return_winner(new_y, new_x)
        }
    }
    
    fn winning_move(&self, index: usize, player_move: ConnectFourMove, free_moves: &Vec<usize>) -> Option<usize> {
        let mut cloned = self.clone();
        //println!("FAOFAFA {:?} {}", free_moves, free_moves[index]);
        cloned.make_move(player_move, free_moves[index]);
        match cloned.return_winner(0, 0) {
            Some(_) => {
                // println!("GOING THROUGH {}", free_moves[index]);
                Some(free_moves[index])
            },
            _ if index == 0 => None,
            _ => self.winning_move(index - 1, player_move, &free_moves)
        }
    }

    pub fn next_winning_move(&self, player: Player) -> Option<usize> {
        let player_move = player_move(player);
        let free_moves = self.free_moves(); 
        
        self.winning_move((free_moves.len() as u32 - 1u32) as usize, player_move, &free_moves)
    }

    pub fn free_moves(&self) -> Vec<usize> {
       self.empty_moves_by_column()
            .iter()
            .enumerate()
            .filter(|(index, amount_left)| **amount_left != 0usize)
            .map(|(index, _)| index)
            .collect::<Vec<_>>() 
    }

    pub fn moves_left(&self) -> usize {
        self.empty_moves_by_column().iter().sum()
    }
}

static ASCII_UPPERCASE: &'static [char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
                                        'J', 'K', 'L', 'M', 'N', 'O', 'p', 'Q', 'R',
                                        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
impl ToString for ConnectFourBoard {
    fn to_string(&self) -> String {
        let cols_size = self.board.len();
        let header: String = Range { start: 0, end: cols_size }
                        .map(|_| { "_____" })
                        .collect::<Vec<_>>()
                        .join(" ");
        let body: String = (0..self.height).map(|y| {
            let mut base = (y + 1).to_string();
            base.push_str("\t|");
            let line = (0..self.width).map(|x|
                self.board[x as usize][y as usize].to_string()
            ).collect::<Vec<_>>().join("|");
            base.push_str(&line);
            base.push_str("|");
            base
        }).collect::<Vec<_>>().join("\n");
        let footer = Range { start: 0, end: cols_size }
            .map(|_| { "_____" })
            .collect::<Vec<_>>()
            .join(" ");
        let footer_commands = Range { start: 0, end: cols_size }
            .map(|index| {
                let mut cell_str: String = String::from("  ");
                cell_str.push(ASCII_UPPERCASE[index]);
                cell_str.push_str("  ");
                cell_str
            })
            .collect::<Vec<_>>()
            .join(" ");
        let mut concatenation: String = "\t ".into();
        concatenation.push_str(&header);
        concatenation.push_str("\n");
        concatenation.push_str(&body);
        concatenation.push_str("\n\t ");
        concatenation.push_str(&footer);
        concatenation.push_str("\n\t|");
        concatenation.push_str(&footer_commands);
        concatenation.push_str("|");
        concatenation
    }
}

impl Clone for ConnectFourBoard {
    fn clone(&self) -> ConnectFourBoard {
        let board = self.board.clone();
        ConnectFourBoard::create_another(board, self.height, self.width)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Player {
    Player1,
    Player2,
    AIPlayer1,
    AIPlayer2,
}

pub fn player_move(player: Player) -> ConnectFourMove {
    match player {
        Player::Player1 => ConnectFourMove::XPosition,
        Player::Player2 => ConnectFourMove::OPosition,
        Player::AIPlayer1 => ConnectFourMove::XPosition,
        Player::AIPlayer2 => ConnectFourMove::OPosition,
    }
}

#[cfg(test)]
mod tests {
    use models::*;
    #[test]
    fn check_winning_position() {
        let mut board = ConnectFourBoard::new(&vec![6u32, 6u32, 6u32, 6u32, 6u32], &vec![]);
        assert_eq!(board.is_winning_position(0usize, 0usize), false);
        board.make_move(ConnectFourMove::OPosition, 1);
        board.make_move(ConnectFourMove::OPosition, 2);
        board.make_move(ConnectFourMove::OPosition, 3);
        board.make_move(ConnectFourMove::OPosition, 4);
        print!("board status:\n {}", board.to_string());
        assert_eq!(board.is_winning_position(5usize, 1usize), true);
    }
}
