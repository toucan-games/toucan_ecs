# toucan_ecs

[![Crate](https://img.shields.io/crates/v/toucan_ecs.svg)](https://crates.io/crates/toucan_ecs)
[![Docs](https://docs.rs/toucan_ecs/badge.svg)](https://docs.rs/toucan_ecs)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](/docs/LICENSE-APACHE-2.0)

Simple and safe ECS library for Rust.

Provides basic features, such as:

- create and destroy entities;
- attach, get or remove components from the entity;
- use entry of the entity to modify it;
- view components of different types immutably or mutably;
- use systems to get and update data efficiently.

## Feature flags

This library contains some feature flags:

| Feature name | Description                                |
|--------------|--------------------------------------------|
| `resource`   | Store resources in the world and view them |

## License

Licensed under either of

* Apache License, Version 2.0,
  ([docs/LICENSE-APACHE-2.0](/docs/LICENSE-APACHE-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([docs/LICENSE-MIT](/docs/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
