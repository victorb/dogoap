use bevy::{color::palettes::css::*, prelude::*, time::common_conditions::on_timer};
use bevy_dogoap::prelude::*;
use dogoap::prelude::*;
use rand::Rng;
use std::{collections::HashMap, time::Duration};

// A more involved example that resembles a simulation of sorts.
// The simulation is of a miner, who wants to earn 3 gold
// In order to get gold, they need to sell metal at the Merchant
// And in order to get metal, the miner needs to smelt some ore
// And in order to get ore, the miner needs to go out, find ore and mine it
// And besides those requirements, the miner also have hunger and energy
// that constantly change, and they need to sleep and eat in order
// to satisfy those needs.

// Put another way, the miner has to:
// Eat and Sleep every now and then
// Mine to get Ore
// Smelt Ore to get Metal
// Sell Metal to get Gold

#[derive(Clone, Default, Reflect, Copy, EnumDatum)]
enum Location {
    #[default]
    House,
    Outside,
    Mushroom,
    Ore,
    Smelter,
    Merchant,
}

/// This is our marker components, so we can keep track of the various in-game entities
#[derive(Component)]
struct Miner;

#[derive(Component)]
struct House;

#[derive(Component)]
struct Smelter;

#[derive(Component)]
struct Mushroom;

#[derive(Component)]
struct Ore;

#[derive(Component)]
struct Merchant;

// Various actions our Miner can perform

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct EatAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct SleepAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct MineOreAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct SmeltOreAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct SellMetalAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToOutsideAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToHouseAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToMushroomAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToOreAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToSmelterAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToMerchantAction;

// All of our State fields

#[derive(Component, Clone, DatumComponent)]
struct Hunger(f64);

#[derive(Component, Clone, DatumComponent)]
struct Energy(f64);

#[derive(Component, Clone, EnumComponent)]
struct AtLocation(Location);

#[derive(Component, Clone, DatumComponent)]
struct HasOre(bool);

#[derive(Component, Clone, DatumComponent)]
struct HasMetal(bool);

#[derive(Component, Clone, DatumComponent)]
struct GoldAmount(i64);

// UI elements
#[derive(Component)]
struct NeedsText;

