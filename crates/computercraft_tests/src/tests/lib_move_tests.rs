use computercraft_simulator::Simulator;
use indoc::indoc;
use minecraft::world::{Direction, Position};
use pretty_assertions::assert_eq;

fn script_root() -> String {
    format!("{}/../..", env!("CARGO_MANIFEST_DIR"))
}

fn set_script_root(simulator: &mut Simulator) {
    simulator.set_current_dir(script_root());
}

#[test]
fn test_move_functions() {
    let mut simulator = Simulator::new().unwrap();
    set_script_root(&mut simulator);

    simulator
        .run_lua(indoc! {r#"
            local move = require "lib.move"

            move.forward(3)
        "#})
        .unwrap();

    assert_eq!(simulator.turtle().position, Position::new(0, 0, -3));
}

#[test]
fn test_traverse_plane() {
    let mut simulator = Simulator::new().unwrap();
    set_script_root(&mut simulator);

    simulator
        .run_lua(indoc! {r#"
            local move = require "lib.move"

            move.traverse_plane(3, 3, function()
            end)
        "#})
        .unwrap();
    assert_eq!(simulator.turtle().position, Position::new(2, 0, -2));
    assert_eq!(simulator.turtle().direction, Direction::South);
    assert_eq!(
        simulator.turtle().position_history,
        vec![
            Position::new(0, 0, 0),
            Position::new(0, 0, -1),
            Position::new(0, 0, -2),
            Position::new(1, 0, -2),
            Position::new(1, 0, -1),
            Position::new(1, 0, 0),
            Position::new(2, 0, 0),
            Position::new(2, 0, -1),
            Position::new(2, 0, -2),
        ]
    );
}
