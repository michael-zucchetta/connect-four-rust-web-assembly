use crate::constants;
use crate::game;
use crate::game_modes;
use crate::models;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure};

use crate::drawer::*;
use crate::game::ConnectFourGame;
use crate::models::{ConnectFourBoard, Position};

use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, MouseEvent, Window,
};

pub fn get_document() -> Document {
    let window: Window = web_sys::window().expect("should have a Window object");
    let document: Document = window.document().expect("should have a Document object");
    return document;
}

pub fn append_div() -> HtmlElement {
    let document: Document = get_document();
    let container: Element = document
        .query_selector("#game-area")
        .unwrap()
        .or_else(|| document.query_selector("body").unwrap())
        .unwrap();
    let div: HtmlElement = document
        .create_element("div")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_id("canvas-container");
    div.set_tab_index(1);
    container.append_child(&div).unwrap();
    div.focus().unwrap();
    return div;
}

fn player_winner_text(player: models::Player) -> &'static str {
    match player {
        models::Player::Player1 => "Player X wins",
        models::Player::Player2 => "Player O wins",
        models::Player::AIPlayer1 => "AI X wins",
        models::Player::AIPlayer2 => "AI O wins",
    }
}

fn set_game_status(message: &str) -> () {
    if let Ok(Some(status_element)) = get_document().query_selector("#game-status") {
        status_element.set_text_content(Some(message));
    }
    if let Ok(Some(status_element)) = get_document().query_selector("#canvas-status") {
        status_element.set_text_content(Some(message));
    }
}

fn set_coin_status(player_starts: bool) -> () {
    let message = if player_starts {
        "Player starts"
    } else {
        "AI starts"
    };
    if let Ok(Some(status_element)) = get_document().query_selector("#coin-result") {
        status_element.set_text_content(Some(message));
    }
}

fn ensure_fireworks_styles() -> () {
    let document = get_document();
    if document
        .query_selector("#connect-four-fireworks-styles")
        .unwrap()
        .is_some()
    {
        return;
    }

    let style = document.create_element("style").unwrap();
    style.set_id("connect-four-fireworks-styles");
    style.set_text_content(Some(
        r#"
            #canvas-container {
                position: relative;
            }

            #coin-toss {
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 10px;
                min-height: 46px;
                margin: 0 0 8px;
                color: #8a8f8f;
                font: 700 1rem ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
            }

            #coin-toss .coin {
                width: 38px;
                height: 38px;
                border-radius: 999px;
                background: #ff5f56;
                border: 1px solid #f5f7f7;
                box-shadow: 0 0 10px rgba(255, 95, 86, 0.32);
                animation: connect-four-coin-spin 900ms cubic-bezier(.2, .7, .25, 1.05) 1;
            }

            #coin-toss .coin.ai-starts {
                background: #ffd866;
                animation-name: connect-four-coin-spin-ai;
            }

            #canvas-status {
                min-height: 24px;
                margin: 0 0 8px;
                color: #1cba22;
                font: 700 0.95rem ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
                text-align: center;
            }

            .fireworks-layer {
                position: absolute;
                inset: 32px 0 0;
                pointer-events: none;
                overflow: hidden;
                z-index: 2;
            }

            .firework-burst {
                position: absolute;
                width: 8px;
                height: 8px;
            }

            .firework-burst-1 {
                left: 24%;
                top: 24%;
            }

            .firework-burst-2 {
                left: 52%;
                top: 18%;
            }

            .firework-burst-3 {
                left: 76%;
                top: 30%;
            }

            .firework-burst span {
                position: absolute;
                left: 0;
                top: 0;
                width: 6px;
                height: 6px;
                border-radius: 999px;
                background: #1cba22;
                transform: rotate(var(--angle)) translateX(0);
                opacity: 0;
                animation: connect-four-spark-pop 1600ms ease-out infinite;
            }

            .firework-burst span:nth-child(3n + 1) {
                background: #0ff;
            }

            .firework-burst span:nth-child(3n + 2) {
                background: #d8a100;
            }

            @keyframes connect-four-spark-pop {
                0% {
                    opacity: 0;
                    transform: rotate(var(--angle)) translateX(0) scale(0.4);
                }

                16% {
                    opacity: 0.95;
                }

                100% {
                    opacity: 0.18;
                    transform: rotate(var(--angle)) translateX(var(--distance)) scale(0.8);
                }
            }

            @keyframes connect-four-coin-spin {
                0% {
                    background: #ff5f56;
                    transform: scaleX(1) scale(0.86);
                }

                24% {
                    background: #ffd866;
                    transform: scaleX(0.12) scale(1);
                }

                48% {
                    background: #ff5f56;
                    transform: scaleX(1) scale(1.05);
                }

                70% {
                    background: #ffd866;
                    transform: scaleX(0.12) scale(1.08);
                }

                100% {
                    background: #ff5f56;
                    transform: scaleX(1) scale(1);
                }
            }

            @keyframes connect-four-coin-spin-ai {
                0% {
                    background: #ffd866;
                    transform: scaleX(1) scale(0.86);
                }

                24% {
                    background: #ff5f56;
                    transform: scaleX(0.12) scale(1);
                }

                48% {
                    background: #ffd866;
                    transform: scaleX(1) scale(1.05);
                }

                70% {
                    background: #ff5f56;
                    transform: scaleX(0.12) scale(1.08);
                }

                100% {
                    background: #ffd866;
                    transform: scaleX(1) scale(1);
                }
            }
        "#,
    ));

    let style_container = document
        .query_selector("head")
        .unwrap()
        .or_else(|| document.query_selector("body").unwrap())
        .unwrap();
    style_container.append_child(&style).unwrap();
}