fn startup(mut commands: Commands, windows: Query<&Window>) {
    for i in 0..1 {
        let gold_goal = Goal::from_reqs(&[GoldAmount::is(3)]);

        let sleep_action = SleepAction::new()
            .add_precondition(Energy::is_less(50.0))
            .add_precondition(AtLocation::is(Location::House))
            .add_mutator(Energy::increase(100.0))
            .set_cost(1);

        let eat_action = EatAction::new()
            .add_precondition(Hunger::is_more(50.0))
            .add_precondition(AtLocation::is(Location::Mushroom))
            .add_mutator(Hunger::decrease(25.0))
            .add_mutator(AtLocation::set(Location::Outside))
            .set_cost(2);

        let mine_ore_action = MineOreAction::new()
            .add_precondition(Energy::is_more(10.0))
            .add_precondition(Hunger::is_less(75.0))
            .add_precondition(AtLocation::is(Location::Ore))
            .add_mutator(HasOre::set(true))
            .set_cost(3);

        let smelt_ore_action = SmeltOreAction::new()
            .add_precondition(Energy::is_more(10.0))
            .add_precondition(Hunger::is_less(75.0))
            .add_precondition(AtLocation::is(Location::Smelter))
            .add_precondition(HasOre::is(true))
            .add_mutator(HasOre::set(false))
            .add_mutator(HasMetal::set(true))
            .set_cost(4);

        let sell_metal_action = SellMetalAction::new()
            .add_precondition(Energy::is_more(10.0))
            .add_precondition(Hunger::is_less(75.0))
            .add_precondition(AtLocation::is(Location::Merchant))
            .add_precondition(HasMetal::is(true))
            .add_mutator(GoldAmount::increase(1))
            .add_mutator(HasMetal::set(false))
            .set_cost(5);

        let go_to_outside_action = GoToOutsideAction::new()
            .add_mutator(AtLocation::set(Location::Outside))
            .set_cost(1);

        let go_to_house_action = GoToHouseAction::new()
            .add_precondition(AtLocation::is(Location::Outside))
            .add_mutator(AtLocation::set(Location::House))
            .set_cost(1);

        let go_to_mushroom_action = GoToMushroomAction::new()
            .add_precondition(AtLocation::is(Location::Outside))
            .add_mutator(AtLocation::set(Location::Mushroom))
            .set_cost(2);

        let go_to_ore_action = GoToOreAction::new()
            .add_precondition(AtLocation::is(Location::Outside))
            .add_mutator(AtLocation::set(Location::Ore))
            .set_cost(3);

        let go_to_smelter_action = GoToSmelterAction::new()
            .add_precondition(AtLocation::is(Location::Outside))
            .add_mutator(AtLocation::set(Location::Smelter))
            .set_cost(4);

        let go_to_merchant_action = GoToMerchantAction::new()
            .add_precondition(AtLocation::is(Location::Outside))
            .add_mutator(AtLocation::set(Location::Merchant))
            .set_cost(5);

        let (mut planner, components) = create_planner!({
            actions: [
                (EatAction, eat_action),
                (SleepAction, sleep_action),
                (MineOreAction, mine_ore_action),
                (SmeltOreAction, smelt_ore_action),
                (SellMetalAction, sell_metal_action),
                //
                (GoToOutsideAction, go_to_outside_action),
                (GoToHouseAction, go_to_house_action),
                (GoToMushroomAction, go_to_mushroom_action),
                (GoToOreAction, go_to_ore_action),
                (GoToSmelterAction, go_to_smelter_action),
                (GoToMerchantAction, go_to_merchant_action),
            ],
            state: [GoldAmount(0), Hunger(25.0), Energy(75.0), AtLocation(Location::Outside), HasOre(false), HasMetal(false)],
            goals: [gold_goal],
        });

        // Don't remove the goal if there is no plan found
        planner.remove_goal_on_no_plan_found = false;
        // Re-calculate our plan constantly
        planner.always_plan = true;
        // Set current goal to be to acquire gold
        planner.current_goal = Some(gold_goal.clone());

        let text_style = TextStyle {
            font_size: 18.0,
            ..default()
        };

        commands
            .spawn((
                Name::new("Miner"),
                Miner,
                planner,
                components,
                Transform::from_translation(Vec3::ZERO.with_x(50.0 * i as f32)),
                GlobalTransform::from_translation(Vec3::ZERO.with_x(50.0 * i as f32)),
            ))
            .with_children(|subcommands| {
                subcommands.spawn((
                    Text2dBundle {
                        transform: Transform::from_translation(Vec3::new(10.0, -10.0, 10.0)),
                        text: Text::from_section("", text_style.clone())
                            .with_justify(JustifyText::Left),
                        text_anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                    NeedsText,
                ));
            });
    }

    commands.spawn((
        Name::new("House"),
        House,
        Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
    ));

    commands.spawn((
        Name::new("Smelter"),
        Smelter,
        Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
    ));

    commands.spawn((
        Name::new("Merchant"),
        Merchant,
        Transform::from_translation(Vec3::new(-300.0, -50.0, 0.0)),
    ));

    let window = windows.get_single().expect("Expected only one window! Wth");
    let window_height = window.height() / 2.0;
    let window_width = window.width() / 2.0;

    let mut rng = rand::thread_rng();

    // Begin with three mushrooms our miner can eat
    for _i in 0..3 {
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Mushroom"),
            Mushroom,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }

    // Spawn 10 ores we can mine as well
    for _i in 0..10 {
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Ore"),
            Ore,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }

    // Spawn a camera so we see something
    commands.spawn(Camera2dBundle::default());
}

// Spawn new mushrooms if there are less than 10
fn spawn_random_mushroom(
    windows: Query<&Window>,
    mut commands: Commands,
    mushrooms: Query<Entity, With<Mushroom>>,
) {
    if mushrooms.iter().len() < 10 {
        let window = windows.get_single().expect("Expected only one window! Wth");
        let window_height = window.height() / 2.0;
        let window_width = window.width() / 2.0;

        let mut rng = rand::thread_rng();
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Mushroom"),
            Mushroom,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
}

// Spawn new mushrooms if there are less than 10
fn spawn_random_ore(
    windows: Query<&Window>,
    mut commands: Commands,
    ores: Query<Entity, With<Ore>>,
) {
    if ores.iter().len() < 10 {
        let window = windows.get_single().expect("Expected only one window! Wth");
        let window_height = window.height() / 2.0;
        let window_width = window.width() / 2.0;

        let mut rng = rand::thread_rng();
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Ore"),
            Ore,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
}

// Helper function to handle the GoTo* actions
fn go_to_location<T>(
    at_location: &mut AtLocation,
    delta: f32,
    origin: &mut Transform,
    destination: Vec3,
    destination_enum: Location,
    entity: Entity,
    commands: &mut Commands,
) where
    T: Component,
{
    if origin.translation.distance(destination) > 5.0 {
        // We're not quite there yet, move closer
        let direction = (destination - origin.translation).normalize();
        origin.translation += direction * 128.0 * delta;
    } else {
        // We're there!
        at_location.0 = destination_enum;

        // Remove our action to signal we've completed the move
        commands.entity(entity).remove::<T>();
    }
}

fn handle_go_to_house_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &GoToHouseAction, &mut Transform, &mut AtLocation), Without<House>>,
    q_house: Query<&Transform, With<House>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_house = q_house
            .get_single()
            .expect("There should only be one house!");

        go_to_location::<GoToHouseAction>(
            &mut at_location,
            time.delta_seconds(),
            &mut t_entity,
            t_house.translation,
            Location::House,
            entity,
            &mut commands,
        )
    }
}

