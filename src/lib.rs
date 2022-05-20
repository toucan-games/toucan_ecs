//! # toucan_ecs
//!
//! Simple and safe ECS library for Rust.
//!
//! Provides basic features, such as:
//! - create and destroy entities;
//! - attach, get or remove components from the entity;
//! - use [entry][`crate::component::Entry`] of the entity to modify it;
//! - view components of different types;
//! - view components [immutably][`World::view`] or [mutably][`World::view_mut`].
//!
//! For now library provides nothing for systems (are responsible for logic).
//! You are free to create your own system!
//!
//! This crate contains no `unsafe` code.
//!
//! # Examples
//!
//! ## Create and destroy entities
//!
//! ```
//! use toucan_ecs::World;
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
//! use toucan_ecs::World;
//!
//! #[derive(Copy, Clone)]
//! struct Name(&'static str);
//!
//! #[derive(Copy, Clone)]
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
//!     entry.remove_one::<ID>();
//! }
//! assert!(!world.attached_one::<ID>(entity));
//! ```
//!
//! ## View components with ease
//!
//! ```
//! use toucan_ecs::{Entity, World};
//!
//! #[derive(Debug, Copy, Clone)]
//! struct Position {
//!     x: f32,
//!     y: f32,
//! }
//!
//! #[derive(Debug, Copy, Clone)]
//! struct Mass(f32);
//!
//! let mut world = World::new();
//!
//! // Create our entities and their data
//! for i in 0..10 {
//!     let f = i as f32;
//!     let position = Position { x: f / 10.0, y: -f / 10.0 };
//!     let entity = world.create_with((position,));
//!     assert!(world.attached_one::<Position>(entity));
//!     if i % 2 != 0 {
//!         let mass = Mass(f);
//!         world.attach_one(entity, mass);
//!         assert!(world.attached_one::<Mass>(entity));
//!     }
//! }
//!
//! // Get all entities which have `Position` and may have `Mass` components
//! for (_, mut position, mass) in world.view_mut::<(Entity, &mut Position, Option<&Mass>)>() {
//!     position.x += 1.0;
//!     println!("position is {:?}, mass is {:?}", position, mass.as_deref());
//! }
//! ```

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(clippy::disallowed_types)]
#![forbid(unsafe_code)]

pub use entity::Entity;
pub use world::World;

pub mod component;
pub mod entity;
#[cfg(feature = "resource")]
pub mod resource;
pub mod system;
pub mod world;