fn launch_fireworks() -> () {
    ensure_fireworks_styles();

    let document = get_document();
    let Ok(Some(canvas_container)) = document.query_selector("#canvas-container") else {
        return;
    };

    if let Ok(Some(existing_layer)) = canvas_container.query_selector(".fireworks-layer") {
        existing_layer.remove();
    }

    let layer = document.create_element("div").unwrap();
    layer.set_attribute("class", "fireworks-layer").unwrap();

    for burst in 0..3 {
        let burst_element = document.create_element("div").unwrap();
        burst_element
            .set_attribute(
                "class",
                &format!("firework-burst firework-burst-{}", burst + 1),
            )
            .unwrap();

        for spark in 0..12 {
            let spark_element = document.create_element("span").unwrap();
            spark_element
                .set_attribute(
                    "style",
                    &format!(
                        "--angle: {}deg; --distance: {}px; animation-delay: {}ms;",
                        spark * 30,
                        34 + burst * 8,
                        burst * 160
                    ),
                )
                .unwrap();
            burst_element.append_child(&spark_element).unwrap();
        }

        layer.append_child(&burst_element).unwrap();
    }

    canvas_container.append_child(&layer).unwrap();
}

fn append_coin_toss(container: &HtmlElement, player_starts: bool) -> () {
    ensure_fireworks_styles();

    let document = get_document();
    let coin_toss = document.create_element("div").unwrap();
    coin_toss.set_id("coin-toss");

    let coin = document.create_element("span").unwrap();
    let coin_class = if player_starts {
        "coin player-starts"
    } else {
        "coin ai-starts"
    };
    coin.set_attribute("class", coin_class).unwrap();
    coin_toss.append_child(&coin).unwrap();

    let result = document.create_element("span").unwrap();
    result.set_id("coin-result");
    coin_toss.append_child(&result).unwrap();

    container.append_child(&coin_toss).unwrap();
}
/*
 * Very, very bad. Don't do this at home.
 * Its use will (maybe) be replaced by a worker with Yew
 */
static mut IS_GAME_ACTIVE: bool = true;

fn create_canvas(
    container_id: &str,
    column_sizes: &Vec<u32>,
    player_starts: bool,
) -> HtmlCanvasElement {
    let div_container = append_div();
    let document = get_document();
    append_coin_toss(&div_container, player_starts);

    let status_element = document.create_element("div").unwrap();
    status_element.set_id("canvas-status");
    div_container.append_child(&status_element).unwrap();

    let canvas_html_element = document.create_element("canvas").unwrap();
    canvas_html_element.set_id(container_id);
    div_container.append_child(&canvas_html_element).unwrap();
    let canvas: HtmlCanvasElement = canvas_html_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    // canvas.set_width(1024u32);
    // canvas.set_height(768u32);

    canvas.set_width(
        ((constants::PADDING * 2f64) as usize + constants::CELL_WIDTH * column_sizes.len()) as u32,
    );
    canvas.set_height(
        (constants::PADDING * 2f64) as u32
            + constants::CELL_HEIGHT as u32 * (column_sizes.iter().max().unwrap() + 1),
    );
    canvas
}

#[derive(Clone)]
pub struct WebConnectFourGame {
    game_board: models::ConnectFourBoard,
    game_mode: game_modes::Modalities,
    level_ai2: game_modes::AILevel,
    level_ai1_opt: Option<game_modes::AILevel>,
    canvas: HtmlCanvasElement,
}

