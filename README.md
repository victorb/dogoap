# Data-Oriented GOAP (Goal-Oriented Action Planning)
> AKA DOGOAP - GOAP implemented in data-oriented way to facilitate dynamically setting up states/actions/goals rather than only at compile-time

> Includes bevy_dogoap which provides a neat Bevy integration of the dogoap library

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/victorb/dogoap#License)
[![Crates.io](https://img.shields.io/crates/v/dogoap.svg)](https://crates.io/crates/dogoap)
[![Downloads](https://img.shields.io/crates/d/dogoap.svg)](https://crates.io/crates/dogoap)
[![Docs](https://docs.rs/dogoap/badge.svg)](https://docs.rs/dogoap/latest/dogoap/)
[![ci](https://github.com/victorb/dogoap/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/victorb/dogoap/actions/workflows/ci.yml)

## Documentation

- [`dogoap`](./crates/dogoap/README.md) docs - Standalone library for creation actions, states and goals to be used with the provided planner
- [`bevy_dogoap`](./crates/bevy_dogoap/README.md) docs - Integration of the `dogoap` library into Bevy

## Prior Art / Other similar projects

- https://github.com/skyne98/soap - A lot of inspiration taken from this repository, biggest difference is the data-oriented structure that dogoap has
- https://github.com/dmackdev/bevy_goap - Native Bevy GOAP library, API interface isn't ideal though
- https://github.com/QueenOfSquiggles/bevy_htnp - Native Bevy HTN (Hierarchical Task Network) library

## License

MIT 2024 - Victor Bjelkholm
