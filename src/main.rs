use connect_four::console;
use connect_four::game::ConnectFourGame;
use connect_four::game_modes;
use connect_four::models;

fn main() {
    let ignored_positions: Vec<models::Position> = Vec::new();
    let game_mode = "Hard";
    let ai_level = game_modes::ai_level_from_str(&*game_mode).unwrap();
    let column_sizes: Vec<u32> = vec![7u32, 7];
    let empty_vals: Vec<usize> = vec![0usize, 0];
    let mut game = console::ConsoleConnectFourGame::new(
        &column_sizes,
        &ignored_positions,
        game_modes::Modalities::HumanVsComputer,
        ai_level,
        None,
    );
    game.play_game();
}
