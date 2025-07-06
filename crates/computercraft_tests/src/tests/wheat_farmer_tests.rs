use computercraft_simulator::Simulator;
use indoc::indoc;
use minecraft::blocks;
use minecraft::world::Position;
use pretty_assertions::assert_eq;

use crate::setup::set_script_root;

#[test]
fn test_wheat_farmer() {
    let mut simulator = Simulator::new().unwrap();
    set_script_root(&mut simulator);

    for x in 0..3 {
        for z in 0..3 {
            simulator.set_block_at(Position::new(x, 0, z), blocks::DIRT.clone());
        }
    }

    simulator.move_turtle_to(Position::new(0, 1, 0));

    simulator
        .exec_lua(indoc! {r#"
            local wheat_farmer = require("programs.wheat_farmer")

            wheat_farmer.do_turn(3)
        "#})
        .unwrap();
    assert_eq!(simulator.turtle().position, Position::new(2, 1, -2));
}
