use minecraft::world::{Direction, Position, World};
use minecraft::{Block, ItemStack};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurtleKind {
    Normal,
    Advanced,
}

impl TurtleKind {
    pub fn fuel_limit(&self) -> u32 {
        match self {
            TurtleKind::Normal => 20_000,
            TurtleKind::Advanced => 100_000,
        }
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurtleMoveError {
    #[error("Movement obstructed")]
    Obstructed,
    #[error("Out of fuel")]
    OutOfFuel,
}

#[derive(Debug)]
pub struct Turtle {
    pub position: Position,
    pub position_history: Vec<Position>,
    pub direction: Direction,
    pub kind: TurtleKind,
    pub fuel: u32,
    pub inventory: [Option<ItemStack>; 16],
    pub selected_slot: usize,
    pub left_upgrade: Option<String>,
    pub right_upgrade: Option<String>,
}

impl Turtle {
    pub fn new(position: Position, direction: Direction, kind: TurtleKind) -> Self {
        Self {
            position,
            position_history: vec![position],
            direction,
            kind,
            fuel: kind.fuel_limit(),
            inventory: Default::default(),
            selected_slot: 0,
            left_upgrade: None,
            right_upgrade: None,
        }
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
        self.position_history.push(position);
    }

    pub fn forward(&mut self, world: &mut World) -> Result<(), TurtleMoveError> {
        let target_position = self.position.forward(self.direction);

        if world.is_solid(target_position) {
            return Err(TurtleMoveError::Obstructed);
        }

        if self.fuel == 0 {
            return Err(TurtleMoveError::OutOfFuel);
        }

        self.set_position(target_position);
        self.fuel = self.fuel.saturating_sub(1);

        Ok(())
    }

    pub fn back(&mut self, world: &mut World) -> Result<(), TurtleMoveError> {
        let target_position = self.position.back(self.direction);

        if world.is_solid(target_position) {
            return Err(TurtleMoveError::Obstructed);
        }

        if self.fuel == 0 {
            return Err(TurtleMoveError::OutOfFuel);
        }

        self.set_position(target_position);
        self.fuel = self.fuel.saturating_sub(1);

        Ok(())
    }

    pub fn up(&mut self, world: &mut World) -> Result<(), TurtleMoveError> {
        let target_position = self.position.up();

        if world.is_solid(target_position) {
            return Err(TurtleMoveError::Obstructed);
        }

        if self.fuel == 0 {
            return Err(TurtleMoveError::OutOfFuel);
        }

        self.set_position(target_position);
        self.fuel = self.fuel.saturating_sub(1);

        Ok(())
    }

    pub fn down(&mut self, world: &mut World) -> Result<(), TurtleMoveError> {
        let target_position = self.position.down();

        if world.is_solid(target_position) {
            return Err(TurtleMoveError::Obstructed);
        }

        if self.fuel == 0 {
            return Err(TurtleMoveError::OutOfFuel);
        }

        self.set_position(target_position);
        self.fuel = self.fuel.saturating_sub(1);

        Ok(())
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
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

    pub fn dig(&mut self, world: &mut World) -> (bool, Option<String>) {
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

    pub fn dig_up(&mut self, world: &mut World) -> (bool, Option<String>) {
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

    pub fn dig_down(&mut self, world: &mut World) -> (bool, Option<String>) {
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
        self.kind.fuel_limit()
    }

    pub fn get_selected_slot(&self) -> usize {
        self.selected_slot
    }

    pub fn select(&mut self, slot: usize) -> (bool, Option<String>) {
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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_direction_turns() {
        let dir = Direction::North;
        let dir = dir.turn_right();
        assert_eq!(dir, Direction::East);
        let dir = dir.turn_right();
        assert_eq!(dir, Direction::South);
        let dir = dir.turn_left();
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
        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleKind::Normal);

        turtle.forward(&mut world).unwrap();
        assert_eq!(turtle.position, Position::new(0, 0, -1));

        turtle.turn_right();
        assert_eq!(turtle.direction, Direction::East);

        turtle.forward(&mut world).unwrap();
        assert_eq!(turtle.position, Position::new(1, 0, -1));
    }

    #[test]
    fn test_turtle_blocked_movement() {
        let mut world = World::new();
        world.set_block(Position::new(0, 0, -1), Block::Stone);

        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleKind::Normal);

        let result = turtle.forward(&mut world);
        assert_eq!(result, Err(TurtleMoveError::Obstructed));
        assert_eq!(turtle.position, Position::new(0, 0, 0));
    }

    #[test]
    fn test_turtle_fuel_consumption() {
        let mut world = World::new();
        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleKind::Normal);

        let initial_fuel = turtle.get_fuel_level();
        turtle.forward(&mut world).unwrap();
        assert_eq!(turtle.get_fuel_level(), initial_fuel - 1);
    }

    #[test]
    fn test_turtle_out_of_fuel() {
        let mut world = World::new();
        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleKind::Normal);
        turtle.fuel = 0;

        let result = turtle.forward(&mut world);
        assert_eq!(result, Err(TurtleMoveError::OutOfFuel));
    }

    #[test]
    fn test_turtle_dig() {
        let mut world = World::new();
        world.set_block(Position::new(0, 0, -1), Block::Stone);

        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleKind::Normal);

        assert!(turtle.detect(&world));

        let (success, _) = turtle.dig(&mut world);
        assert!(success);
        assert_eq!(world.get_block(Position::new(0, 0, -1)), Block::Air);
        assert!(!turtle.detect(&world));
    }
}
