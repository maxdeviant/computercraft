use computercraft_simulator::Simulator;
use minecraft::world::Position;
use pretty_assertions::assert_eq;

use crate::setup::set_script_root;

#[test]
fn test_shaft_miner() {
    let mut simulator = Simulator::new().unwrap();
    set_script_root(&mut simulator);

    simulator
        .call_lua_file::<_, ()>("programs/shaft_miner.lua", (1, 3))
        .unwrap();
    assert_eq!(simulator.turtle().position, Position::new(2, -1, -2));

    simulator
        .call_lua_file::<_, ()>("programs/shaft_miner.lua", (1, 3))
        .unwrap();
    assert_eq!(simulator.turtle().position, Position::new(0, -2, -0));

    simulator
        .call_lua_file::<_, ()>("programs/shaft_miner.lua", (2, 3))
        .unwrap();
    assert_eq!(simulator.turtle().position, Position::new(0, -4, -0));
}

#[test]
fn test_shaft_miner_with_different_width_and_height_values() {
    let mut simulator = Simulator::new().unwrap();
    set_script_root(&mut simulator);

    simulator
        .call_lua_file::<_, ()>("programs/shaft_miner.lua", (1, 3, 5))
        .unwrap();
    assert_eq!(simulator.turtle().position, Position::new(2, -1, -4));
}
