extern crate stdweb;

use constants;
use game;
use game_modes;
use models;

use game::ConnectFourGame;
use models::{ConnectFourBoard, Position};
use drawer::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    Element,
    CanvasRenderingContext2d,
};
use std::char;
use self::stdweb::web::event::{
    MouseDownEvent,
    KeyUpEvent,
};
use stdweb::web::html_element::CanvasElement;

pub fn append_div() -> Element {
    // stdweb::initialize();
    // first two unwraps: Result -> Option | try_into htmlelement 
    let body: Element = document().query_selector( "body" ).unwrap().unwrap();
    let div = document().create_element("div").unwrap();
    body.append_child(&div);
    return div;
}
/*
 * Very, very bad. Don't do this at home.
 * Its use will (maybe) be replaced by a worker with Yew
 */
static mut IS_GAME_ACTIVE: &bool = &true;

fn create_canvas(container_id: &str, column_sizes: &Vec<u32>) -> CanvasElement { 
    let div_container = append_div();
    let canvas_html_element = document().create_element("canvas").unwrap();
    div_container.append_child(&canvas_html_element);
    let canvas: CanvasElement = canvas_html_element.try_into().unwrap();
    // canvas.set_width(1024u32);
    // canvas.set_height(768u32);

    canvas.set_width( ( (constants::PADDING * 2f64) as usize + constants::CELL_WIDTH * column_sizes.len()) as u32);
    canvas.set_height( (constants::PADDING * 2f64) as u32 + constants::CELL_HEIGHT as u32 * ( column_sizes.iter().max().unwrap() + 1) );
    js!{
        let div_container = @{div_container};
        div_container.id = "canvas-container";
        //let canvas = @{canvas};
        div_container.tabIndex = 1;
        div_container.focus();
    };
    // stdweb::event_loop();
    canvas
}

#[derive(Clone)]
pub struct WebConnectFourGame {
    game_board: models::ConnectFourBoard,
    game_mode: game_modes::Modalities,
    level_ai2: game_modes::AILevel,
    level_ai1_opt: Option<game_modes::AILevel>,
    canvas: CanvasElement,
}

impl WebConnectFourGame {
    fn execute_human_move(&mut self, chosen_move: usize, _moves_left: Vec<usize>, game_turn: models::Player) -> () {
            let game_mode = *self.game_mode();
            self.game_board().make_move(models::player_move(game_turn), chosen_move); 
            self.draw();
            let winner_opt = self.game_board.return_winner(0, 0);
            if !winner_opt.is_some() {
                unsafe {
                    IS_GAME_ACTIVE = &false;
                }
                self.make_next_move(game_modes::get_opposite_from_turn(game_turn, game_mode));
            } else {
                let (_, winning_sequence) = winner_opt.unwrap();
                self.draw_endgame(game_turn, winning_sequence);
            }
    }

    fn get_column_from_key_event(&self, event: KeyUpEvent, _moves_left: Vec<usize>, column_sizes: usize) -> Option<usize> {
        let max_char = char::from_u32('A' as u32 + (column_sizes - 1) as u32).unwrap();
        if event.key().len() == 1 {
            let event_key_opt = event.key().chars().nth(0);
            match event_key_opt {
                None => None, 
                Some(value) if value >= 'A' && value <= max_char ||
                    value >= 'a' && value <= max_char.to_ascii_lowercase()
                    => {
                        Some(value.to_ascii_lowercase() as usize - 'a' as usize)
                    }
                Some(_) => None,
            }
        } else {
            None
        }
    }

    fn get_column_from_coordinates(&self, x: f64, column_sizes: usize)-> Option<usize> {
        let column = ( (x - constants::PADDING) as usize ) / constants::CELL_WIDTH; // - 1usize
        if column < column_sizes {
            Some(column)
        } else {
            None
        }
    }

