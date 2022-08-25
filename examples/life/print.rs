use toucan_ecs::prelude::*;

use crate::field::{Alive, Field, Point};

type PrintQuery<'a> = (&'a Point, Option<&'a Alive>);

pub fn print_field<'a>(data: View<'a, PrintQuery<'a>>, field: Res<'a, Field>) {
    let width = field.width();
    for (point, alive) in data {
        print!("{}", alive.map(|_| 'X').unwrap_or('O'));
        if point.y == width - 1 {
            println!();
        }
    }
}
