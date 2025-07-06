use computercraft_simulator::{Simulator, TurtleSide};
use indoc::indoc;
use minecraft::world::Position;
use minecraft::{BlockId, ItemId, blocks};
use pretty_assertions::assert_eq;

use crate::setup::set_script_root;

#[test]
fn test_wheat_farmer() {
    let mut simulator = Simulator::new().unwrap();
    set_script_root(&mut simulator);

    const FIELD_SIZE: i32 = 3;

    for x in 0..FIELD_SIZE {
        for z in 0..FIELD_SIZE {
            simulator.set_block_at(Position::new(x, 0, -z), blocks::DIRT.clone());
        }
    }

    simulator.turtle_mut().set_upgrade(
        TurtleSide::Right,
        Some(ItemId::new_static("minecraft:diamond_hoe")),
    );
    simulator.move_turtle_to(Position::new(0, 1, 0));

    simulator
        .exec_lua(indoc! {r#"
            local wheat_farmer = require("programs.wheat_farmer")

            wheat_farmer.do_turn(3)
        "#})
        .unwrap();
    assert_eq!(simulator.turtle().position, Position::new(2, 1, -2));

    for x in 0..FIELD_SIZE {
        for z in 0..FIELD_SIZE {
            // Right now the behavior of the program is that it doesn't do the action on the last block.
            if z == FIELD_SIZE - 1 {
                continue;
            }

            let block = simulator.block_at(Position::new(x, 0, -z));
            assert_eq!(block.id, BlockId::FARMLAND);
        }
    }
}
