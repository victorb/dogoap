// This is a really basic example of how to use dogoap with bevy_dogoap for use in Bevy
// It sets up two states on a entity, is_hungry and is_tired
// Then it sets up two possible actions:
// - SleepAction, which sets is_tired to false
// - EatAction, which requires is_tired to be false, and sets is_hungry to true

// We run three app.update() instead of app.run(), then print the final state after
// those three updates, so there is some sort of end to it all...

// Final state should be that is_hungry is false, and is_tired is false

use bevy::{log::LogPlugin, prelude::*};
use bevy_dogoap::prelude::*;

// This is a LocalFieldComponent that can hold one value
#[derive(Component, Clone, LocalFieldComponent)]
struct IsHungry(bool);

#[derive(Component, Clone, LocalFieldComponent)]
struct IsTired(bool);

// This is our ActionComponent that gets added whenever the planner thinks
// the entity need to perform the action in order to reach the goal
#[derive(Component, Reflect, Clone, Default, ActionComponent)]
struct EatAction;

#[derive(Component, Reflect, Clone, Default, ActionComponent)]
struct SleepAction;

fn startup(mut commands: Commands) {

    // This is the goal we want the planner to help us reach
    let goal = create_goal!(
        (IsHungry, Compare::Equals, Field::Bool(false)),
        (IsTired, Compare::Equals, Field::Bool(false))
    );

    // Our first action, the eat action, that sets is_hungry to false
    // but requires is_tired to be set to false first
    let eat_action = simple_action(&EatAction::key(), &IsHungry::key(), Field::Bool(false))
        .with_precondition(&IsTired::key(), Compare::Equals(Field::Bool(false)));

    // Our first action, the sleep action, that sets is_tired to false
    // No preconditions in order to sleep
    let sleep_action = simple_action(&SleepAction::key(), &IsTired::key(), Field::Bool(false));

    // Here we connect our string action with the Component we want to be added
    // for that action
    let actions_map = create_action_map!(
        (EatAction::key(), eat_action, EatAction),
        (SleepAction::key(), sleep_action, SleepAction)
    );

    // To spawn the planner, we first create a new entity
    // You can also use an existing entity if you have one at hand
    // This will usually be whatever Entity you use for your NPC
    let entity = commands.spawn_empty().insert(Name::new("Planner")).id();

    // Create our initial state
    let initial_state = create_state!(IsHungry(true), IsTired(true));

    // We create the instance of our planner with our initial state, our goals and finally the
    // possible actions
    let planner = Planner::new(initial_state, vec![goal], actions_map);


    // Next we need to add all the LocalFieldComponent we've created
    // this function helps us do just that a tiny bit easier, it'll insert all registered components
    planner.insert_field_components(&mut commands, entity);

    // We could also manually insert them ourselves like this:
    // commands
    //     .entity(entity)
    //     .insert((IsHungry(true), IsTired(true)));

    // Finally we add our Planner component to the entity we spawned earlier
    commands.entity(entity).insert(planner);
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

    // Make sure to include the DogoapPlugin which manages the planner for us
    app.add_plugins(DogoapPlugin);

    // We configure the logplugin to output debug info for both dogoap and bevy_dogoap
    app.add_plugins(LogPlugin {
        filter: "dogoap=debug,bevy_dogoap=debug".to_string(),
        ..default()
    });

    // We need to register our components as LocalFieldComponent, otherwise planner won't be able to find them
    register_components!(app, vec![IsHungry, IsTired]);

    app.add_systems(Startup, startup);
    app.add_systems(Update, (handle_eat_action, handle_sleep_action));

    // Run five frames to advance
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
