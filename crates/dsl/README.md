Allows you to do something like this:

```rust
let (planner, components) = create_planner!({
    goal: [[Hunger <= 50.0]
           [Energy >= 50.0]
           [GoldAmount = 3]]
    state: [[Hunger 75.0]
            [Energy 25.0]
            [AtLocation Location::Outside]
            [HasOre false]
            [HasMetal false]
            [GoldAmount 0]]
    actions: {EatAction: {preconditions: [[Energy >= 50.0]
                                          [Location = Location::Mushroom]]
                          effects: [{cost: 2
                                     mutators: [[Hunger - 10.0]
                                                [Energy - 1.0]
                                                [Location = Location::Outside]]}]}
              SleepAction: {preconditions: [[Location = Location::House]]
                            effects: [{cost: 1
                                       mutators: [[Energy + 50.0]]}]}
              MineOreAction: {preconditions: [[Location = Location::Ore]
                                              [Energy >= 50]
                                              [Hunger <= 50]]
                              effects: [{cost: 3
                                         mutators: [[HasOre = true]
                                                    [Hunger - 30]
                                                    [Energy - 30]]}]}}});

// Insert planner + initial state field components
commands.entity(my_entity).insert((components, planner));
```