use components::{Mass, Position, Velocity};

mod components;
mod utils;

#[test]
fn view_one_mut() {
    let mut registry = utils::prepare_for_view();

    for (entity, mut component) in registry.view_one_mut::<Position>() {
        component.x -= 10.0;
        println!("entity: {:?}, component: {:?}", entity, *component)
    }
}

#[test]
fn view_mut() {
    let mut registry = utils::prepare_for_view();

    for (entity, (mut position, velocity, mut mass)) in
        registry.view_mut::<(&mut Position, &Velocity, &mut Mass)>()
    {
        position.x -= 10.0;
        mass.0 += 1.0;
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity, *position, *velocity, *mass,
        )
    }
}

#[test]
fn option_view_mut() {
    let mut registry = utils::prepare_for_optional_view();

    for (entity, (mut position, velocity, mut mass)) in
        registry.view_mut::<(&mut Position, Option<&Velocity>, Option<&mut Mass>)>()
    {
        position.x -= 10.0;
        if let Some(ref mut mass) = mass {
            mass.0 += 1.0;
        }
        println!(
            "entity: {:?}, position: {:?}, velocity: {:?}, mass: {:?}",
            entity,
            *position,
            velocity.as_deref(),
            mass.as_deref(),
        )
    }
}