    pub fn new(
        column_sizes: &Vec<u32>,
        ignored_positions: &Vec<Position>,
        game_mode: game_modes::Modalities,
        level_ai2: game_modes::AILevel,
        level_ai1_opt: Option<game_modes::AILevel>,
        canvas_id: &str,
        ) -> Self {
        let game_board = ConnectFourBoard::new(column_sizes, ignored_positions);
        let canvas = create_canvas(canvas_id, column_sizes);

        let mut web_connected_four_game = WebConnectFourGame {
            game_board: game_board,
            game_mode: game_mode,
            level_ai2: level_ai2,
            level_ai1_opt: level_ai1_opt,
            canvas: canvas,
        };

        web_connected_four_game.draw();
        
        let game = web_connected_four_game.clone();
        let mut game_cloned = web_connected_four_game.clone();
        let div_container = web_connected_four_game.canvas.parent_node().unwrap();
        /*
        div_container.add_event_listener( move |event: KeyUpEvent| {
            unsafe {
                if *IS_GAME_ACTIVE {
                    let player = if web_connected_four_game.level_ai1_opt.is_some() {
                        models::Player::Player2
                    } else {
                        models::Player::Player1
                    };
                    let game_board = web_connected_four_game.game_board().clone();
                    let column_sizes = game_board.width;
                    let free_moves = game_board.free_moves();
                    let column_chosen_opt = web_connected_four_game.get_column_from_key_event(event, free_moves.clone(), column_sizes);
                    if column_chosen_opt.is_some() {
                        web_connected_four_game.execute_human_move(column_chosen_opt.unwrap(), free_moves, player);
                    }
                }
            }
        });
        */
        div_container.add_event_listener( move |event: MouseDownEvent| {
            unsafe {
                let click_x = event.offset_x();
                if *IS_GAME_ACTIVE {
                    let player = if game_cloned.level_ai1_opt.is_some() {
                        models::Player::Player2
                    } else {
                        models::Player::Player1
                    };
                    let free_moves = game_cloned.game_board().clone().free_moves();
                    let column_sizes = game_cloned.game_board().clone().width;
                    let position_opt = game_cloned.get_column_from_coordinates(click_x, column_sizes);
                    if position_opt.is_some() {
                        game_cloned.execute_human_move(position_opt.unwrap(), free_moves, player);
                    }
                }
            }
        });
        // stdweb::event_loop();
        game
        // web_connected_four_game
    }
    
}


impl game::ConnectFourGame for WebConnectFourGame {
    fn game_board(&mut self) -> &mut models::ConnectFourBoard {
        &mut self.game_board//.clone() // very wrong
    }

    fn game_mode(&self) -> &game_modes::Modalities {
        &self.game_mode
    }

    fn level_ai1(&self) -> game_modes::AILevel {
        self.level_ai1_opt.unwrap()
    }

    fn level_ai2(&self) -> game_modes::AILevel {
        self.level_ai2
    }

    fn draw(&mut self) -> () {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        let canvas_context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
        // self.game_board().make_move(ConnectFourMove::OPosition, 0); 
        self.game_board().draw(canvas_context, width, height); 
    }

    fn draw_endgame(&mut self, player: models::Player, winning_sequence: Vec<(usize, usize)>) -> () {
        let canvas_context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
        self.game_board().draw_endgame(canvas_context, player, winning_sequence);
        js!{ 
            setTimeout(function() {
                alert("GAME FINISHED");
            }, 500); 
        }
    }

    fn next_human_move(&self, _moves_left: Vec<usize>, _game_turn: models::Player) -> usize {
        unsafe {
            IS_GAME_ACTIVE = &true; // TEMPORARY
        }
        // let (mut tx, rx): (Sender<usize>, Receiver<usize>) = channel(100usize);
        // let mut ref_game = self.clone();
        /*let actual_listener = ref_game.canvas.parent_node().unwrap().add_event_listener( move |event: KeyUpEvent| {
            ref_game.execute_human_move(event, moves_left.clone(), game_turn.clone());
        });*/
        self.canvas.add_event_listener( move |_event: MouseDownEvent| {
            println!("!! CIAO");
            
            // tx.try_send(5usize).unwrap();
        });
        // }; 
        
        /*thread::spawn(move || {
            println!("CIAO");
            // thread::sleep(5usize);
            tx.try_send(5usize).unwrap();
        }).join().unwrap();
        */
       // thread::current().join().unwrap();
       0usize
    }
}