fn handle_go_to_smelter_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &GoToSmelterAction, &mut Transform, &mut AtLocation),
        Without<Smelter>,
    >,
    q_smelter: Query<&Transform, With<Smelter>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_smelter = q_smelter
            .get_single()
            .expect("There should only be one smelter!");

        go_to_location::<GoToSmelterAction>(
            &mut at_location,
            time.delta_seconds(),
            &mut t_entity,
            t_smelter.translation,
            Location::Smelter,
            entity,
            &mut commands,
        )
    }
}

fn handle_go_to_outside_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &GoToOutsideAction, &mut Transform, &mut AtLocation), Without<House>>,
    q_house: Query<&Transform, With<House>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_house = q_house
            .get_single()
            .expect("There should only be one house!");

        // Outside is slightly to the left of the house... Fight me
        let offset = Vec3::new(-30.0, 0.0, 0.0);
        let new_pos = t_house.translation + offset;

        go_to_location::<GoToOutsideAction>(
            &mut at_location,
            time.delta_seconds(),
            &mut t_entity,
            new_pos,
            Location::Outside,
            entity,
            &mut commands,
        )
    }
}

fn handle_go_to_merchant_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &GoToMerchantAction, &mut Transform, &mut AtLocation),
        Without<Merchant>,
    >,
    q_destination: Query<&Transform, With<Merchant>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_destination = q_destination
            .get_single()
            .expect("There should only be one merchant!");

        go_to_location::<GoToMerchantAction>(
            &mut at_location,
            time.delta_seconds(),
            &mut t_entity,
            t_destination.translation,
            Location::Merchant,
            entity,
            &mut commands,
        )
    }
}

fn handle_go_to_mushroom_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &GoToMushroomAction, &mut Transform, &mut AtLocation),
        Without<Mushroom>,
    >,
    q_mushrooms: Query<(Entity, &Transform), With<Mushroom>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_mushrooms.iter().map(|(e, t)| (e, *t)).collect();
        let mushroom = find_closest(origin, items);

        let mushroom = match mushroom {
            Some(v) => v,
            None => panic!("No mushroom could be found, HOW?!"),
        };

        go_to_location::<GoToMushroomAction>(
            &mut at_location,
            time.delta_seconds(),
            &mut t_entity,
            mushroom.1,
            Location::Mushroom,
            entity,
            &mut commands,
        )
    }
}

fn handle_go_to_ore_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &GoToOreAction, &mut Transform, &mut AtLocation), Without<Ore>>,
    q_world_resource: Query<(Entity, &Transform), With<Ore>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> =
            q_world_resource.iter().map(|(e, t)| (e, *t)).collect();
        let closest = find_closest(origin, items);

        let closest = match closest {
            Some(v) => v,
            None => panic!("No closest could be found, HOW?!"),
        };

        go_to_location::<GoToOreAction>(
            &mut at_location,
            time.delta_seconds(),
            &mut t_entity,
            closest.1,
            Location::Ore,
            entity,
            &mut commands,
        )
    }
}

