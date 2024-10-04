use crate::drawer::*;
use crate::game;
use crate::game_modes;
use crate::models;
use crate::models::{ConnectFourBoard, Position};
use crate::utils;
use std::io;
use std::{thread, time};
#[derive(Clone)]
pub struct ConsoleConnectFourGame {
    game_board: models::ConnectFourBoard,
    game_mode: game_modes::Modalities,
    level_ai2: game_modes::AILevel,
    level_ai1_opt: Option<game_modes::AILevel>,
}

impl ConsoleConnectFourGame {
    pub fn new(
        column_sizes: &Vec<u32>,
        ignored_positions: &Vec<Position>,
        // game_board: models::ConnectFourBoard,
        game_mode: game_modes::Modalities,
        level_ai2: game_modes::AILevel,
        level_ai1_opt: Option<game_modes::AILevel>,
    ) -> Self {
        let board = ConnectFourBoard::new(column_sizes, &ignored_positions);
        let console_four_game = ConsoleConnectFourGame {
            game_board: board,
            game_mode: game_mode,
            level_ai2: level_ai2,
            level_ai1_opt: level_ai1_opt,
        };
        console_four_game.game_board.draw((), 0f64, 0f64);
        console_four_game
    }
}

impl game::ConnectFourGame for ConsoleConnectFourGame {
    fn game_board(&mut self) -> &mut models::ConnectFourBoard {
        &mut self.game_board //.clone() // very wrong
    }

    fn game_mode(&self) -> &game_modes::Modalities {
        &self.game_mode
    }

    fn draw(&mut self) -> () {
        self.game_board().draw((), 0f64, 0f64);
    }

    fn execute_human_move(
        &mut self,
        _chosen_move: usize,
        moves_left: Vec<usize>,
        game_turn: models::Player,
    ) -> () {
        let mut command_as_text = String::new();
        println!("Selecting move");
        io::stdin()
            .read_line(&mut command_as_text)
            .expect("failed to read from stdin");
        println!("Selecting move");
        match utils::parse_and_return_value(&command_as_text, moves_left.len()) {
            Some(chosen_move) => {
                let game_mode = *self.game_mode();
                if self
                    .game_board()
                    .make_move(models::player_move(game_turn.clone()), chosen_move)
                {
                    self.draw();
                    thread::sleep(time::Duration::from_millis(100));
                    self.execute_human_move(
                        0usize,
                        moves_left,
                        game_modes::get_opposite_from_turn(game_turn, game_mode),
                    );
                } else {
                    self.execute_human_move(0usize, moves_left, game_turn);
                }
            }
            _ => {
                println!("Key {} not valid", &command_as_text);
                self.execute_human_move(0usize, moves_left, game_turn);
            }
        };
    }

    fn level_ai1(&self) -> game_modes::AILevel {
        self.level_ai1_opt.unwrap()
    }

    fn level_ai2(&self) -> game_modes::AILevel {
        self.level_ai2
    }

    fn draw_endgame(
        &mut self,
        _player: models::Player,
        _winning_sequence: Vec<(usize, usize)>,
    ) -> () {
    }
}
