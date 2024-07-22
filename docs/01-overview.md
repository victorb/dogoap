# Overview

> TODO names in flux

## Glossary

The `dogoap` API is for the standalone Rust library, it has nothing to do with Bevy.

The `bevy_dogoap` API however is all about Bevy, and integrates `dogoap`.

## `dogoap` API

In order of appearance in a basic example.

### `LocalState`

This represents someone's/something's local state that they are aware of. This is considered the "starting state" for the planner when trying to figure out what actions to take to reach the Goal state.

### `Field`

A Field represents a type + value.

### `Goal`

A Goal is the final state we want the planner to plan for.

### `Compare`

A Compare is used in Preconditions and Goals to indicate what we want a Field in our LocalState to be. `Compare::GreaterThan(Field::I64(10))` would mean we're looking to have a i64 result that is greater than 10. 

### `Action`

Action is built to be able to tell the planner what they could do to reach the final Goal state. The Action has Preconditions and Effects.

#### `Precondition`

The Precondition tells the planner what the LocalState must look like before the Planner could take this Action.

#### `Effect`

#### `Mutator`

#### `make_plan`

##### Plan nodes `Node::Effect` and `Node::State`

## `bevy_dogoap` API

### `LocalFieldComponent`

Currently, has to be a struct with arity 1, where argument is either a `bool`, `int64`, `f64` or `usize`

### `ActionComponent`

### `BevyPlanner`