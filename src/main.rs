pub mod model;
pub mod service;


extern crate rand;
extern crate piston_window;

use piston_window::{Button, clear, PistonWindow, PressEvent, UpdateEvent, WindowSettings};
use piston_window::types::Color;
use crate::service::draw::to_coordinate_u32;
use crate::service::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new("Snake",
                                                       [to_coordinate_u32(width), to_coordinate_u32(height)])
        .exit_on_esc(true)
        .build().unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key)
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw_board(&c, g)
        });
        event.update(|arg| {
            game.update(arg.dt)
        });
    }
}
