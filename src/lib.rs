#![feature(proc_macro)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate stdweb;

extern crate rand;

#[macro_use]
pub mod models;
pub mod montecarlo;
pub mod constants;
pub mod drawer;
pub mod game_modes;
pub mod game;
pub mod web;

use models::Position;
use stdweb::js_export;

#[js_export]
fn create_game(
      column_sizes: Vec<u32>,
      game_mode: String, 
    ) -> () {
    let ignored_positions: Vec<Position> = Vec::new();
    let ai_level = game_modes::ai_level_from_str(&*game_mode).unwrap();
    web::WebConnectFourGame::new(&column_sizes, &ignored_positions, game_modes::Modalities::HumanVsComputer, ai_level, None, "test");
}

#[no_mangle]
pub fn hello2() -> () {
  println!("hello2");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
