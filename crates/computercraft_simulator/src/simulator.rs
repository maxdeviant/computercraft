use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use minecraft::Block;
use minecraft::world::{Direction, Position, World};
use mlua::{Lua, LuaOptions, LuaSerdeExt, StdLib};
use serde::Serialize;
use thiserror::Error;

use crate::{
    InspectData, Turtle, TurtleDigError, TurtleInspectError, TurtleKind, TurtleMoveError,
    TurtlePlaceError, TurtleSide,
};

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("Lua error: {0}")]
    LuaError(#[from] mlua::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type SimulatorResult<T, E = SimulatorError> = Result<T, E>;

pub struct Simulator {
    lua: Lua,
    state: Rc<SimulatorState>,
}

impl Simulator {
    pub fn new() -> SimulatorResult<Self> {
        let lua = {
            let stdlib = StdLib::ALL_SAFE | StdLib::DEBUG;
            let options = LuaOptions::default();

            unsafe { Lua::unsafe_new_with(stdlib, options) }
        };

        let mut this = Self {
            lua,
            state: Rc::new(SimulatorState::new()),
        };

        this.init_require()?;
        this.init_turtle_api()?;

        Ok(this)
    }

    pub fn world(&self) -> std::cell::Ref<'_, World> {
        self.state.world.borrow()
    }

    pub fn turtle(&self) -> std::cell::Ref<'_, Turtle> {
        self.state.turtle.borrow()
    }

    pub fn turtle_mut(&self) -> std::cell::RefMut<'_, Turtle> {
        self.state.turtle.borrow_mut()
    }

    pub fn set_current_dir(&mut self, root_dir: impl AsRef<Path>) {
        *self.state.current_dir.borrow_mut() = root_dir.as_ref().to_path_buf();
    }

    pub fn block_at(&self, position: Position) -> Block {
        let world = self.state.world.borrow();
        world.get_block(position)
    }

    /// Sets the block at the given position.
    pub fn set_block_at(&self, position: Position, block: Block) {
        let mut world = self.state.world.borrow_mut();
        world.set_block(position, block);
    }

    /// Moves the turtle to the given position.
    pub fn move_turtle_to(&self, position: Position) {
        let mut turtle = self.state.turtle.borrow_mut();
        turtle.position = position;
    }

    fn init_require(&mut self) -> SimulatorResult<()> {
        let state = self.state.clone();
        let globals = self.lua.globals();

        globals.set(
            "require",
            self.lua.create_function(move |lua, module_name: String| {
                let module_name_normalized = module_name.replace('.', "/");
                let file_path = state
                    .current_dir
                    .borrow()
                    .join(format!("{module_name_normalized}.lua"));
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => {
                        let result: mlua::Value<'_> = lua.load(content.as_bytes()).eval()?;
                        Ok(result)
                    }
                    Err(err) => Err(mlua::Error::RuntimeError(format!(
                        "Module '{module_name}' not found: {err}",
                    ))),
                }
            })?,
        )?;

        Ok(())
    }

    fn init_turtle_api(&mut self) -> SimulatorResult<()> {
        let globals = self.lua.globals();

        let turtle_table = self.lua.create_table()?;

        turtle_table.set(
            "forward",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();
                    let world = state.world.borrow();

                    Ok(turtle.forward(&world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "back",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();
                    let world = state.world.borrow();

                    Ok(turtle.back(&world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "up",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();
                    let world = state.world.borrow();

                    Ok(turtle.up(&world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "down",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();
                    let world = state.world.borrow();

                    Ok(turtle.down(&world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "turnLeft",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();

                    turtle.turn_left();

                    Ok((true, None::<String>))
                }
            })?,
        )?;
        turtle_table.set(
            "turnRight",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();

                    turtle.turn_right();

                    Ok((true, None::<String>))
                }
            })?,
        )?;
        turtle_table.set(
            "dig",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, _side: Option<String>| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    Ok(turtle
                        .dig_forward(TurtleSide::Right, &mut world)
                        .to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "digUp",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, _side: Option<String>| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    Ok(turtle.dig_up(TurtleSide::Right, &mut world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "select",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, slot: i32| {
                    let mut turtle = state.turtle.borrow_mut();

                    Ok(turtle.select((slot - 1) as usize))
                }
            })?,
        )?;
        turtle_table.set(
            "digDown",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, _side: Option<String>| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    Ok(turtle
                        .dig_down(TurtleSide::Right, &mut world)
                        .to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "place",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, text: Option<String>| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    Ok(turtle.place_forward(text, &mut world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "placeUp",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, text: Option<String>| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    Ok(turtle.place_up(text, &mut world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "placeDown",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, text: Option<String>| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    Ok(turtle.place_down(text, &mut world).to_lua_result())
                }
            })?,
        )?;
        turtle_table.set(
            "inspect",
            self.lua.create_function({
                let state = self.state.clone();
                move |lua, ()| {
                    let turtle = state.turtle.borrow();
                    let world = state.world.borrow();

                    let (has_block, data) = turtle.inspect_forward(&world).to_lua_result();

                    Ok((has_block, lua.to_value(&data)?))
                }
            })?,
        )?;
        turtle_table.set(
            "inspectUp",
            self.lua.create_function({
                let state = self.state.clone();
                move |lua, ()| {
                    let turtle = state.turtle.borrow();
                    let world = state.world.borrow();

                    let (has_block, data) = turtle.inspect_up(&world).to_lua_result();

                    Ok((has_block, lua.to_value(&data)?))
                }
            })?,
        )?;
        turtle_table.set(
            "inspectDown",
            self.lua.create_function({
                let state = self.state.clone();
                move |lua, ()| {
                    let turtle = state.turtle.borrow();
                    let world = state.world.borrow();

                    let (has_block, data) = turtle.inspect_down(&world).to_lua_result();

                    Ok((has_block, lua.to_value(&data)?))
                }
            })?,
        )?;
        turtle_table.set(
            "getItemDetail",
            self.lua.create_function({
                let state = self.state.clone();
                move |lua, (slot, detailed): (Option<i32>, bool)| {
                    let turtle = state.turtle.borrow();

                    let slot = slot
                        .map(|slot| (slot - 1) as usize)
                        .unwrap_or(turtle.selected_slot);

                    let detail = turtle.get_item_detail(slot, detailed);

                    detail.map(|detail| lua.to_value(&detail)).transpose()
                }
            })?,
        )?;

        globals.set("turtle", turtle_table)?;

        Ok(())
    }

    fn read_lua_file(&self, path: impl AsRef<Path>) -> SimulatorResult<String> {
        let path = self.state.current_dir.borrow().join(path);
        let content = std::fs::read_to_string(path)?;

        Ok(content)
    }

    pub fn exec_lua(&self, code: &str) -> SimulatorResult<()> {
        Ok(self.lua.load(code).exec()?)
    }

    pub fn exec_lua_file(&self, path: impl AsRef<Path>) -> SimulatorResult<()> {
        let code = self.read_lua_file(path)?;
        self.exec_lua(&code)
    }

    pub fn eval_lua<'a, R>(&'a self, code: &str) -> SimulatorResult<R>
    where
        R: mlua::FromLuaMulti<'a>,
    {
        Ok(self.lua.load(code).eval()?)
    }

    pub fn eval_lua_file<'a, R>(&'a self, path: impl AsRef<Path>) -> SimulatorResult<R>
    where
        R: mlua::FromLuaMulti<'a>,
    {
        let code = self.read_lua_file(path)?;
        self.eval_lua(&code)
    }

    pub fn call_lua<'a, A, R>(&'a self, code: &str, args: A) -> SimulatorResult<R>
    where
        A: mlua::IntoLuaMulti<'a>,
        R: mlua::FromLuaMulti<'a>,
    {
        Ok(self.lua.load(code).call(args)?)
    }

    pub fn call_lua_file<'a, A, R>(&'a self, path: impl AsRef<Path>, args: A) -> SimulatorResult<R>
    where
        A: mlua::IntoLuaMulti<'a>,
        R: mlua::FromLuaMulti<'a>,
    {
        let code = self.read_lua_file(path)?;
        self.call_lua(&code, args)
    }
}