impl WebConnectFourGame {
    fn get_column_from_coordinates(&self, x: f64, column_sizes: usize) -> Option<usize> {
        let column = ((x - constants::PADDING) as usize) / constants::CELL_WIDTH; // - 1usize
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
        player_starts: bool,
    ) -> Self {
        unsafe {
            IS_GAME_ACTIVE = player_starts;
        }

        let game_board = ConnectFourBoard::new(column_sizes, ignored_positions);
        let canvas = create_canvas("connect-four-canvas", column_sizes, player_starts);

        let mut web_connected_four_game = WebConnectFourGame {
            game_board: game_board,
            game_mode: game_mode,
            level_ai2: level_ai2,
            level_ai1_opt: level_ai1_opt,
            canvas: canvas,
        };

        web_connected_four_game.draw();
        set_coin_status(player_starts);

        let game_state = Rc::new(RefCell::new(web_connected_four_game));
        let event_target = game_state.borrow().canvas.parent_element().unwrap();
        let click_game_state = game_state.clone();
        let mouse_down = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| unsafe {
            let click_x = event.offset_x() as f64;
            if IS_GAME_ACTIVE {
                let mut game_cloned = click_game_state.borrow_mut();
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
        });
        event_target
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())
            .unwrap();
        mouse_down.forget();

        if !player_starts {
            let initial_game_state = game_state.clone();
            let initial_ai_move = Closure::<dyn FnMut()>::new(move || {
                initial_game_state.borrow_mut().execute_initial_ai_move();
            });
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    initial_ai_move.as_ref().unchecked_ref(),
                    950,
                )
                .unwrap();
            initial_ai_move.forget();
        }

        game_state.borrow().clone()
    }

    fn execute_initial_ai_move(&mut self) -> () {
        let game_mode = self.game_mode;
        let ai_turn = models::Player::AIPlayer2;
        let ai_move = self.game_board().next_winning_move(ai_turn).unwrap_or(
            self.game_board()
                .next_winning_move(game_modes::get_opposite_from_turn(ai_turn, game_mode))
                .unwrap_or(self.next_ai_move(ai_turn)),
        );

        if self
            .game_board()
            .make_move(models::player_move(ai_turn), ai_move)
        {
            self.draw();
        }

        if let Some((_, winning_sequence)) = self.game_board.return_winner(0, 0) {
            unsafe {
                IS_GAME_ACTIVE = false;
            }
            self.draw_endgame(ai_turn, winning_sequence);
        } else {
            unsafe {
                IS_GAME_ACTIVE = true;
            }
        }
    }
}

impl game::ConnectFourGame for WebConnectFourGame {
    fn game_board(&mut self) -> &mut models::ConnectFourBoard {
        &mut self.game_board //.clone() // very wrong
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

    fn execute_human_move(
        &mut self,
        chosen_move: usize,
        _moves_left: Vec<usize>,
        game_turn: models::Player,
    ) -> () {
        let game_mode = *self.game_mode();
        if !self
            .game_board()
            .make_move(models::player_move(game_turn), chosen_move)
        {
            return;
        }
        self.draw();
        if let Some((_, winning_sequence)) = self.game_board.return_winner(0, 0) {
            unsafe {
                IS_GAME_ACTIVE = false;
            }
            self.draw_endgame(game_turn, winning_sequence);
            return;
        }

        if self.game_board.moves_left() == 0 {
            unsafe {
                IS_GAME_ACTIVE = false;
            }
            return;
        }

        let ai_turn = game_modes::get_opposite_from_turn(game_turn, game_mode);
        let ai_move = self.game_board().next_winning_move(ai_turn).unwrap_or(
            self.game_board()
                .next_winning_move(game_modes::get_opposite_from_turn(ai_turn, game_mode))
                .unwrap_or(self.next_ai_move(ai_turn)),
        );

        if self
            .game_board()
            .make_move(models::player_move(ai_turn), ai_move)
        {
            self.draw();
        }

        if let Some((_, winning_sequence)) = self.game_board.return_winner(0, 0) {
            unsafe {
                IS_GAME_ACTIVE = false;
            }
            self.draw_endgame(ai_turn, winning_sequence);
        } else if self.game_board.moves_left() > 0 {
            unsafe {
                IS_GAME_ACTIVE = true;
            }
        } else {
            unsafe {
                IS_GAME_ACTIVE = false;
            }
        }
    }

    fn draw(&mut self) -> () {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        let canvas_context: CanvasRenderingContext2d = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        // self.game_board().make_move(ConnectFourMove::OPosition, 0);
        self.game_board().draw(canvas_context, width, height);
    }

    fn draw_endgame(
        &mut self,
        player: models::Player,
        winning_sequence: Vec<(usize, usize)>,
    ) -> () {
        set_game_status(player_winner_text(player));
        launch_fireworks();
        let canvas_context: CanvasRenderingContext2d = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        self.game_board()
            .draw_endgame(canvas_context, player, winning_sequence);
    }
}
