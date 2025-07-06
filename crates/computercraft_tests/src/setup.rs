use computercraft_simulator::Simulator;

fn script_root() -> String {
    format!("{}/../..", env!("CARGO_MANIFEST_DIR"))
}

pub fn set_script_root(simulator: &mut Simulator) {
    simulator.set_current_dir(script_root());
}