pub trait TurtleResultExt<T> {
    fn to_lua_result(self) -> (bool, T);
}

impl TurtleResultExt<Option<String>> for Result<(), TurtleMoveError> {
    fn to_lua_result(self) -> (bool, Option<String>) {
        match self {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string())),
        }
    }
}

#[derive(Serialize)]
enum InspectDataOrReason {
    Data(InspectData),
    Reason(String),
}

impl TurtleResultExt<InspectDataOrReason> for Result<InspectData, TurtleInspectError> {
    fn to_lua_result(self) -> (bool, InspectDataOrReason) {
        match self {
            Ok(data) => (true, InspectDataOrReason::Data(data)),
            Err(err) => (false, InspectDataOrReason::Reason(err.to_string())),
        }
    }
}

impl TurtleResultExt<Option<String>> for Result<(), TurtleDigError> {
    fn to_lua_result(self) -> (bool, Option<String>) {
        match self {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string())),
        }
    }
}

impl TurtleResultExt<Option<String>> for Result<(), TurtlePlaceError> {
    fn to_lua_result(self) -> (bool, Option<String>) {
        match self {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string())),
        }
    }
}

pub struct SimulatorState {
    current_dir: RefCell<PathBuf>,
    world: RefCell<World>,
    turtle: RefCell<Turtle>,
}

