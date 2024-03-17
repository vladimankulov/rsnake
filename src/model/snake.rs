use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::model::block::Block;
use crate::model::direction::Direction;
use crate::service::draw::draw_block;

const SNAKE_COLOR: Color = [0.0, 0.80, 0.0, 1.00];

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::RIGHT,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, context, g2d);
        }
    }

    pub fn head(&self) -> Block {
        let x = &self.body.front().unwrap();
        Block { x: x.x, y: x.y }
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        direction.map(|d| self.direction = d);

        let last_head = self.head();

        let new_block = match self.direction {
            Direction::UP => Block { x: last_head.x, y: last_head.y - 1 },
            Direction::DOWN => Block { x: last_head.x, y: last_head.y + 1 },
            Direction::LEFT => Block { x: last_head.x - 1, y: last_head.y },
            Direction::RIGHT => Block { x: last_head.x + 1, y: last_head.y }
        };
        self.body.push_front(new_block);
        self.tail = self.body.pop_back();
    }


    pub fn next_head(&self, direction: Option<Direction>) -> Block {
        let mut new_direction = self.direction;
        direction.map(|d| new_direction = d);

        let mut next_head = self.head();
        match new_direction {
            Direction::UP => { next_head.y -= 1 }
            Direction::DOWN => { next_head.y += 1 }
            Direction::LEFT => { next_head.x -= 1 }
            Direction::RIGHT => { next_head.x += 1 }
        };
        next_head
    }
    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn restore_tail(&mut self) {
        self.tail.map(|b| self.body.push_back(b));
    }

    pub fn is_overlapped(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }
            ch += 1;
            if self.body.len() - 1 == ch {
                break;
            }
        }
        false
    }
}