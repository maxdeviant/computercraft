use std::cell::RefCell;
use std::rc::Rc;

use minecraft::world::{Direction, Position, World};
use mlua::Lua;
use thiserror::Error;

use crate::{Turtle, TurtleType};

#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("Lua error: {0}")]
    LuaError(#[from] mlua::Error),
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

        this.init_turtle_api()?;

        Ok(this)
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
                    let mut world = state.world.borrow_mut();

                    let (success, err) = turtle.forward(&mut world);

                    Ok((success, err))
                }
            })?,
        )?;
        turtle_table.set(
            "back",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    let (success, err) = turtle.back(&mut world);

                    Ok((success, err))
                }
            })?,
        )?;
        turtle_table.set(
            "up",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    let (success, err) = turtle.up(&mut world);

                    Ok((success, err))
                }
            })?,
        )?;
        turtle_table.set(
            "down",
            self.lua.create_function({
                let state = self.state.clone();
                move |_lua, ()| {
                    let mut turtle = state.turtle.borrow_mut();
                    let mut world = state.world.borrow_mut();

                    let (success, err) = turtle.down(&mut world);

                    Ok((success, err))
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
}

pub struct SimulatorState {
    world: RefCell<World>,
    turtle: RefCell<Turtle>,
}

impl SimulatorState {
    pub fn new() -> Self {
        Self {
            world: RefCell::new(World::new()),
            turtle: RefCell::new(Turtle::new(
                Position::new(0, 0, 0),
                Direction::North,
                TurtleType::Advanced,
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
    }
}
