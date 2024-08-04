# `dogoap` - Data-Oriented, Goal-Oriented Action Planning
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

This is the API of the library:

```rust

```