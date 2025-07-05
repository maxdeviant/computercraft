use computercraft_simulator::{Turtle, TurtleKind};
use minecraft::Block;
use minecraft::world::{Direction, Position, World};

fn main() {
    // Create a new world
    let mut world = World::new();

    // Add some blocks to the world
    world.set_block(Position::new(0, 0, -2), Block::Stone);
    world.set_block(Position::new(1, 0, -2), Block::Dirt);
    world.set_block(Position::new(2, 0, -2), Block::Wood);
    world.set_block(Position::new(0, 1, -1), Block::Cobblestone);

    // Create a turtle at origin facing north
    let mut turtle = Turtle::new(Position::new(0, 0, 0), Direction::North, TurtleKind::Normal);

    println!("Starting turtle simulation...");
    println!("Turtle position: {:?}", turtle.position);
    println!("Turtle direction: {:?}", turtle.direction);
    println!(
        "Turtle fuel: {}/{}",
        turtle.get_fuel_level(),
        turtle.get_fuel_limit()
    );

    // Move forward
    println!("\n--- Moving forward ---");
    let (success, error) = turtle.forward(&mut world);
    if success {
        println!("✓ Moved forward to {:?}", turtle.position);
    } else {
        println!(
            "✗ Failed to move forward: {}",
            error.unwrap_or("Unknown error".to_string())
        );
    }

    // Try to move forward again (should hit the stone block)
    println!("\n--- Moving forward again (should hit stone) ---");
    let (success, error) = turtle.forward(&mut world);
    if success {
        println!("✓ Moved forward to {:?}", turtle.position);
    } else {
        println!(
            "✗ Failed to move forward: {}",
            error.unwrap_or("Unknown error".to_string())
        );
    }

    // Detect the block in front
    println!("\n--- Detecting block in front ---");
    if turtle.detect(&world) {
        println!("✓ Detected solid block in front");
    } else {
        println!("✗ No solid block detected in front");
    }

    // Dig the block
    println!("\n--- Digging block in front ---");
    let (success, error) = turtle.dig(&mut world);
    if success {
        println!("✓ Successfully dug block");
    } else {
        println!(
            "✗ Failed to dig: {}",
            error.unwrap_or("Unknown error".to_string())
        );
    }

    // Try to move forward again (should succeed now)
    println!("\n--- Moving forward after digging ---");
    let (success, error) = turtle.forward(&mut world);
    if success {
        println!("✓ Moved forward to {:?}", turtle.position);
    } else {
        println!(
            "✗ Failed to move forward: {}",
            error.unwrap_or("Unknown error".to_string())
        );
    }

    // Turn right and move
    println!("\n--- Turning right and moving ---");
    let (success, _) = turtle.turn_right();
    if success {
        println!("✓ Turned right, now facing {:?}", turtle.direction);
    }

    let (success, error) = turtle.forward(&mut world);
    if success {
        println!("✓ Moved forward to {:?}", turtle.position);
    } else {
        println!(
            "✗ Failed to move forward: {}",
            error.unwrap_or("Unknown error".to_string())
        );
    }

    // Detect and dig the block in front
    println!("\n--- Detecting and digging block in front ---");
    if turtle.detect(&world) {
        println!("✓ Detected solid block in front");
        let (success, error) = turtle.dig(&mut world);
        if success {
            println!("✓ Successfully dug block");
        } else {
            println!(
                "✗ Failed to dig: {}",
                error.unwrap_or("Unknown error".to_string())
            );
        }
    } else {
        println!("✗ No solid block detected in front");
    }

    // Check what's above
    println!("\n--- Checking above ---");
    if turtle.detect_up(&world) {
        println!("✓ Detected solid block above");
        let (success, error) = turtle.dig_up(&mut world);
        if success {
            println!("✓ Successfully dug block above");
        } else {
            println!(
                "✗ Failed to dig above: {}",
                error.unwrap_or("Unknown error".to_string())
            );
        }
    } else {
        println!("✗ No solid block detected above");
    }

    // Try to move up
    println!("\n--- Moving up ---");
    let (success, error) = turtle.up(&mut world);
    if success {
        println!("✓ Moved up to {:?}", turtle.position);
    } else {
        println!(
            "✗ Failed to move up: {}",
            error.unwrap_or("Unknown error".to_string())
        );
    }

    // Final status
    println!("\n--- Final Status ---");
    println!("Final position: {:?}", turtle.position);
    println!("Final direction: {:?}", turtle.direction);
    println!(
        "Fuel remaining: {}/{}",
        turtle.get_fuel_level(),
        turtle.get_fuel_limit()
    );

    // Demonstrate inventory operations
    println!("\n--- Inventory Operations ---");
    println!("Current selected slot: {}", turtle.get_selected_slot());
    println!("Items in slot 0: {}", turtle.get_item_count(Some(0)));
    println!("Space in slot 0: {}", turtle.get_item_space(Some(0)));

    let (success, _) = turtle.select(5);
    if success {
        println!("✓ Selected slot 5");
        println!("Current selected slot: {}", turtle.get_selected_slot());
    }
}
