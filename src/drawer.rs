use crate::constants::{ASCII_UPPERCASE, CELL_HEIGHT, CELL_PADDING, CELL_WIDTH, PADDING};
use crate::models::{ConnectFourBoard, ConnectFourMove, Player};
use web_sys::CanvasRenderingContext2d;
use web_sys::wasm_bindgen::JsValue;

pub trait Drawer<T> {
    fn draw(&self, canvas: T, width: f64, height: f64) -> ();

    fn draw_endgame(&self, _canvas: T, player: Player, winning_sequence: Vec<(usize, usize)>)
    -> ();
}

impl Drawer<()> for ConnectFourBoard {
    fn draw(&self, _canvas: (), _width: f64, _height: f64) -> () {
        println!("{}", self.to_string());
    }

    fn draw_endgame(
        &self,
        _canvas: (),
        _winner: Player,
        _winning_sequence: Vec<(usize, usize)>,
    ) -> () {
        println!("{}", "STOP");
    }
}

fn draw_disc(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize, color: &str) -> () {
    canvas_context.set_fill_style(&JsValue::from(color));
    canvas_context.set_stroke_style(&JsValue::from("#f5f7f7"));
    canvas_context.begin_path();
    canvas_context
        .arc(
            CELL_PADDING * 3.5f64 + f64::from(CELL_WIDTH as i32) * x as f64, // x
            CELL_PADDING * 3.5f64 + y as f64 * CELL_HEIGHT as f64,           // y
            f64::from(CELL_WIDTH as i32) / 1.7f64 - 10f64,                   // radius
            0f64,                                                            // start_angle
            2f64 * 3.14f64,                                                  // end_angle
        )
        .unwrap();
    canvas_context.fill();
    canvas_context.stroke();
    canvas_context.set_stroke_style(&JsValue::from("#2f5f2f"));
}

pub fn draw_o(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    draw_disc(canvas_context, x, y, "#ffd866");
}

pub fn draw_x(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    draw_disc(canvas_context, x, y, "#ff5f56");
}

pub fn draw_non_playable(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    canvas_context.begin_path();
    canvas_context
        .arc(
            CELL_PADDING * 3.5f64 + f64::from(CELL_WIDTH as i32) * x as f64,
            CELL_PADDING * 3.5f64 + y as f64 * CELL_HEIGHT as f64,
            f64::from(CELL_WIDTH as i32) / 2.1f64 - 10f64,
            0f64,
            2f64 * 3.14f64,
        )
        .unwrap();
    canvas_context.move_to(
        1.5f64 * CELL_PADDING + f64::from(CELL_WIDTH as i32) * x as f64,
        CELL_PADDING * 2f64 + f64::from(CELL_WIDTH as i32) / 1.5f64 + y as f64 * CELL_HEIGHT as f64,
    );
    canvas_context.line_to(
        CELL_PADDING * 0.5f64
            + f64::from(CELL_WIDTH as i32) * x as f64
            + f64::from(CELL_WIDTH as i32) / 1f64,
        CELL_PADDING * 1.5f64 + y as f64 * CELL_HEIGHT as f64,
    );
    canvas_context.stroke();
}

impl Drawer<CanvasRenderingContext2d> for ConnectFourBoard {
    fn draw(&self, canvas: CanvasRenderingContext2d, width: f64, height: f64) -> () {
        canvas.clear_rect(0f64, 0f64, width, height);
        canvas.set_line_width(2f64);
        canvas.set_stroke_style_str("#2f5f2f");
        canvas.set_font("25px monospace");
        for x in 0..self.width {
            for y in 0..self.height {
                canvas.stroke_rect(
                    f64::from((x * CELL_WIDTH) as i32) + CELL_PADDING,
                    f64::from((y * CELL_HEIGHT) as i32) + PADDING,
                    f64::from(CELL_WIDTH as i32),
                    f64::from(CELL_HEIGHT as i32),
                );
                match self.board[x][y] {
                    ConnectFourMove::OPosition => draw_o(&canvas, x, y),
                    ConnectFourMove::XPosition => draw_x(&canvas, x, y),
                    ConnectFourMove::UnplayablePosition => draw_non_playable(&canvas, x, y),
                    _ => (), //println!(""),
                };
            }
            canvas.set_fill_style_str("#1cba22");
            canvas.set_stroke_style_str("#1cba22");
            canvas.set_font("25px monospace");
            canvas
                .fill_text(
                    &ASCII_UPPERCASE[x].to_string(),
                    CELL_WIDTH as f64 / 2f64 + (x * CELL_WIDTH) as f64,
                    ((self.height + 1) * CELL_HEIGHT) as f64,
                )
                .unwrap();
            // canvas.fill_text(&ASCII_UPPERCASE[x].to_string(), CELL_PADDING +  CELL_WIDTH as f64 / 2f64 + (x * CELL_WIDTH) as f64, ((self.height + 1) * CELL_HEIGHT) as f64, None);
        }
    }

    fn draw_endgame(
        &self,
        canvas: CanvasRenderingContext2d,
        _winner: Player,
        winning_sequence: Vec<(usize, usize)>,
    ) -> () {
        let bursts = [
            ((self.width * CELL_WIDTH) as f64 * 0.24, 54f64, "#0ff"),
            ((self.width * CELL_WIDTH) as f64 * 0.52, 44f64, "#1cba22"),
            ((self.width * CELL_WIDTH) as f64 * 0.78, 60f64, "#d8a100"),
        ];
        for (burst_x, burst_y, color) in bursts {
            canvas.set_stroke_style_str(color);
            for index in 0..8 {
                let angle = index as f64 * std::f64::consts::PI / 4f64;
                canvas.begin_path();
                canvas.move_to(burst_x, burst_y);
                canvas.line_to(burst_x + angle.cos() * 18f64, burst_y + angle.sin() * 18f64);
                canvas.stroke();
            }
        }

        canvas.set_line_width(4f64);
        let (mut previous_y, mut previous_x) = winning_sequence[0];
        canvas.begin_path();
        for (index, (y, x)) in winning_sequence.iter().enumerate() {
            if index != 0 {
                canvas.move_to(
                    f64::from(((previous_x) * CELL_WIDTH + CELL_WIDTH / 2) as i32) + PADDING,
                    f64::from(((previous_y) * CELL_HEIGHT + CELL_HEIGHT / 2) as i32) + PADDING,
                );
                canvas.line_to(
                    f64::from(((x) * CELL_WIDTH + CELL_WIDTH / 2) as i32) + PADDING,
                    f64::from(((y) * CELL_HEIGHT + CELL_HEIGHT / 2) as i32) + PADDING,
                );
                canvas.stroke();
                previous_y = *y;
                previous_x = *x;
            }
        }
        canvas.set_line_width(0f64);
    }
}
