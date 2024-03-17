#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Block {
        Block { x, y }
    }

    pub fn is_intersect(&self, another: &Block) -> bool {
        return self.x == another.x && self.y == another.y;
    }
}