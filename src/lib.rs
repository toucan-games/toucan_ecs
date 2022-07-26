#![warn(missing_docs)]
#![warn(
    clippy::disallowed_types,
    clippy::undocumented_unsafe_blocks,
    clippy::missing_safety_doc
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Simple and safe ECS library for Rust.
//!
//! Provides basic features, such as:
//! - create and destroy entities;
//! - attach, get or remove components from the entity;
//! - use [entry](crate::world::Entry) of the entity to modify it;
//! - view components of different types
//! [immutably][crate::world::World::view()] or [mutably][crate::world::World::view_mut()];
//! - use [systems](crate::system::System) to get and update data efficiently.
//!
//! # Examples
//!
//! ## Create and destroy entities
//!
//! ```
//! use toucan_ecs::world::World;
//!
//! let mut world = World::new();
//!
//! let entity = world.create();
//! assert!(world.contains(entity));
//!
//! world.destroy(entity);
//! assert!(!world.contains(entity));
//! ```
//!
//! ## Use entries to simplify access to the entity's data
//!
//! ```
//! use toucan_ecs::component::Component;
//! use toucan_ecs::world::World;
//!
//! #[derive(Copy, Clone, Component)]
//! struct Name(&'static str);
//!
//! #[derive(Copy, Clone, Component)]
//! struct ID(u32);
//!
//! let mut world = World::new();
//!
//! // Create new entity
//! let entity = {
//!     let mut entry = world.create_entry();
//!     entry.attach((Name("Hello, World"), ID(42)));
//!     assert!(entry.attached::<(Name, ID)>());
//!     entry.entity()
//! };
//! assert!(world.attached::<(Name, ID)>(entity));
//!
//! // Or reuse existing ones
//! if let Some(mut entry) = world.entry(entity) {
//!     entry.remove::<ID>();
//! }
//! assert!(!world.attached::<ID>(entity));
//! ```
//!
//! ## View components with ease
//!
//! ```
//! use toucan_ecs::component::Component;
//! use toucan_ecs::entity::Entity;
//! use toucan_ecs::world::World;
//!
//! #[derive(Debug, Copy, Clone, Component)]
//! struct Position {
//!     x: f32,
//!     y: f32,
//! }
//!
//! #[derive(Debug, Copy, Clone, Component)]
//! struct Mass(f32);
//!
//! let mut world = World::new();
//!
//! // Create our entities and their data
//! for i in 0..10 {
//!     let f = i as f32;
//!     let position = Position { x: f / 10.0, y: -f / 10.0 };
//!     let entity = world.create_with((position,));
//!     assert!(world.attached::<Position>(entity));
//!     if i % 2 != 0 {
//!         let mass = Mass(f);
//!         world.attach(entity, mass);
//!         assert!(world.attached::<Mass>(entity));
//!     }
//! }
//!
//! // Get all entities which have `Position` and may have `Mass` components
//! for (_, mut position, mass) in world.view_mut::<(Entity, &mut Position, Option<&Mass>)>() {
//!     position.x += 1.0;
//!     println!("position is {:?}, mass is {:?}", position, mass.as_deref());
//! }
//! ```
//!
//! ## Use systems to get and update data
//!
//! ```
//! use toucan_ecs::component::Component;
//! use toucan_ecs::entity::Entity;
//! use toucan_ecs::world::World;
//! use toucan_ecs::system::Schedule;
//! use toucan_ecs::resource::marker::Resource;
//!
//! #[derive(Copy, Clone, Component)]
//! struct Name(&'static str);
//!
//! #[derive(Copy, Clone, Component)]
//! struct ID(u32);
//!
//! // notice no `Copy` and `Clone`
//! struct MyResource(i32);
//!
//! let mut world = World::new();
//! // create new resource of this world
//! world.create_resource(MyResource(128));
//!
//! let mut schedule = Schedule::builder()
//!     .system(|| println!("Hello, World"))
//!     // this system will be executed once
//!     .system(|resource: Resource<MyResource>| println!("Resource value: {}", resource.0))
//!     // this system will be executed for each entity with `Name` and `ID` components
//!     .foreach_system(|name: &Name, id: &mut ID| {
//!         id.0 += 100;
//!         println!("Changed ID: {}", id.0);
//!     })
//!     .build();
//!
//! // execute all the systems in schedule
//! schedule.run(&mut world);
//! ```

#[macro_use]
mod macros;
mod error;
mod hash;
mod type_id;

pub mod component;
pub mod entity;
cfg_resource! {
    pub mod resource;
}
pub mod system;
pub mod world;
