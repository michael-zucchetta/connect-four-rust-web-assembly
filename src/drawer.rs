use crate::constants::{CELL_HEIGHT, CELL_WIDTH, PADDING};
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

fn theme_color(name: &str, fallback: &str) -> String {
    web_sys::window()
        .and_then(|window| {
            window
                .document()
                .and_then(|document| document.body())
                .and_then(|body| window.get_computed_style(&body).ok().flatten())
        })
        .and_then(|style| style.get_property_value(name).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn cell_center(x: usize, y: usize) -> (f64, f64) {
    (
        PADDING + (x as f64 + 0.5) * CELL_WIDTH as f64,
        PADDING + (y as f64 + 0.5) * CELL_HEIGHT as f64,
    )
}

fn cell_gradient_radius() -> f64 {
    let half_width = CELL_WIDTH as f64 / 2f64;
    let half_height = CELL_HEIGHT as f64 / 2f64;
    half_width.hypot(half_height)
}

fn draw_slot(
    canvas_context: &CanvasRenderingContext2d,
    x: usize,
    y: usize,
    color: Option<&str>,
) -> () {
    let grid = theme_color("--canvas-grid", "#2f5f2f");
    let background = theme_color("--bg", "#000");
    let (center_x, center_y) = cell_center(x, y);
    let gradient_radius = cell_gradient_radius();
    let (fill_radius, ring_center) = match color {
        Some(_) => (gradient_radius * 0.40f64, 0.42f64),
        None => (gradient_radius * 0.38f64, 0.40f64),
    };

    canvas_context.set_fill_style(&JsValue::from(color.unwrap_or(&background)));
    canvas_context.set_stroke_style(&JsValue::from(&grid));
    canvas_context.set_line_width(gradient_radius * 0.04f64);
    canvas_context.begin_path();
    canvas_context
        .arc(
            center_x,
            center_y,
            fill_radius,
            0f64,
            2f64 * std::f64::consts::PI,
        )
        .unwrap();
    canvas_context.fill();

    canvas_context.begin_path();
    canvas_context
        .arc(
            center_x,
            center_y,
            gradient_radius * ring_center,
            0f64,
            2f64 * std::f64::consts::PI,
        )
        .unwrap();
    canvas_context.stroke();
}

pub fn draw_o(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    let yellow = theme_color("--piece-yellow", "#ffd866");
    draw_slot(canvas_context, x, y, Some(&yellow));
}

pub fn draw_x(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    let red = theme_color("--piece-red", "#ff5f56");
    draw_slot(canvas_context, x, y, Some(&red));
}

pub fn draw_non_playable(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    draw_slot(canvas_context, x, y, None);
    let (center_x, center_y) = cell_center(x, y);
    let slash_radius = cell_gradient_radius() * 0.18f64;
    canvas_context.set_line_width(1.5f64);
    canvas_context.begin_path();
    canvas_context.move_to(center_x - slash_radius, center_y + slash_radius);
    canvas_context.line_to(center_x + slash_radius, center_y - slash_radius);
    canvas_context.stroke();
}

impl Drawer<CanvasRenderingContext2d> for ConnectFourBoard {
    fn draw(&self, canvas: CanvasRenderingContext2d, width: f64, height: f64) -> () {
        canvas.clear_rect(0f64, 0f64, width, height);
        canvas.set_line_width(1f64);
        canvas.set_font("25px monospace");
        for x in 0..self.width {
            for y in 0..self.height {
                let position = self.board[x][y];
                match position {
                    ConnectFourMove::OPosition => draw_o(&canvas, x, y),
                    ConnectFourMove::OIgnoredPosition => draw_o(&canvas, x, y),
                    ConnectFourMove::XPosition => draw_x(&canvas, x, y),
                    ConnectFourMove::XIgnoredPosition => draw_x(&canvas, x, y),
                    ConnectFourMove::UnplayablePosition => draw_non_playable(&canvas, x, y),
                    _ => draw_slot(&canvas, x, y, None),
                };
            }
        }
    }

    fn draw_endgame(
        &self,
        canvas: CanvasRenderingContext2d,
        _winner: Player,
        winning_sequence: Vec<(usize, usize)>,
    ) -> () {
        let bursts = [
            (
                (self.width * CELL_WIDTH) as f64 * 0.24,
                54f64,
                theme_color("--canvas-burst-a", "#0ff"),
            ),
            (
                (self.width * CELL_WIDTH) as f64 * 0.52,
                44f64,
                theme_color("--canvas-burst-b", "#1cba22"),
            ),
            (
                (self.width * CELL_WIDTH) as f64 * 0.78,
                60f64,
                theme_color("--canvas-burst-c", "#d8a100"),
            ),
        ];
        for (burst_x, burst_y, color) in bursts {
            canvas.set_stroke_style_str(&color);
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
                let (previous_center_x, previous_center_y) = cell_center(previous_x, previous_y);
                let (center_x, center_y) = cell_center(*x, *y);
                canvas.move_to(previous_center_x, previous_center_y);
                canvas.line_to(center_x, center_y);
                canvas.stroke();
                previous_y = *y;
                previous_x = *x;
            }
        }
        canvas.set_line_width(0f64);
    }
}
