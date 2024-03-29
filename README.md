# toucan_ecs

[![Crate](https://img.shields.io/crates/v/toucan_ecs.svg)](https://crates.io/crates/toucan_ecs)
[![Docs](https://docs.rs/toucan_ecs/badge.svg)](https://docs.rs/toucan_ecs)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg)

Simple and safe ECS library for Rust.

Provides basic features, such as:

- create and destroy entities;
- attach, get or remove components from the entity;
- use entry of the entity to modify it;
- view components of different types immutably or mutably;
- use systems to get and update data efficiently.

## Feature flags

This crate has the following Cargo features:

| Feature name | Description                                |
|--------------|--------------------------------------------|
| `resource`   | Store resources in the world and view them |

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
