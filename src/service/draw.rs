use piston_window::{rectangle, Context, G2d};
use piston_window::types::{Color};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coordinate(game_coordinate: i32) -> f64 {
    (game_coordinate as f64) * BLOCK_SIZE
}

pub fn to_coordinate_u32(game_coordinate: i32) -> u32 {
    to_coordinate(game_coordinate) as u32
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, context: &Context, graphic: &mut G2d) {
    let x_coordinate: f64 = to_coordinate(x);
    let y_coordinate: f64 = to_coordinate(y);

    rectangle(color,
              [x_coordinate, y_coordinate, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
              context.transform,
              graphic)
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, graphic: &mut G2d) {
    let x_coordinate: f64 = to_coordinate(x);
    let y_coordinate: f64 = to_coordinate(y);

    rectangle(color,
              [x_coordinate, y_coordinate, BLOCK_SIZE, BLOCK_SIZE],
              context.transform,
              graphic)
}