// Helper function that figures out what (Entity, Transform) tuple is the closest to our origin
fn find_closest(origin: Vec3, items: Vec<(Entity, Transform)>) -> Option<(Entity, Vec3)> {
    let mut closest: Option<(Entity, Transform, f32)> = None;
    for (_entity, transform) in &items {
        match closest {
            Some((_m, _t, d)) => {
                let distance = transform.translation.distance(origin);
                if distance < d {
                    closest = Some((*_entity, *transform, distance));
                }
            }
            None => {
                closest = Some((*_entity, *transform, 1000.0));
            }
        }
    }
    match closest {
        Some((e, t, _f)) => Some((e, t.translation)),
        None => None,
    }
}

fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &EatAction,
            &mut Transform,
            &mut Hunger,
            &mut AtLocation,
        ),
        Without<Mushroom>,
    >,
    q_mushrooms: Query<(Entity, &Transform), With<Mushroom>>,
) {
    for (entity, _action, t_entity, mut hunger, mut at_location) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_mushrooms.iter().map(|(e, t)| (e, *t)).collect();
        let mushroom = find_closest(origin, items);

        println!("Eating mushroom we found at {:?}", mushroom);

        let mushroom = match mushroom {
            Some(v) => v,
            None => panic!("No mushroom could be found, HOW?!"),
        };

        hunger.0 -= 50.0;

        commands.entity(entity).remove::<EatAction>();
        commands.entity(mushroom.0).despawn_recursive();

        at_location.0 = Location::Outside;
    }
}

fn handle_sleep_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &SleepAction, &mut Energy, &mut Planner)>,
) {
    let mut rng = rand::thread_rng();
    for (entity, _action, mut energy, mut planner) in query.iter_mut() {
        // Stop planning while we sleep, so we regain all the energy we can
        planner.always_plan = false;

        let r = rng.gen_range(5.0..20.0);
        let val: f64 = r * time.delta_seconds_f64();
        energy.0 += val;
        if energy.0 >= 100.0 {
            commands.entity(entity).remove::<SleepAction>();

            // We can manually control actions as well if needed, here we make sure to go outside
            // after we finish sleeping
            commands.entity(entity).insert(GoToOutsideAction);
            energy.0 = 100.0;

            // Enable continous planning again after we've done sleeping
            planner.always_plan = true;
        }
    }
}

// Helper method that allows us to "delay" an action by a set amount
// Accepts a callback that has `is_completed: bool` as a parameter
fn action_with_progress<F>(
    progresses: &mut Local<HashMap<Entity, Timer>>,
    entity: Entity,
    time: &Res<Time>,
    delay_seconds: f32,
    on_progress: F,
) where
    F: FnOnce(bool),
{
    let progress = progresses.get_mut(&entity);

    match progress {
        Some(progress) => {
            if progress.tick(time.delta()).just_finished() {
                // TODO Wonder if we can do this in a nicer way?
                on_progress(true);
                progresses.remove(&entity);
            } else {
                on_progress(false);
            }
        }
        None => {
            progresses.insert(entity, Timer::from_seconds(delay_seconds, TimerMode::Once));
        }
    }
}

fn handle_mine_ore_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &MineOreAction,
            &mut Transform,
            &mut HasOre,
            &mut AtLocation,
            &mut Energy,
        ),
        Without<Ore>,
    >,
    q_ores: Query<(Entity, &Transform), With<Ore>>,
    mut mining_progress: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, t_entity, mut has_ore, mut at_location, mut energy) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_ores.iter().map(|(e, t)| (e, *t)).collect();
        let closest = find_closest(origin, items);

        let closest = match closest {
            Some(v) => v,
            None => panic!("No ore could be found, HOW?!"),
        };

        action_with_progress(
            &mut mining_progress,
            closest.0,
            &time,
            2.0,
            |is_completed| {
                if is_completed {
                    has_ore.0 = true;
                    at_location.0 = Location::Outside;

                    commands.entity(entity).remove::<MineOreAction>();
                    commands.entity(closest.0).despawn_recursive();
                } else {
                    let mut rng = rand::thread_rng();

                    // Mining consumes energy!
                    let r = rng.gen_range(5.0..10.0);
                    let val: f64 = r * time.delta_seconds_f64();
                    energy.0 -= val;
                    // If we're running out of energy before finishing, stop mining for now
                    if energy.0 <= 0.0 {
                        commands.entity(entity).remove::<MineOreAction>();
                        energy.0 = 0.0
                    }
                }
            },
        );
    }
}

