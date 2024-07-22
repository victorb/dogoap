Allows you to do something like this:

```rust
let (planner, components) = create_planner!({
    goal: [[Hunger <= 50]],
    state: [[Hunger 75]],
    actions: {
        EatAction: {
            preconditions: [[Energy >= 50]],
            effects: [{mutators: [[Hunger - 10],
                                  [Location = Location::Outside]]}]}}});

// Insert planner + initial state field components
commands.entity(my_entity).insert(components, planner);
```