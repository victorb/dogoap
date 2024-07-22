// This is a really basic example of how to use dogoap with bevy_dogoap for use in Bevy
// It sets up two states on a entity, is_hungry and is_tired
// Then it sets up two possible actions:
// - SleepAction, which sets is_tired to false
// - EatAction, which requires is_tired to be false, and sets is_hungry to true

// We run three app.update() instead of app.run(), then print the final state after
// those three updates, so there is some sort of end to it all...

// Final state should be that is_hungry is false, and is_tired is false

use bevy::{log::LogPlugin, prelude::*};

// bevy_dogoap exposes a `prelude` that contains everything you might need
use bevy_dogoap::prelude::*;

// This is a DatumComponent that can hold one value
#[derive(Component, Clone, DatumComponent)]
struct IsHungry(bool);

#[derive(Component, Clone, DatumComponent)]
struct IsTired(bool);

// This is our ActionComponent that gets added whenever the planner thinks
// the entity need to perform the action in order to reach the goal
#[derive(Component, Reflect, Clone, Default, ActionComponent)]
struct EatAction;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
struct SleepAction;

fn startup(mut commands: Commands) {
    // This is the goal we want the planner to help us reach
    let goal = Goal::from_reqs(&[IsHungry::is(false), IsTired::is(false)]);

    // Our first action, the eat action, that sets is_hungry to false
    // but requires is_tired to be set to false first
    let eat_action = EatAction::new()
        .add_precondition(IsTired::is(false))
        .add_mutator(IsHungry::set(false));

    // Our first action, the sleep action, that sets is_tired to false
    // No preconditions in order to sleep
    let sleep_action = SleepAction::new().add_mutator(IsTired::set(false));

    // This create_planner! macro doesn't yet exists
    let (planner, components) = create_planner!({
        actions: [
            (EatAction, eat_action),
            (SleepAction, sleep_action)
        ],
        state: [IsHungry(true), IsTired(true)],
        goals: [goal],
    });

    // To spawn the planner, we first create a new entity
    // You can also use an existing entity if you have one at hand
    // This will usually be whatever Entity you use for your NPC
    commands.spawn((Name::new("Planner"), planner, components));
}

// These two systems are regular Bevy systems. Looks for Entities that have
// both EatAction and IsHungry, and when it finds one, replace the IsHungry
// component with one that says `false`
fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut IsHungry)>,
) {
    for (entity, _eat_action, mut need) in query.iter_mut() {
        *need = IsHungry(false);
        commands.entity(entity).remove::<EatAction>();
        info!("IsHungry been set to false and removed EatAction");
    }
}

// Same goes for the sleep_action, but with SleepAction and IsTired
fn handle_sleep_action(
    mut commands: Commands,
    mut query: Query<(Entity, &SleepAction, &mut IsTired)>,
) {
    for (entity, _sleep_action, mut need) in query.iter_mut() {
        *need = IsTired(false);
        commands.entity(entity).remove::<SleepAction>();
        info!("IsTired been set to false and removed SleepAction");
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Make sure to include the DogoapPlugin which manages the planner for us
    app.add_plugins(DogoapPlugin);

    // We configure the logplugin to output debug info for both dogoap and bevy_dogoap
    app.add_plugins(LogPlugin {
        filter: "dogoap=debug,bevy_dogoap=debug".to_string(),
        ..default()
    });

    // We need to register our components as DatumComponent, otherwise planner won't be able to find them
    register_components!(app, vec![IsHungry, IsTired]);

    app.add_systems(Startup, startup);
    app.add_systems(Update, (handle_eat_action, handle_sleep_action));

    // Run three frames to advance
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
