# `bevy_dogoap` - Integration of `dogoap` into Bevy's ECS model
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/victorb/dogoap#License)
[![Crates.io](https://img.shields.io/crates/v/bevy_dogoap.svg)](https://crates.io/crates/bevy_dogoap)
[![Downloads](https://img.shields.io/crates/d/bevy_dogoap.svg)](https://crates.io/crates/bevy_dogoap)
[![Docs](https://docs.rs/bevy_dogoap/badge.svg)](https://docs.rs/bevy_dogoap/latest/bevy_dogoap/)
[![ci](https://github.com/victorb/dogoap/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/victorb/dogoap/actions/workflows/ci.yml)

Full Bevy + `bevy_dogoap` example:

```rust
use bevy::prelude::*;
use bevy_dogoap::prelude::*;

// Define a DatumComponent as a struct with one member that can be either bool, f64 or u64
#[derive(Component, Clone, DatumComponent)]
struct IsHungry(bool);

// Define a ActionComponent as a struct without any fields
#[derive(Component, Reflect, Clone, Default, ActionComponent)]
struct EatAction;

fn startup(mut commands: Commands) {
    // Set our goal to be that we shouldn't be hungry
    let goal = Goal::from_reqs(&[IsHungry::is(false)]);

    // Create our action from EatAction
    let eat_action = EatAction::new()
        // Mutators define what happens when this action is executed
        // In this case, we set IsHungry to false
        .add_mutator(IsHungry::set(false));

    // Create our planner + required DatumComponents
    let (planner, components) = create_planner!({
        actions: [(EatAction, eat_action)],
        // We put our starting state to be IsHungry = true
        state: [IsHungry(true)],
        goals: [goal],
    });

    // Spawn a entity with our Planner component + the DatumComponents
    commands.spawn((Name::new("Planner"), planner, components));
}

// Create our system that handles executing of the EatAction
fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut IsHungry)>,
) {
    for (entity, _eat_action, mut need) in query.iter_mut() {
        // We set IsHungry to false
        need.0 = false;
        // And remove the action from our entity as we're done with this action
        commands.entity(entity).remove::<EatAction>();
    }
}

fn main() {
    let mut app = App::new();

    // We need to register our components as DatumComponent, otherwise planner won't be able to find them
    // as we're using bevy_trait_query to be able to query for components implementing a trait
    register_components!(app, vec![IsHungry]);

    app.add_plugins(MinimalPlugins)
        // !!! Don't forget to add the plugin ;)
       .add_plugins(DogoapPlugin)
       .add_systems(Startup, startup)
       .add_systems(FixedUpdate, handle_eat_action);

    // Run a couple of updates to run forward the world
    for _i in 0..3 {
        app.update();
    }

    // Lets inspect the final state of our (one) Planner we have in the World
    let mut query = app.world_mut().query::<&Planner>();
    let planner = query.get_single(&app.world()).unwrap();

    // This should confirm that is_hungry and is_tired have been set to `false`
    println!("Final state in our planner:");
    println!("{:#?}", planner.state);
}
```

With this example, it should take about 2-3 frames until IsHungry is now set to `false` as the planner came up with a plan, added the EatAction component, our system handled the action and changed the DatumComponent

### More Examples

- [`bevy_basic.rs`](./examples/bevy_basic.rs) - Quickstart - Basic setup possible for integration between `dogoap` and Bevy
- [`miner.rs`](./examples/miner.rs) - Long plans - How to setup more complicated interactions
- [`sneaky.rs`](./examples/sneaky.rs) - Player Interactions - How to use `dogoap` for NPCs in a world with a player controlled entity
- [`villages.rs`](./examples/villages.rs) - Nested Planners - How you can nest planners to achieve something smarter
- [`lemonade_stand.rs`](./examples/lemonade_stand.rs) - Co-operating Planners - Shows how you can have two entities with two very different planners and "simulate" co-operation

# Bevy Version Support

| bevy | bevy_dogoap |
| ---- | ----------- |
| 0.14 | 0.2.0       |
