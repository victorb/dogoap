# `dogoap` - Data-Oriented, Goal-Oriented Action Planning
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/victorb/dogoap#License)
[![Crates.io](https://img.shields.io/crates/v/dogoap.svg)](https://crates.io/crates/dogoap)
[![Downloads](https://img.shields.io/crates/d/dogoap.svg)](https://crates.io/crates/dogoap)
[![Docs](https://docs.rs/dogoap/badge.svg)](https://docs.rs/dogoap/latest/dogoap/)
[![ci](https://github.com/victorb/dogoap/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/victorb/dogoap/actions/workflows/ci.yml)
> GOAP implemented in data-oriented way to facilitate dynamically setting up states/actions/goals rather than statically

> This is a standalone Rust library for doing GOAP declaratively.

## Pseudo-example

Given a current state like this:

```
is_hungry = true
```

And with the following available actions:

```
eat_action = Mutator::set("is_hungry", false)
```

And with a goal like this:

```
is_hungry = false
```

The magical planner can figure out the best path of actions to reach that goal. In this case, the best plan is just one `eat_action` which results in the `is_hungry` state variable to be set to `false`, a very simplistic example.

That's the basics. Of course, things get more interesting once we start to include preconditions, multiple actions and multiple state variables, and the planner needs to figure out the right way of reaching there.

## Code / API example

This is the actual API of the library:

```rust

```