fn handle_smelt_ore_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &SmeltOreAction,
            &mut Transform,
            &mut Energy,
            &mut HasOre,
            &mut HasMetal,
            &mut AtLocation,
        ),
        Without<Smelter>,
    >,
    q_smelters: Query<(Entity, &Transform), With<Smelter>>,
    mut progress: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, t_entity, mut energy, mut has_ore, mut has_metal, mut at_location) in
        query.iter_mut()
    {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_smelters.iter().map(|(e, t)| (e, *t)).collect();
        let closest = find_closest(origin, items);

        let closest = match closest {
            Some(v) => v,
            None => panic!("No ore could be found, HOW?!"),
        };

        action_with_progress(&mut progress, closest.0, &time, 5.0, |is_completed| {
            if is_completed {
                has_metal.0 = true;

                has_ore.0 = false;

                at_location.0 = Location::Outside;

                commands.entity(entity).remove::<SmeltOreAction>();
            } else {
                let mut rng = rand::thread_rng();
                // Smelting consumes even more energy!
                let r = rng.gen_range(10.0..15.0);
                let val: f64 = r * time.delta_seconds_f64();
                energy.0 -= val;
                if energy.0 <= 0.0 {
                    commands.entity(entity).remove::<SmeltOreAction>();
                    energy.0 = 0.0
                }
            }
        });
    }
}

fn handle_sell_metal_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &SellMetalAction,
            &mut Transform,
            &mut HasMetal,
            &mut GoldAmount,
            &mut AtLocation,
        ),
        Without<Smelter>,
    >,
    mut progress: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, _t_entity, mut has_metal, mut gold_amount, mut at_location) in
        query.iter_mut()
    {
        action_with_progress(&mut progress, entity, &time, 1.0, |is_completed| {
            if is_completed {
                has_metal.0 = false;

                gold_amount.0 += 1;

                at_location.0 = Location::Outside;

                commands.entity(entity).remove::<SellMetalAction>();
            } else {
                // Do nothing in particular while we perform the selling
            }
        });
    }
}

// Increases hunger and decreases energy over time
fn over_time_needs_change(time: Res<Time>, mut query: Query<(&mut Hunger, &mut Energy)>) {
    let mut rng = rand::thread_rng();
    for (mut hunger, mut energy) in query.iter_mut() {
        // Increase hunger
        let r = rng.gen_range(10.0..20.0);
        let val: f64 = r * time.delta_seconds_f64();
        hunger.0 += val;
        if hunger.0 > 100.0 {
            hunger.0 = 100.0;
        }

        // Decrease energy
        let r = rng.gen_range(1.0..10.0);
        let val: f64 = r * time.delta_seconds_f64();
        energy.0 -= val;
        if energy.0 < 0.0 {
            energy.0 = 0.0;
        }
    }
}

