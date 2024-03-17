use piston_window::{Context, G2d, Key};
use piston_window::types::{Color};
use rand::{Rng, thread_rng};
use crate::model::block::Block;
use crate::model::direction::Direction;
use crate::model::snake::Snake;
use crate::service::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.0, 0.0, 1.00];
const BORDER_COLOR: Color = [0.00, 0.0, 0.0, 1.00];
const GAME_OVER_COLOR: Color = [0.90, 0.0, 0.0, 0.5];
const RESTART_TIME: f64 = 1.0;
const MOVING_PERIOD: f64 = 0.1;

pub struct Game {
    snake: Snake,
    food: Option<Block>,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food: Some(Block::new(6, 4)),
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over { return; }

        let next_direction = match key {
            Key::Up => Some(Direction::UP),
            Key::Down => Some(Direction::DOWN),
            Key::Left => Some(Direction::LEFT),
            Key::Right => Some(Direction::RIGHT),
            _ => None
        };

        if next_direction.is_some() {
            if self.snake.get_direction().opposite() == next_direction.unwrap() {
                return;
            }
        }

        self.update_snake(next_direction);
    }

    pub fn draw_board(&self, context: &Context, g2d: &mut G2d) {
        self.snake.draw(context, g2d);

        self.food.map(|b| { draw_block(FOOD_COLOR, b.x, b.y, context, g2d) });

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g2d);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, self.height, context, g2d);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g2d);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, context, g2d);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, context, g2d)
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if self.food.is_none() {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn check_if_snake_has_eaten_food(&mut self) {
        let snake_head = self.snake.head();
        self.food.map(|f| {
            if f.is_intersect(&snake_head) {
                self.snake.restore_tail();
                self.food = None
            }
        });
    }

    pub fn check_if_snake_is_alive(&self, direction: Option<Direction>) -> bool {
        let snake_head = self.snake.next_head(direction);
        if self.snake.is_overlapped(snake_head.x, snake_head.y) {
            println!("snake is overlapped");
            return false;
        }
        snake_head.x > 0 && snake_head.y > 0 && snake_head.x < self.width - 1 && snake_head.y < self.height - 1
    }

    pub fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut food_x = rng.gen_range(1..self.width - 1);
        let mut food_y = rng.gen_range(1..self.height - 1);
        while self.snake.is_overlapped(food_x, food_y) {
            food_x = rng.gen_range(1..self.width - 1);
            food_y = rng.gen_range(1..self.height - 1);
        }
        self.food = Some(Block::new(food_x, food_y))
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.check_if_snake_is_alive(direction) {
            self.snake.move_forward(direction);
            self.check_if_snake_has_eaten_food();
        } else {
            self.game_over = true
        }
        self.waiting_time = 0.0
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food = Some(Block::new(6, 4));
        self.game_over = false;
        self.waiting_time = 0.0;
    }
}
