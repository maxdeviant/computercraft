use computercraft_simulator::Simulator;
use indoc::indoc;
use minecraft::world::Position;
use pretty_assertions::assert_eq;

#[test]
fn test_move_functions() {
    let mut simulator = Simulator::new().unwrap();
    simulator
        .set_root_dir(format!("{}/../..", env!("CARGO_MANIFEST_DIR")))
        .unwrap();

    simulator
        .run_lua(indoc! {r#"
            local move = require "lib.move"

            move.forward(3)
        "#})
        .unwrap();

    assert_eq!(simulator.turtle().position, Position::new(0, 0, -3));
}
