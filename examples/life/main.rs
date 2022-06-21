use std::error::Error;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use toucan_ecs::World;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut _world = World::new();

    print!("Hello there");
    print!("\r");
    stdout().flush()?;
    sleep(Duration::from_millis(500));

    println!("Hello there again");
    todo!()
}
