use minecraft::world::{Direction, Position, World};
use minecraft::{BlockId, ItemStack};
use minecraft::{ItemId, blocks};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractDirection {
    Forward,
    Up,
    Down,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurtleSide {
    Left,
    Right,
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurtleMoveError {
    #[error("Movement obstructed")]
    Obstructed,
    #[error("Out of fuel")]
    OutOfFuel,
}

#[derive(Debug, Serialize)]
pub struct InspectData {
    pub name: String,
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurtleInspectError {}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurtleDigError {
    #[error("Nothing to dig here")]
    NothingToDig,
    #[error("Cannot break unbreakable block")]
    UnbreakableBlock,
    #[error("No tool to dig with")]
    NoTool,
    #[error("Cannot break block with this tool")]
    WrongTool,
}

#[derive(Debug, Serialize)]
pub struct ItemDetail {
    pub name: String,
    pub count: u32,
}

#[derive(Debug)]
pub struct Turtle {
    /// The position of the turtle in the world.
    pub position: Position,
    pub position_history: Vec<Position>,
    /// The direction the turtle is facing.
    pub facing: Direction,
    pub kind: TurtleKind,
    pub fuel: u32,
    pub inventory: [Option<ItemStack>; 16],
    pub selected_slot: usize,
    pub left_upgrade: Option<ItemId>,
    pub right_upgrade: Option<ItemId>,
}

impl Turtle {
    pub fn new(position: Position, direction: Direction, kind: TurtleKind) -> Self {
        Self {
            position,
            position_history: vec![position],
            facing: direction,
            kind,
            fuel: kind.fuel_limit(),
            inventory: Default::default(),
            selected_slot: 0,
            left_upgrade: None,
            right_upgrade: None,
        }
    }

    pub fn set_upgrade(&mut self, side: TurtleSide, upgrade: Option<ItemId>) {
        match side {
            TurtleSide::Left => self.left_upgrade = upgrade,
            TurtleSide::Right => self.right_upgrade = upgrade,
        }
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
        self.position_history.push(position);
    }

    /// Returns the position the turtle is looking at.
    pub fn looking_at(&self) -> Position {
        self.position.forward(self.facing)
    }

    pub fn move_to(&mut self, position: Position, world: &World) -> Result<(), TurtleMoveError> {
        if world.is_solid(position) {
            return Err(TurtleMoveError::Obstructed);
        }

        if self.fuel == 0 {
            return Err(TurtleMoveError::OutOfFuel);
        }

        self.set_position(position);
        self.fuel = self.fuel.saturating_sub(1);

        Ok(())
    }

    pub fn forward(&mut self, world: &World) -> Result<(), TurtleMoveError> {
        self.move_to(self.position.forward(self.facing), world)
    }

    pub fn back(&mut self, world: &World) -> Result<(), TurtleMoveError> {
        self.move_to(self.position.back(self.facing), world)
    }

    pub fn up(&mut self, world: &World) -> Result<(), TurtleMoveError> {
        self.move_to(self.position.up(), world)
    }

    pub fn down(&mut self, world: &World) -> Result<(), TurtleMoveError> {
        self.move_to(self.position.down(), world)
    }

    pub fn turn_left(&mut self) {
        self.facing = self.facing.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.facing = self.facing.turn_right();
    }

    pub fn detect(&self, world: &World) -> bool {
        let target_position = self.position.forward(self.facing);
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

    pub fn inspect(
        &self,
        _direction: InteractDirection,
        _world: &World,
    ) -> Result<InspectData, TurtleInspectError> {
        Ok(InspectData {
            name: "minecraft:wheat".to_string(),
        })
    }

    pub fn inspect_forward(&self, world: &World) -> Result<InspectData, TurtleInspectError> {
        self.inspect(InteractDirection::Forward, world)
    }

    pub fn inspect_up(&self, world: &World) -> Result<InspectData, TurtleInspectError> {
        self.inspect(InteractDirection::Up, world)
    }

    pub fn inspect_down(&self, world: &World) -> Result<InspectData, TurtleInspectError> {
        self.inspect(InteractDirection::Down, world)
    }

    pub fn dig(
        &mut self,
        direction: InteractDirection,
        side: TurtleSide,
        world: &mut World,
    ) -> Result<(), TurtleDigError> {
        let target_position = match direction {
            InteractDirection::Forward => self.position.forward(self.facing),
            InteractDirection::Up => self.position.up(),
            InteractDirection::Down => self.position.down(),
        };

        let block = world.get_block(target_position);
        if block.id == BlockId::AIR {
            return Err(TurtleDigError::NothingToDig);
        }

        if block.id == BlockId::BEDROCK {
            return Err(TurtleDigError::UnbreakableBlock);
        }

        let upgrade = match side {
            TurtleSide::Left => self.left_upgrade.as_ref(),
            TurtleSide::Right => self.right_upgrade.as_ref(),
        };
        let Some(upgrade) = upgrade else {
            return Err(TurtleDigError::NoTool);
        };

        if !world.can_dig(target_position) {
            return Err(TurtleDigError::WrongTool);
        }

        const DIAMOND_HOE: ItemId = ItemId::new_static("minecraft:diamond_hoe");

        if *upgrade == DIAMOND_HOE {
            if block.id != BlockId::GRASS_BLOCK && block.id != BlockId::DIRT {
                return Err(TurtleDigError::WrongTool);
            }

            world.set_block(target_position, blocks::FARMLAND.clone());
        } else {
            world.set_block(target_position, blocks::AIR.clone());
        }

        Ok(())
    }

    pub fn dig_forward(
        &mut self,
        side: TurtleSide,
        world: &mut World,
    ) -> Result<(), TurtleDigError> {
        self.dig(InteractDirection::Forward, side, world)
    }

    pub fn dig_up(&mut self, side: TurtleSide, world: &mut World) -> Result<(), TurtleDigError> {
        self.dig(InteractDirection::Up, side, world)
    }

    pub fn dig_down(&mut self, side: TurtleSide, world: &mut World) -> Result<(), TurtleDigError> {
        self.dig(InteractDirection::Down, side, world)
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

    pub fn get_item_detail(&self, slot: usize, _detailed: bool) -> Option<ItemDetail> {
        if slot >= 16 {
            return None;
        }

        let stack = self.inventory[slot].as_ref()?;

        Some(ItemDetail {
            name: stack.name.clone(),
            count: stack.count,
        })
    }
}

#[cfg(test)]
mod tests {
    use minecraft::blocks;
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
        assert_eq!(turtle.facing, Direction::East);

        turtle.forward(&mut world).unwrap();
        assert_eq!(turtle.position, Position::new(1, 0, -1));
    }

    #[test]
    fn test_turtle_blocked_movement() {
        let mut world = World::new();
        world.set_block(Position::new(0, 0, -1), blocks::STONE.clone());

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
        world.set_block(Position::new(0, 0, -1), blocks::STONE.clone());

        let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleKind::Normal);
        turtle.set_upgrade(
            TurtleSide::Right,
            Some(ItemId::new_static("minecraft:diamond_pickaxe")),
        );

        assert!(turtle.detect(&world));

        turtle.dig_forward(TurtleSide::Right, &mut world).unwrap();
        assert_eq!(
            world.get_block(Position::new(0, 0, -1)),
            blocks::AIR.clone()
        );
        assert!(!turtle.detect(&world));
    }
}
