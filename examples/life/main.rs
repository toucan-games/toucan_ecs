use std::io::{stdin, stdout, BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

use crossterm::cursor::MoveToPreviousLine;
use crossterm::ExecutableCommand;

use field::{Alive, Field, Point, WatchAfter};
use toucan_ecs::{Entity, World};

mod field;
mod utils;

type Error = Box<dyn std::error::Error + 'static>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut world = World::new();

    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    print!("Enter field width: ");
    stdout.flush()?;
    let field_width = utils::read_and_parse(&mut stdin)?;
    let field = Field::new(field_width);

    println!("Preparing for field generation...");
    world.create_resource(field);
    world.register::<Point>();
    world.register::<Alive>();
    world.register::<WatchAfter>();

    println!("Generating the field...");
    let range: Vec<_> = (0..field_width).collect();
    let entities = range
        .iter()
        .map(|x| {
            range
                .iter()
                .map(|y| (Point { x: *x, y: *y }, Alive { alive: false }))
        })
        .flatten();
    world.extend_with(entities);

    println!("Field generation was completed!");
    for (entity, point) in world.view::<(Entity, &Point)>() {
        println!("entity: {:?}, point: {:?}", entity, point);
    }
    Ok(())
}
