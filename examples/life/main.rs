use std::io::{stdin, stdout, Write};

use rand::{thread_rng, Rng};
use toucan_ecs::prelude::*;

use crate::field::{Alive, Field, Point, WatchAfter};
use crate::print::print_field;

mod field;
mod print;
mod utils;

fn main() -> utils::Result<()> {
    let mut world = World::new();

    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    print!("Enter field width: ");
    stdout.flush()?;
    let field_width = utils::read_and_parse(&mut stdin)?;
    print!("Enter probability of cell to be initially alive: ");
    stdout.flush()?;
    let probability = utils::read_and_parse(&mut stdin)?;
    if !(0.0..=1.0).contains(&probability) {
        let error = format!("Probability must be between 0 and 1, got {}", probability);
        return Err(error.into());
    }

    println!("Preparing for field generation...");
    world.register::<Point>();
    world.register::<Alive>();
    world.register::<WatchAfter>();
    let field = Field::new(field_width);
    world.create_resources(field);

    println!("Generating the field...");
    let range: Vec<_> = (0..field_width).collect();
    for &x in range.iter() {
        for &y in range.iter() {
            let mut builder = world.entity().with(Point { x, y });
            if thread_rng().gen_bool(probability) {
                builder = builder.with(Alive);
            }
            builder.build();
        }
    }
    drop(range);
    println!("Field generation was completed!");

    let mut schedule = Schedule::builder().system(print_field).build();
    schedule.run(&mut world);

    Ok(())
}
