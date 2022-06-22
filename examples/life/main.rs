use std::io::{stdin, stdout, Write};

use rand::{thread_rng, Rng};

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
    print!("Enter probability of cell to be initially alive: ");
    stdout.flush()?;
    let probability = utils::read_and_parse(&mut stdin)?;
    if probability < 0.0 || probability > 1.0 {
        let error = format!("Probability must be between 0 and 1, got {}", probability);
        return Err(error.into());
    }

    println!("Preparing for field generation...");
    world.register::<Point>();
    world.register::<Alive>();
    world.register::<WatchAfter>();
    let field = Field::new(field_width);
    world.create_resource(field);

    println!("Generating the field...");
    let range: Vec<_> = (0..field_width).collect();
    let entities = range
        .iter()
        .map(|x| {
            range.iter().map(|y| {
                let point = Point { x: *x, y: *y };
                let alive = thread_rng().gen_bool(probability);
                let alive = Alive { alive };
                (point, alive)
            })
        })
        .flatten();
    world.extend_with(entities);

    println!("Field generation was completed!");
    for (entity, point, alive) in world.view::<(Entity, &Point, &Alive)>() {
        println!(
            "entity: {:?}, point: {:?}, alive: {}",
            entity, point, alive.alive
        );
    }
    Ok(())
}