fn print_current_local_state(
    query: Query<(
        Entity,
        &Hunger,
        &Energy,
        &HasOre,
        &HasMetal,
        &GoldAmount,
        &Children,
    )>,
    q_actions: Query<(
        Option<&SleepAction>,
        Option<&EatAction>,
        Option<&MineOreAction>,
        Option<&SmeltOreAction>,
        Option<&SellMetalAction>,
        Option<&GoToHouseAction>,
        Option<&GoToOutsideAction>,
        Option<&GoToMushroomAction>,
        Option<&GoToOreAction>,
        Option<&GoToSmelterAction>,
        Option<&GoToMerchantAction>,
    )>,
    mut q_child: Query<&mut Text, With<NeedsText>>,
) {
    for (entity, hunger, energy, has_ore, has_metal, gold_amount, children) in query.iter() {
        let hunger = hunger.0;
        let energy = energy.0;
        let has_ore = has_ore.0;
        let has_metal = has_metal.0;
        let gold_amount = gold_amount.0;

        let mut current_action = "Idle";

        let (
            sleep,
            eat,
            mine,
            smelting,
            selling_metal,
            go_to_house,
            go_to_outside,
            go_to_mushroom,
            go_to_ore,
            go_to_smelter,
            go_to_merchant,
        ) = q_actions.get(entity).unwrap();

        if sleep.is_some() {
            current_action = "Sleeping";
        }

        if eat.is_some() {
            current_action = "Eating";
        }

        if mine.is_some() {
            current_action = "Mining";
        }

        if smelting.is_some() {
            current_action = "Smelting ore";
        }

        if selling_metal.is_some() {
            current_action = "Selling metal";
        }

        if go_to_house.is_some() {
            current_action = "Going to house";
        }

        if go_to_outside.is_some() {
            current_action = "Going to outside";
        }

        if go_to_mushroom.is_some() {
            current_action = "Going to mushroom";
        }

        if go_to_ore.is_some() {
            current_action = "Going to ore";
        }

        if go_to_smelter.is_some() {
            current_action = "Going to smelter";
        }

        if go_to_merchant.is_some() {
            current_action = "Going to merchant";
        }

        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value = format!(
                "{current_action}\nGold: {gold_amount}\nHunger: {hunger:.0}\nEnergy: {energy:.0}\nHas Ore? {has_ore}\nHas Metal? {has_metal}"
            );
        }
    }
}

// Worlds shittiest graphics incoming, beware and don't copy
fn draw_gizmos(
    mut gizmos: Gizmos,
    q_miner: Query<&Transform, With<Miner>>,
    q_house: Query<&Transform, With<House>>,
    q_smelter: Query<&Transform, With<Smelter>>,
    q_merchant: Query<&Transform, With<Merchant>>,
    q_mushrooms: Query<&Transform, With<Mushroom>>,
    q_ore: Query<&Transform, With<Ore>>,
) {
    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::new(16, 9),
            Vec2::new(80., 80.),
            // Dark gray
            Srgba::new(0.1, 0.1, 0.1, 0.5),
        )
        .outer_edges();

    for miner_transform in q_miner.iter() {
        gizmos.circle_2d(miner_transform.translation.truncate(), 16., NAVY);
    }

    gizmos.rect_2d(
        q_house.get_single().unwrap().translation.truncate(),
        0.0,
        Vec2::new(40.0, 80.0),
        AQUAMARINE,
    );

    gizmos.rect_2d(
        q_smelter.get_single().unwrap().translation.truncate(),
        0.0,
        Vec2::new(30.0, 30.0),
        YELLOW_GREEN,
    );

    gizmos.circle_2d(
        q_merchant.get_single().unwrap().translation.truncate(),
        16.,
        GOLD,
    );

    for mushroom_transform in q_mushrooms.iter() {
        gizmos.circle_2d(mushroom_transform.translation.truncate(), 4., GREEN_YELLOW);
    }

    for ore_transform in q_ore.iter() {
        gizmos.circle_2d(ore_transform.translation.truncate(), 4., ROSY_BROWN);
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#example-canvas".into()),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(DogoapPlugin)
    .add_systems(Startup, startup)
    .add_systems(Update, draw_gizmos)
    .add_systems(
        FixedUpdate,
        (
            handle_go_to_outside_action,
            handle_go_to_house_action,
            handle_go_to_mushroom_action,
            handle_go_to_ore_action,
            handle_go_to_smelter_action,
            handle_go_to_merchant_action,
            handle_sleep_action,
            handle_eat_action,
            handle_mine_ore_action,
            handle_smelt_ore_action,
            handle_sell_metal_action,
        ),
    )
    .add_systems(
        FixedUpdate,
        spawn_random_mushroom.run_if(on_timer(Duration::from_secs(5))),
    )
    .add_systems(
        FixedUpdate,
        spawn_random_ore.run_if(on_timer(Duration::from_secs(5))),
    )
    .add_systems(
        FixedUpdate,
        over_time_needs_change.run_if(on_timer(Duration::from_millis(100))),
    )
    .add_systems(
        FixedUpdate,
        print_current_local_state.run_if(on_timer(Duration::from_millis(50))),
    );

    register_components!(
        app,
        vec![Hunger, Energy, AtLocation, HasOre, HasMetal, GoldAmount]
    );

    app.run();
}
