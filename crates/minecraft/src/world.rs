use std::collections::HashMap;

use crate::Block;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn offset(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn forward(self, direction: Direction) -> Self {
        let (dx, dz) = direction.offset();
        Self {
            x: self.x + dx,
            y: self.y,
            z: self.z + dz,
        }
    }

    pub fn back(self, direction: Direction) -> Self {
        let (dx, dz) = direction.offset();
        Self {
            x: self.x - dx,
            y: self.y,
            z: self.z - dz,
        }
    }

    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }
}

#[derive(Debug)]
pub struct World {
    blocks: HashMap<Position, Block>,
}

impl World {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn get_block(&self, position: Position) -> Block {
        self.blocks.get(&position).copied().unwrap_or(Block::Air)
    }

    pub fn set_block(&mut self, position: Position, block: Block) {
        if block == Block::Air {
            self.blocks.remove(&position);
        } else {
            self.blocks.insert(position, block);
        }
    }

    pub fn is_solid(&self, position: Position) -> bool {
        self.get_block(position).is_solid()
    }

    pub fn can_dig(&self, position: Position) -> bool {
        self.get_block(position).is_diggable()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
