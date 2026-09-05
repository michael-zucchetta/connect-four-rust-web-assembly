// #![feature(proc_macro)]
// #![feature(use_extern_macros)]

extern crate rand;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::wasm_bindgen;

#[macro_use]
pub mod models;
pub mod console;
pub mod constants;
pub mod drawer;
pub mod game;
pub mod game_modes;
pub mod montecarlo;
pub mod utils;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod web;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn create_game(column_sizes: Vec<u32>, game_mode: String) -> () {
    console_error_panic_hook::set_once();
    let ignored_positions: Vec<models::Position> = Vec::new();
    let ai_level = game_modes::ai_level_from_str(&*game_mode).unwrap();
    let player_starts = js_sys::Math::random() < 0.5f64;
    web::WebConnectFourGame::new(
        &column_sizes,
        &ignored_positions,
        game_modes::Modalities::HumanVsComputer,
        ai_level,
        None,
        player_starts,
    );
}
