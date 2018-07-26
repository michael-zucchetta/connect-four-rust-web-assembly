use models;
use game_modes;
use montecarlo;

pub struct Game {
    pub game_board: models::ConnectFourBoard,
    pub game_mode: game_modes::Modalities, 
    pub level_ai2: game_modes::AILevel,
    pub level_ai1_opt: Option<game_modes::AILevel>,
}

pub trait ConnectFourGame {
    fn game_board(&mut self) -> &mut models::ConnectFourBoard;

    fn game_mode(&self) -> &game_modes::Modalities;

    fn level_ai1(&self) -> game_modes::AILevel;
    
    fn level_ai2(&self) -> game_modes::AILevel;
    
    fn apply_ai_moves_sequence(&self, previous_board: models::ConnectFourBoard, mut moves_sequence: Iter<usize>, sequence_index: usize, ai_turn_type: models::Player) -> models::ConnectFourBoard {
      let next_move_sequence_opt = moves_sequence.next(); 
      if previous_board.return_winner(0, 0).is_some() || next_move_sequence_opt.is_none() {
         previous_board
       } else {
         let move_type = if sequence_index % 2 == 0 {
             models::player_move(ai_turn_type)
         } else {
             let opposite_turn = game_modes::get_opposite_from_turn(ai_turn_type, *self.game_mode());
             models::player_move(opposite_turn)
         };
         // previous_board // temp
         let next_move = if sequence_index % 2 == 0 {
            previous_board
                .next_winning_move(ai_turn_type).unwrap_or(
                     // to improve
                     *next_move_sequence_opt.unwrap()
                )
         } else {
             // to be improved
             *next_move_sequence_opt.unwrap()
         };
         let mut updated_board = previous_board;//.clone();
         updated_board.make_move(move_type, next_move);
         self.apply_ai_moves_sequence(updated_board, moves_sequence, sequence_index + 1, ai_turn_type) 
       }
    }

    fn next_ai_move(&mut self, ai_turn_type: models::Player) -> usize {
        let level_ai = match ai_turn_type {
            models::Player::AIPlayer1 => self.level_ai1(),
            models::Player::AIPlayer2 => self.level_ai2(),
            _ => panic!("This should not happen"), 
        };
        let empty_moves_by_column = self.game_board().empty_moves_by_column();
        let columns_size = empty_moves_by_column.len();

        let montecarlo_sequences = montecarlo::generate_n_moves_sequences(empty_moves_by_column,
            self.game_board().moves_left(),
            self.get_montecarlo_sequences_size(&level_ai),
        );
        let simulation_results = montecarlo_sequences.iter().map(|moves_sequence| {
            let game_board = self.game_board().clone();
            let first_element = moves_sequence.first();
            let resulting_board = self.apply_ai_moves_sequence(game_board, moves_sequence.iter(), 0, ai_turn_type);
            let winner_opt = resulting_board.return_winner(0, 0);
            if winner_opt.is_some() && first_element.is_some() {
                Some((winner_opt.unwrap(), *first_element.unwrap()))
            } else {
                None
            }
        });
        /*
        let games_with_winner = simulation_results
            .filter(|simulation_result| simulation_result.is_some())
            .map(|simulation_result| {
                js! { console.log("BREAKING THE LAW") }
                simulation_result.unwrap()
            });*/
        let mut winning_moves_count = vec![0u32; columns_size]; 
        simulation_results.for_each(|simulation| {
            if simulation.is_some() {
                let ((winner, _), initial_move_in_simulation) = simulation.unwrap();
                if winner == models::player_move(ai_turn_type) {
                    winning_moves_count[initial_move_in_simulation] += 1u32;
                }
            }
        });
        let move_with_most_wins = winning_moves_count
            .iter()
            .enumerate()
            .map(|(index, elem)| (elem, index))
            .max()
            .map(|(elem, index)| index)
            .unwrap_or(0usize);
        move_with_most_wins
    }
   
    fn get_montecarlo_sequences_size(&self, level_ai: &game_modes::AILevel) -> usize {
        game_modes::ai_moves(*level_ai)
    }
    
    fn get_move_based_on_turn(&mut self, game_turn: models::Player) -> () {
        let game_mode = *self.game_mode();
        match game_turn {
            models::Player::Player1 | models::Player::Player2 => {
                let board = self.game_board().clone();
                self.next_human_move(board.empty_moves_by_column(), game_turn);
                println!("ARRIVED");
            },
            models::Player::AIPlayer1 | models::Player::AIPlayer2 => {
                let ai_move = self.game_board().next_winning_move(game_turn)
                    .unwrap_or(
                        self.game_board().next_winning_move(game_modes::get_opposite_from_turn(game_turn, game_mode))
                            .unwrap_or(self.next_ai_move(game_turn))
                    );
                self.game_board().make_move(models::player_move(game_turn.clone()), ai_move);
                self.draw();
                self.make_next_move(game_modes::get_opposite_from_turn(game_turn, game_mode));
            }
        }
    }

    fn make_next_move(&mut self, game_turn: models::Player) {
        let winner_opt = self.game_board().return_winner(0, 0);
        if winner_opt.is_some() || self.game_board().moves_left() == 0 {
            let (_, winning_sequence) = winner_opt.unwrap();
            self.draw_endgame(game_turn, winning_sequence); 
        } else {
            self.get_move_based_on_turn(game_turn.clone());
        }
    }

    fn play_game(&mut self) {
        let first_turn = match *self.game_mode() {
            game_modes::Modalities::HumanVsComputer => models::Player::Player1,
            game_modes::Modalities::ComputerVsComputer => models::Player::AIPlayer1,
            game_modes::Modalities::ComputerVsHuman => models::Player::AIPlayer2,
        };
        self.make_next_move(first_turn);
    }

    fn next_human_move(&self, moves_left: Vec<usize>, game_turn: models::Player) -> usize; 

    fn draw(&mut self) -> ();
    
    fn draw_endgame(&mut self, player: models::Player, winning_sequence: Vec<(usize, usize)>) -> ();
}
