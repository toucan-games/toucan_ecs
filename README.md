[![Crate](https://img.shields.io/crates/v/toucan_ecs.svg)](https://crates.io/crates/toucan_ecs)
[![Docs](https://docs.rs/toucan_ecs/badge.svg)](https://docs.rs/toucan_ecs)
![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)

# toucan_ecs

Simple and safe ECS library for Rust.

Provides basic features, such as:

- create and destroy entities;
- attach, get or remove components from the entity;
- use entry of the entity to modify it;
- view components of different types;
- view components immutably or mutably.

For now library provides nothing for systems (are responsible for logic). You are free to create your own system!

This crate contains no `unsafe` code.

# Examples

## Create and destroy entities

```rust
use toucan_ecs::Registry;

let mut registry = Registry::new();

let entity = registry.create();
assert!(registry.contains(entity));

registry.destroy(entity);
assert!(!registry.contains(entity));
```

## Use entries to simplify access to the entity's data

```rust
use toucan_ecs::Registry;

#[derive(Copy, Clone)]
struct Name(&'static str);

#[derive(Copy, Clone)]
struct ID(u32);

let mut registry = Registry::new();

// Create new entity
let entity = {
    let mut entry = registry.create_entry();
    entry.attach((Name("Hello, World"), ID(42)));
    assert!(entry.attached::<(Name, ID)>());
    entry.entity()
};
assert!(registry.attached::<(Name, ID)>(entity));

// Or reuse existing ones
if let Some(mut entry) = registry.entry(entity) {
    entry.remove_one::<ID>();
}
assert!(!registry.attached_one::<ID>(entity));
```

## View components with ease

```rust
use toucan_ecs::{Entity, Registry};

#[derive(Debug, Copy, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
struct Mass(f32);

let mut registry = Registry::new();

// Create our entities and their data
for i in 0..10 {
    let f = i as f32;
    let position = Position { x: f / 10.0, y: -f / 10.0 };
    let entity = registry.create_with((position,));
    assert!(registry.attached_one::<Position>(entity));
    if i % 2 != 0 {
        let mass = Mass(f);
        registry.attach_one(entity, mass);
        assert!(registry.attached_one::<Mass>(entity));
    }
}

// Get all entities which have `Position` and CAN have `Mass` components
for (_, mut position, mass) in registry.view_mut::<(Entity, &mut Position, Option<&Mass>)>() {
    position.x += 1.0;
    println!("position is {:?}, mass is {:?}", *position, mass.as_deref());
}
```