impl SimulatorState {
    pub fn new() -> Self {
        Self {
            current_dir: RefCell::new(PathBuf::new()),
            world: RefCell::new(World::new()),
            turtle: RefCell::new(Turtle::new(
                Position::new(0, 0, 0),
                Direction::North,
                TurtleKind::Advanced,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use minecraft::{ItemId, blocks};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_turtle_movement() {
        let simulator = Simulator::new().unwrap();

        simulator.exec_lua("turtle.forward()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 0, -1)
        );

        simulator.exec_lua("turtle.back()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 0, 0)
        );

        simulator.exec_lua("turtle.up()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 1, 0)
        );

        simulator.exec_lua("turtle.down()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 0, 0)
        );

        simulator.exec_lua("turtle.turnRight()").unwrap();
        assert_eq!(simulator.state.turtle.borrow().facing, Direction::East);

        simulator.exec_lua("turtle.forward()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(1, 0, 0)
        );

        simulator.exec_lua("turtle.turnLeft()").unwrap();
        assert_eq!(simulator.state.turtle.borrow().facing, Direction::North);

        simulator.exec_lua("turtle.forward()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(1, 0, -1)
        );
    }

    #[test]
    fn test_turtle_dig() {
        let simulator = Simulator::new().unwrap();

        simulator.turtle_mut().set_upgrade(
            TurtleSide::Right,
            Some(ItemId::new_static("minecraft:diamond_pickaxe")),
        );

        simulator.set_block_at(simulator.turtle().looking_at(), blocks::AIR.clone());
        let result: (bool, Option<String>) = simulator.eval_lua("turtle.dig()").unwrap();
        assert_eq!(result, (false, Some("Nothing to dig here".to_string())));

        simulator.set_block_at(simulator.turtle().looking_at(), blocks::BEDROCK.clone());
        let result: (bool, Option<String>) = simulator.eval_lua("turtle.dig()").unwrap();
        assert_eq!(
            result,
            (false, Some("Cannot break unbreakable block".to_string()))
        );

        simulator.set_block_at(simulator.turtle().looking_at(), blocks::STONE.clone());
        let result: (bool, Option<String>) = simulator.eval_lua("turtle.dig()").unwrap();
        assert_eq!(result, (true, None));
    }
}
