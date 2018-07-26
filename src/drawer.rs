extern crate stdweb;

use constants::{ASCII_UPPERCASE, CELL_WIDTH, CELL_HEIGHT, CELL_PADDING, PADDING};
use models::{ConnectFourBoard, ConnectFourMove, Player};
use self::stdweb::web::CanvasRenderingContext2d;

pub trait Drawer<T> {
    fn draw(&self, canvas: T, width: f64, height: f64) -> ();
    
    fn draw_endgame(&self, _canvas: T, player: Player, winning_sequence: Vec<(usize, usize)>) -> ();
}

impl Drawer<()> for ConnectFourBoard {
    fn draw(&self, _canvas: (), _width: f64, _height: f64) -> () {
        println!("{}", self.to_string());
    }

    fn draw_endgame(&self, _canvas: (), _winner: Player, _winning_sequence: Vec<(usize, usize)>) -> () {
        println!("{}", "STOP");
    }
}

pub fn draw_o(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    canvas_context.set_stroke_style_color("blue");
    canvas_context.begin_path();
    canvas_context.arc(CELL_PADDING * 3.5f64 + f64::from(CELL_WIDTH as i32) * x as f64, CELL_PADDING * 3.5f64 + y as f64 * CELL_HEIGHT as f64, f64::from(CELL_WIDTH as i32) / 1.7f64 - 10f64, 0f64, 2f64 * 3.14f64, false);
    canvas_context.stroke();
    canvas_context.set_stroke_style_color("black");
}

pub fn draw_x(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    canvas_context.set_stroke_style_color("red");
    canvas_context.begin_path();

    canvas_context.move_to(1.5f64 * CELL_PADDING + f64::from(CELL_WIDTH as i32) * x as f64, CELL_PADDING * 1.5f64 + y as f64 * CELL_HEIGHT as f64);
    canvas_context.line_to(CELL_PADDING * 0.5f64 + f64::from(CELL_WIDTH as i32) * x as f64 + f64::from(CELL_WIDTH as i32)/1f64, CELL_PADDING * 2f64 + f64::from(CELL_WIDTH as i32)/1.5f64 + y as f64 * CELL_HEIGHT as f64);
    canvas_context.stroke();

    canvas_context.begin_path();
    canvas_context.move_to(1.5f64 * CELL_PADDING + f64::from(CELL_WIDTH as i32) * x as f64, CELL_PADDING * 2f64 + f64::from(CELL_WIDTH as i32)/1.5f64 + y as f64 * CELL_HEIGHT as f64);
    canvas_context.line_to(CELL_PADDING * 0.5f64 + f64::from(CELL_WIDTH as i32) * x as f64 + f64::from(CELL_WIDTH as i32)/1f64, CELL_PADDING * 1.5f64 + y as f64 * CELL_HEIGHT as f64);

    canvas_context.stroke();
    canvas_context.set_stroke_style_color("black");
}

pub fn draw_non_playable(canvas_context: &CanvasRenderingContext2d, x: usize, y: usize) -> () {
    canvas_context.begin_path();
    canvas_context.arc(CELL_PADDING * 3.5f64 + f64::from(CELL_WIDTH as i32) * x as f64, CELL_PADDING * 3.5f64 + y as f64 * CELL_HEIGHT as f64, f64::from(CELL_WIDTH as i32) / 2.1f64 - 10f64, 0f64, 2f64 * 3.14f64, false);
    canvas_context.move_to(1.5f64 * CELL_PADDING + f64::from(CELL_WIDTH as i32) * x as f64, CELL_PADDING * 2f64 + f64::from(CELL_WIDTH as i32)/1.5f64 + y as f64 * CELL_HEIGHT as f64);
    canvas_context.line_to(CELL_PADDING * 0.5f64 + f64::from(CELL_WIDTH as i32) * x as f64 + f64::from(CELL_WIDTH as i32)/1f64, CELL_PADDING * 1.5f64 + y as f64 * CELL_HEIGHT as f64);
    canvas_context.stroke();
}

impl Drawer<CanvasRenderingContext2d> for ConnectFourBoard {
    fn draw(&self, canvas: CanvasRenderingContext2d, width: f64, height: f64) -> () {
        canvas.clear_rect(0f64, 0f64, width, height);
        canvas.set_line_width(2f64);
        canvas.set_font("25px sans-serif");
        for x in 0..self.width {
            for y in 0..self.height {
                canvas.stroke_rect(f64::from( (x * CELL_WIDTH) as i32 ) + CELL_PADDING, f64::from( (y * CELL_HEIGHT) as i32 ) + PADDING, f64::from(CELL_WIDTH as i32), f64::from(CELL_HEIGHT as i32));
                match self.board[x][y] {
                    ConnectFourMove::OPosition => draw_o(&canvas, x, y), 
                    ConnectFourMove::XPosition => draw_x(&canvas, x, y),
                    ConnectFourMove::UnplayablePosition => draw_non_playable(&canvas, x, y),
                    _ => (),//println!(""),
                };
            }
            canvas.fill_text(&ASCII_UPPERCASE[x].to_string(), CELL_WIDTH as f64 / 2f64 + (x * CELL_WIDTH) as f64, ((self.height + 1) * CELL_HEIGHT) as f64, None);
            // canvas.fill_text(&ASCII_UPPERCASE[x].to_string(), CELL_PADDING +  CELL_WIDTH as f64 / 2f64 + (x * CELL_WIDTH) as f64, ((self.height + 1) * CELL_HEIGHT) as f64, None);
        }
    }
    
    fn draw_endgame(&self, canvas: CanvasRenderingContext2d, winner: Player, winning_sequence: Vec<(usize, usize)>) -> () {
        // canvas.set_font("25px sans-serif");
        canvas.set_line_width(4f64);
        let (mut previous_y, mut previous_x) = winning_sequence[0];
        canvas.begin_path();
        for (index, (y, x)) in winning_sequence.iter().enumerate() {
            if index != 0 {
                println!("CIAO {} {}", y, x);
                println!("CIAO2 {} {}", previous_y, previous_x);
                
                canvas.move_to(f64::from( ( (previous_x) * CELL_WIDTH + CELL_WIDTH / 2) as i32) + PADDING, f64::from( ( (previous_y) * CELL_HEIGHT + CELL_HEIGHT / 2) as i32 ) + PADDING);
                canvas.line_to(f64::from( ( (x) * CELL_WIDTH + CELL_WIDTH / 2) as i32) + PADDING, f64::from( ( (y) * CELL_HEIGHT + CELL_HEIGHT / 2) as i32) + PADDING) ;
                canvas.stroke();
                previous_y = *y;
                previous_x = *x;
            }
        }
        canvas.set_line_width(0f64);
    }
}
