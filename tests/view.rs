use components::{Mass, Position, Velocity};

mod components;
mod utils;

#[test]
fn view_one() {
    let registry = utils::prepare_for_view();

    for (entity, component) in registry.view_one::<Position>() {
        println!("entity: {:?}, component: {:?}", entity, *component)
    }
}

#[test]
fn view() {
    let registry = utils::prepare_for_view();

    for (entity, (position, velocity, mass)) in registry.view::<(&Position, &Velocity, &Mass)>() {
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity, *position, *velocity, *mass
        )
    }
}
