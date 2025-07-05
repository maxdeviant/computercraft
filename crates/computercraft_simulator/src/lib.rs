use std::collections::HashMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Air,
    Stone,
    Dirt,
    Wood,
    Cobblestone,
    Bedrock,
}

impl Block {
    pub fn is_solid(&self) -> bool {
        !matches!(self, Block::Air)
    }

    pub fn is_diggable(&self) -> bool {
        !matches!(self, Block::Air | Block::Bedrock)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

impl ItemStack {
    pub fn new(name: String, count: u32) -> Self {
        Self { name, count }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn max_stack_size(&self) -> u32 {
        64
    }

    pub fn space_left(&self) -> u32 {
        self.max_stack_size().saturating_sub(self.count)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurtleType {
    Normal,
    Advanced,
}

impl TurtleType {
    pub fn fuel_limit(&self) -> u32 {
        match self {
            TurtleType::Normal => 20_000,
            TurtleType::Advanced => 100_000,
        }
    }
}

#[derive(Debug)]
pub struct Turtle {
    pub position: Position,
    pub direction: Direction,
    pub turtle_type: TurtleType,
    pub fuel: u32,
    pub inventory: [Option<ItemStack>; 16],
    pub selected_slot: usize,
    pub left_upgrade: Option<String>,
    pub right_upgrade: Option<String>,
}

pub type TurtleResult = (bool, Option<String>);

impl Turtle {
    pub fn new(position: Position, direction: Direction, turtle_type: TurtleType) -> Self {
        Self {
            position,
            direction,
            turtle_type,
            fuel: turtle_type.fuel_limit(),
            inventory: Default::default(),
            selected_slot: 0,
            left_upgrade: None,
            right_upgrade: None,
        }
    }

    pub fn forward(&mut self, world: &mut World) -> TurtleResult {
        let target_position = self.position.forward(self.direction);

        if world.is_solid(target_position) {
            return (false, Some("Movement obstructed".to_string()));
        }

        if self.fuel == 0 {
            return (false, Some("Out of fuel".to_string()));
        }

        self.position = target_position;
        self.fuel = self.fuel.saturating_sub(1);
        (true, None)
    }

    pub fn back(&mut self, world: &mut World) -> TurtleResult {
        let target_position = self.position.back(self.direction);

        if world.is_solid(target_position) {
            return (false, Some("Movement obstructed".to_string()));
        }

        if self.fuel == 0 {
            return (false, Some("Out of fuel".to_string()));
        }

        self.position = target_position;
        self.fuel = self.fuel.saturating_sub(1);
        (true, None)
    }

    pub fn up(&mut self, world: &mut World) -> TurtleResult {
        let target_position = self.position.up();

        if world.is_solid(target_position) {
            return (false, Some("Movement obstructed".to_string()));
        }

        if self.fuel == 0 {
            return (false, Some("Out of fuel".to_string()));
        }

        self.position = target_position;
        self.fuel = self.fuel.saturating_sub(1);
        (true, None)
    }

    pub fn down(&mut self, world: &mut World) -> TurtleResult {
        let target_position = self.position.down();

        if world.is_solid(target_position) {
            return (false, Some("Movement obstructed".to_string()));
        }

        if self.fuel == 0 {
            return (false, Some("Out of fuel".to_string()));
        }

        self.position = target_position;
        self.fuel = self.fuel.saturating_sub(1);
        (true, None)
    }

    pub fn turn_left(&mut self) -> TurtleResult {
        self.direction = self.direction.turn_left();
        (true, None)
    }

    pub fn turn_right(&mut self) -> TurtleResult {
        self.direction = self.direction.turn_right();
        (true, None)
    }

    pub fn detect(&self, world: &World) -> bool {
        let target_position = self.position.forward(self.direction);
        world.is_solid(target_position)
    }

    pub fn detect_up(&self, world: &World) -> bool {
        let target_position = self.position.up();
        world.is_solid(target_position)
    }

    pub fn detect_down(&self, world: &World) -> bool {
        let target_position = self.position.down();
        world.is_solid(target_position)
    }

    pub fn dig(&mut self, world: &mut World) -> TurtleResult {
        let target_position = self.position.forward(self.direction);

        if !world.can_dig(target_position) {
            return (false, Some("Nothing to dig".to_string()));
        }

        let block = world.get_block(target_position);
        if block == Block::Air {
            return (false, Some("Nothing to dig".to_string()));
        }

        world.set_block(target_position, Block::Air);
        (true, None)
    }

    pub fn dig_up(&mut self, world: &mut World) -> TurtleResult {
        let target_position = self.position.up();

        if !world.can_dig(target_position) {
            return (false, Some("Nothing to dig".to_string()));
        }

        let block = world.get_block(target_position);
        if block == Block::Air {
            return (false, Some("Nothing to dig".to_string()));
        }

        world.set_block(target_position, Block::Air);
        (true, None)
    }

    pub fn dig_down(&mut self, world: &mut World) -> TurtleResult {
        let target_position = self.position.down();

        if !world.can_dig(target_position) {
            return (false, Some("Nothing to dig".to_string()));
        }

        let block = world.get_block(target_position);
        if block == Block::Air {
            return (false, Some("Nothing to dig".to_string()));
        }

        world.set_block(target_position, Block::Air);
        (true, None)
    }

    pub fn get_fuel_level(&self) -> u32 {
        self.fuel
    }

    pub fn get_fuel_limit(&self) -> u32 {
        self.turtle_type.fuel_limit()
    }

    pub fn get_selected_slot(&self) -> usize {
        self.selected_slot
    }

    pub fn select(&mut self, slot: usize) -> TurtleResult {
        if slot >= 16 {
            return (false, Some("Slot out of range".to_string()));
        }

        self.selected_slot = slot;
        (true, None)
    }

    pub fn get_item_count(&self, slot: Option<usize>) -> u32 {
        let slot = slot.unwrap_or(self.selected_slot);
        if slot >= 16 {
            return 0;
        }

        self.inventory[slot]
            .as_ref()
            .map(|stack| stack.count)
            .unwrap_or(0)
    }

    pub fn get_item_space(&self, slot: Option<usize>) -> u32 {
        let slot = slot.unwrap_or(self.selected_slot);
        if slot >= 16 {
            return 0;
        }

        self.inventory[slot]
            .as_ref()
            .map(|stack| stack.space_left())
            .unwrap_or(64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_turns() {
        let mut dir = Direction::North;
        dir = dir.turn_right();
        assert_eq!(dir, Direction::East);
        dir = dir.turn_right();
        assert_eq!(dir, Direction::South);
        dir = dir.turn_left();
        assert_eq!(dir, Direction::East);
    }

    #[test]
    fn test_position_movement() {
        let pos = Position::new(0, 0, 0);
        let forward = pos.forward(Direction::North);
        assert_eq!(forward, Position::new(0, 0, -1));

        let up = pos.up();
        assert_eq!(up, Position::new(0, 1, 0));
    }

    #[test]
    fn test_turtle_basic_movement() {
        let mut world = World::new();
        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleType::Normal);

        let (success, _) = turtle.forward(&mut world);
        assert!(success);
        assert_eq!(turtle.position, Position::new(0, 0, -1));

        let (success, _) = turtle.turn_right();
        assert!(success);
        assert_eq!(turtle.direction, Direction::East);

        let (success, _) = turtle.forward(&mut world);
        assert!(success);
        assert_eq!(turtle.position, Position::new(1, 0, -1));
    }

    #[test]
    fn test_turtle_blocked_movement() {
        let mut world = World::new();
        world.set_block(Position::new(0, 0, -1), Block::Stone);

        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleType::Normal);

        let (success, error) = turtle.forward(&mut world);
        assert!(!success);
        assert!(error.is_some());
        assert_eq!(turtle.position, Position::new(0, 0, 0));
    }

    #[test]
    fn test_turtle_fuel_consumption() {
        let mut world = World::new();
        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleType::Normal);

        let initial_fuel = turtle.get_fuel_level();
        turtle.forward(&mut world);
        assert_eq!(turtle.get_fuel_level(), initial_fuel - 1);
    }

    #[test]
    fn test_turtle_out_of_fuel() {
        let mut world = World::new();
        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleType::Normal);
        turtle.fuel = 0;

        let (success, error) = turtle.forward(&mut world);
        assert!(!success);
        assert_eq!(error, Some("Out of fuel".to_string()));
    }

    #[test]
    fn test_turtle_dig() {
        let mut world = World::new();
        world.set_block(Position::new(0, 0, -1), Block::Stone);

        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleType::Normal);

        assert!(turtle.detect(&world));

        let (success, _) = turtle.dig(&mut world);
        assert!(success);
        assert_eq!(world.get_block(Position::new(0, 0, -1)), Block::Air);
        assert!(!turtle.detect(&world));
    }
}
