use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use minecraft::world::{Direction, Position, World};
use mlua::Lua;
use thiserror::Error;

use crate::{Turtle, TurtleKind, TurtleMoveError};

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
        let lua = Lua::new();

        let mut this = Self {
            lua,
            state: Rc::new(SimulatorState::new()),
        };

        this.init_require()?;
        this.init_turtle_api()?;

        Ok(this)
    }

    pub fn turtle(&self) -> std::cell::Ref<'_, Turtle> {
        self.state.turtle.borrow()
    }

    pub fn set_current_dir(&mut self, root_dir: impl AsRef<Path>) {
        *self.state.current_dir.borrow_mut() = root_dir.as_ref().to_path_buf();
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

        globals.set("turtle", turtle_table)?;

        Ok(())
    }

    pub fn run_lua(&self, code: &str) -> SimulatorResult<mlua::Value<'_>> {
        let result = self.lua.load(code).eval()?;

        Ok(result)
    }

    pub fn run_lua_file(&self, path: impl AsRef<Path>) -> SimulatorResult<mlua::Value<'_>> {
        let path = self.state.current_dir.borrow().join(path);
        let content = std::fs::read_to_string(path)?;

        self.run_lua(&content)
    }
}

pub trait TurtleResultExt {
    fn to_lua_result(self) -> (bool, Option<String>);
}

impl<T> TurtleResultExt for Result<T, TurtleMoveError> {
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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_turtle_movement() {
        let simulator = Simulator::new().unwrap();

        simulator.run_lua("turtle.forward()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 0, -1)
        );

        simulator.run_lua("turtle.back()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 0, 0)
        );

        simulator.run_lua("turtle.up()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 1, 0)
        );

        simulator.run_lua("turtle.down()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(0, 0, 0)
        );

        simulator.run_lua("turtle.turnRight()").unwrap();
        assert_eq!(simulator.state.turtle.borrow().direction, Direction::East);

        simulator.run_lua("turtle.forward()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(1, 0, 0)
        );

        simulator.run_lua("turtle.turnLeft()").unwrap();
        assert_eq!(simulator.state.turtle.borrow().direction, Direction::North);

        simulator.run_lua("turtle.forward()").unwrap();
        assert_eq!(
            simulator.state.turtle.borrow().position,
            Position::new(1, 0, -1)
        );
    }
}
