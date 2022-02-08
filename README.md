# toucan_ecs

[![Crate](https://img.shields.io/crates/v/toucan_ecs.svg)](https://crates.io/crates/toucan_ecs)
[![Docs](https://docs.rs/toucan_ecs/badge.svg)](https://docs.rs/toucan_ecs)
![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)

Simple and safe ECS library for Rust.

Provides basic features, such as:

- create and destroy entities;
- attach, get or remove components from the entity;
- use entry of the entity to modify it;
- view components of different types;
- view components immutably or mutably.

For now library provides nothing for systems (are responsible for logic). You are free to create your own system!

This crate contains no `unsafe` code.

## Feature flags

This library contains some feature flags:

| Feature name | Description                                |
|--------------|--------------------------------------------|
| `resource`   | Store resources in the world and view them |
