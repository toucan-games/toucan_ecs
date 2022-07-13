use toucan_ecs::resource::marker::Resource;
use toucan_ecs::world::view::View;

use crate::field::{Alive, Field, Point};

type PrintQuery<'a> = (&'a Point, Option<&'a Alive>);

pub fn print_field<'a>(data: View<'a, PrintQuery<'a>>, field: Resource<'a, Field>) {
    let width = field.width();
    for (point, alive) in data {
        print!("{}", alive.map(|_| 'X').unwrap_or('O'));
        if point.y == width - 1 {
            println!();
        }
    }
}
