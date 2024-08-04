use bevy::{color::palettes::css::*, prelude::*, time::common_conditions::on_timer};
use bevy_dogoap::prelude::*;
use dogoap::prelude::*;
use rand::Rng;
use std::{collections::HashMap, time::Duration};

// A more involved example that resembles a simulation of sorts.
// The simulation is of a miner, who wants to upgrade their house
// The upgrade costs gold, which the miner doesn't have
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
// Upgrade House which costs Gold

// These are just handy strings so we don't fuck it up later.
const HUNGER_KEY: &str = "hunger";
const ENERGY_KEY: &str = "energy";
const LOCATION_KEY: &str = "at_location";
const HAS_ORE_KEY: &str = "has_ore";
const HAS_METAL_KEY: &str = "has_metal";
const GOLD_KEY: &str = "gold_amount";
// const IS_HOUSE_UPGRADED_KEY: &str = "is_house_upgraded";

#[derive(Clone, Default, Reflect)]
enum Location {
    #[default]
    House,
    Outside,
    Mushroom,
    Ore,
    Smelter,
    Merchant,
}

// All the keys for our actions
const EAT_ACTION: &str = "eat";
const SLEEP_ACTION: &str = "sleep";
const MINE_ORE_ACTION: &str = "mine_ore";
const SMELT_ORE_ACTION: &str = "smelt_ore";
const SELL_METAL_ACTION: &str = "sell_metal";
// const UPGRADE_HOUSE_ACTION: &str = "upgrade_house";

// All actions related to locations
const GO_TO_HOUSE: &str = "go_to_house";
const GO_TO_OUTSIDE: &str = "go_to_outside";
const GO_TO_MUSHROOM: &str = "go_to_mushroom";
const GO_TO_ORE: &str = "go_to_ore";
const GO_TO_SMELTER: &str = "go_to_smelter";
const GO_TO_MERCHANT: &str = "go_to_merchant";

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

#[derive(Component, Clone, DatumComponent)]
struct AtLocation(usize);

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
    // Some helpers for our enums
    let loc_house = Datum::Enum(Location::House as usize);
    let loc_outside = Datum::Enum(Location::Outside as usize);
    let loc_mushroom = Datum::Enum(Location::Mushroom as usize);
    let loc_ore = Datum::Enum(Location::Ore as usize);
    let loc_smelter = Datum::Enum(Location::Smelter as usize);
    let loc_merchant = Datum::Enum(Location::Merchant as usize);

    let window = windows.get_single().expect("Expected only one window! Wth");
    let window_height = window.height() / 2.0;
    let window_width = window.width() / 2.0;

    for i in 0..1 {
        // To spawn the miner+AI:
        // First we define our initial state
        // let state = LocalState::new()
        //     .with_field(HUNGER_KEY, Field::from(0.0))
        //     .with_field(ENERGY_KEY, Field::from(75.0))
        //     .with_field(LOCATION_KEY, Field::Enum(Location::Outside as usize))
        //     // .with_field(HAS_ORE_KEY, Field::from(false))
        //     // .with_field(HAS_METAL_KEY, Field::from(false))
        //     // .with_field(GOLD_KEY, Field::I64(0))
        //     ;

        // Then we decide a goal of not being hungry nor tired
        let goal = Goal::new()
            .with_req(HUNGER_KEY, Compare::LessThanEquals(Datum::F64(50.0)))
            .with_req(ENERGY_KEY, Compare::GreaterThanEquals(Datum::F64(50.0)))
            // .with_req(HAS_ORE_KEY, Compare::Equals(Field::Bool(true)))
            // .with_req(HAS_METAL_KEY, Compare::Equals(Field::from(true)))
            .with_req(GOLD_KEY, Compare::GreaterThanEquals(Datum::I64(10)));

        let goals = vec![goal.clone()];

        let eat_action = Action::new(EAT_ACTION)
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_mushroom))
            .with_precondition(
                ENERGY_KEY,
                Compare::GreaterThanEquals(Datum::F64(50.0)),
            )
            .with_effect(
                Effect::new(EAT_ACTION)
                    .with_mutator(Mutator::Decrement(HUNGER_KEY.to_string(), Datum::F64(25.0)))
                    .with_mutator(Mutator::Decrement(ENERGY_KEY.to_string(), Datum::F64(5.0)))
                    .with_mutator(Mutator::Set(LOCATION_KEY.to_string(), loc_outside)),
                1,
            );

        let sleep_action = simple_increment_action(SLEEP_ACTION, ENERGY_KEY, Datum::F64(50.0))
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_house));

        let mine_ore_action = Action::new(MINE_ORE_ACTION)
            .with_precondition(
                ENERGY_KEY,
                Compare::GreaterThanEquals(Datum::F64(50.0)),
            )
            .with_precondition(
                HUNGER_KEY,
                Compare::LessThanEquals(Datum::F64(50.0)),
            )
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_ore))
            .with_effect(
                Effect::new(MINE_ORE_ACTION)
                    .with_mutator(Mutator::Set(HAS_ORE_KEY.to_string(), Datum::Bool(true)))
                    .with_mutator(Mutator::Decrement(HUNGER_KEY.to_string(), Datum::F64(15.0)))
                    .with_mutator(Mutator::Increment(ENERGY_KEY.to_string(), Datum::F64(50.0))),
                2,
            );

        let smelt_ore_action = Action::new(SMELT_ORE_ACTION)
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_smelter))
            .with_precondition(HAS_ORE_KEY, Compare::Equals(Datum::Bool(true)))
            .with_precondition(ENERGY_KEY, Compare::GreaterThanEquals(Datum::F64(25.0)))
            .with_precondition(HUNGER_KEY, Compare::LessThanEquals(Datum::F64(50.0)))
            .with_effect(
                Effect::new(SMELT_ORE_ACTION)
                    .with_mutator(Mutator::Set(HAS_METAL_KEY.to_string(), Datum::Bool(true)))
                    .with_mutator(Mutator::Set(HAS_ORE_KEY.to_string(), Datum::Bool(false))),
                // .with_mutator(Mutator::Set(LOCATION_KEY.to_string(), loc_outside)),
                3,
            );

        let sell_metal_action = Action::new(SELL_METAL_ACTION)
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_merchant))
            .with_precondition(HAS_METAL_KEY, Compare::Equals(Datum::Bool(true)))
            .with_effect(
                Effect::new(SELL_METAL_ACTION)
                    .with_mutator(Mutator::Set(HAS_METAL_KEY.to_string(), Datum::Bool(false)))
                    .with_mutator(Mutator::Increment(GOLD_KEY.to_string(), Datum::I64(1))),
                // .with_mutator(Mutator::Set(LOCATION_KEY.to_string(), loc_outside)),
                1,
            );

        // let sell_metal_action = simple_increment_action(SELL_METAL_ACTION, GOLD_KEY, Field::I64(1))
        //     .with_effect(Effect::new(SELL_METAL_ACTION).with_mutator(Mutator::Set(HAS_METAL_KEY.to_string(), Field::Bool(false))), 1)
        //     .with_precondition(LOCATION_KEY, Compare::Equals(loc_merchant))
        //     .with_precondition(HAS_METAL_KEY, Compare::Equals(Field::Bool(true)))
        //     // .with_precondition(ENERGY_KEY, Compare::GreaterThanEquals(Field::from_f64(75.0)))
        //     ;

        println!("Our sell metal action");
        println!("{:#?}", sell_metal_action);

        let go_to_outside_action = simple_action(GO_TO_OUTSIDE, LOCATION_KEY, loc_outside);

        let go_to_house_action = simple_action(GO_TO_HOUSE, LOCATION_KEY, loc_house);

        let go_to_mushroom_action = simple_action(GO_TO_MUSHROOM, LOCATION_KEY, loc_mushroom)
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_outside));

        let go_to_ore_action = simple_action(GO_TO_ORE, LOCATION_KEY, loc_ore)
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_outside));

        let go_to_smelter_action = simple_action(GO_TO_SMELTER, LOCATION_KEY, loc_smelter)
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_outside));

        let go_to_merchant_action = simple_action(GO_TO_MERCHANT, LOCATION_KEY, loc_merchant)
            .with_precondition(LOCATION_KEY, Compare::Equals(loc_outside));

        let actions_map = create_action_map!(
            (EAT_ACTION, eat_action, EatAction),
            (SLEEP_ACTION, sleep_action, SleepAction),
            (MINE_ORE_ACTION, mine_ore_action, MineOreAction),
            (SMELT_ORE_ACTION, smelt_ore_action, SmeltOreAction),
            (SELL_METAL_ACTION, sell_metal_action, SellMetalAction),
            (GO_TO_OUTSIDE, go_to_outside_action, GoToOutsideAction),
            (GO_TO_HOUSE, go_to_house_action, GoToHouseAction),
            (GO_TO_MUSHROOM, go_to_mushroom_action, GoToMushroomAction),
            (GO_TO_ORE, go_to_ore_action, GoToOreAction),
            (GO_TO_SMELTER, go_to_smelter_action, GoToSmelterAction),
            (GO_TO_MERCHANT, go_to_merchant_action, GoToMerchantAction),
        );

        let initial_state = (
            Hunger(25.0),
            Energy(75.0),
            AtLocation(Location::Outside as usize),
            HasOre(false),
            HasMetal(false),
            GoldAmount(0),
        );

        let state = create_state!(
            Hunger(25.0),
            Energy(75.0),
            AtLocation(Location::Outside as usize),
            HasOre(false),
            HasMetal(false),
            GoldAmount(0)
        );

        let mut planner = Planner::new(state, goals, actions_map);

        planner.remove_goal_on_no_plan_found = false; // Don't remove the goal
        planner.always_plan = true; // Re-calculate our plan whenever we can
        planner.current_goal = Some(goal.clone());

        let text_style = TextStyle {
            font_size: 18.0,
            ..default()
        };

        commands
            .spawn((
                Name::new("Miner"),
                Miner,
                planner,
                initial_state,
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

    let mut rng = rand::thread_rng();

    // Begin with three mushrooms
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

    // Misc stuff we want somewhere
    commands.spawn(Camera2dBundle::default());
}

fn spawn_random_mushroom(mut commands: Commands, mushrooms: Query<Entity, With<Mushroom>>) {
    if mushrooms.iter().len() < 10 {
        let mut rng = rand::thread_rng();
        let y = rng.gen_range(-200.0..200.0);
        let x = rng.gen_range(-200.0..200.0);
        commands.spawn((
            Name::new("Mushroom"),
            Mushroom,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
}

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
        let direction = (destination - origin.translation).normalize();
        origin.translation += direction * 128.0 * delta;
    } else {
        // We're there!
        at_location.0 = destination_enum as usize;

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

        // Outside is slightly to the left of the house...
        let offset = Vec3::new(-50.0, 0.0, 0.0);
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

fn find_closest(origin: Vec3, items: Vec<(Entity, Transform)>) -> Option<(Entity, Vec3)> {
    let mut closest: Option<(Entity, Transform, f32)> = None;
    for (_entity, transform) in &items {
        match closest {
            Some((_m, _t, d)) => {
                let distance = transform.translation.distance(origin);
                if distance < d {
                    closest = Some((*_entity, *transform, distance));
                }
                // Compare distance
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

        // planner
        //     .state
        //     .fields
        //     .entry(HUNGER_KEY.to_string())
        //     .and_modify(|e| *e -= Field::from_f64(50.0));
        hunger.0 -= 50.0;

        commands.entity(entity).remove::<EatAction>();
        commands.entity(mushroom.0).despawn_recursive();

        at_location.0 = Location::Outside as usize;

        // planner.state.fields.insert(
        //     LOCATION_KEY.to_string(),
        //     Field::Enum(Location::Outside as usize),
        // );
    }
}

fn handle_sleep_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &SleepAction, &mut Energy)>,
) {
    let mut rng = rand::thread_rng();
    for (entity, _action, mut energy) in query.iter_mut() {
        // Stop planning while we sleep
        // planner.always_plan = false;
        let r = rng.gen_range(5.0..20.0);
        let val: f64 = r * time.delta_seconds_f64();
        energy.0 += val;
        if energy.0 >= 100.0 {
            commands.entity(entity).remove::<SleepAction>();
            commands.entity(entity).insert(GoToOutsideAction); // We can manually control actions as well if needed
            energy.0 = 100.0;
            // planner.always_plan = true;
        }
    }
}

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
                    // planner
                    //     .state
                    //     .fields
                    //     .insert(HAS_ORE_KEY.to_string(), Field::Bool(true));
                    has_ore.0 = true;

                    // planner.state.fields.insert(
                    //     LOCATION_KEY.to_string(),
                    //     Field::Enum(Location::Outside as usize),
                    // );
                    at_location.0 = Location::Outside as usize;

                    commands.entity(entity).remove::<MineOreAction>();
                    commands.entity(closest.0).despawn_recursive();
                } else {
                    let mut rng = rand::thread_rng();
                    // Mining consumes energy!
                    // planner
                    //     .state
                    //     .fields
                    //     .entry(ENERGY_KEY.to_string())
                    //     .and_modify(|e| {
                    let r = rng.gen_range(5.0..10.0);
                    let val: f64 = r * time.delta_seconds_f64();
                    energy.0 -= val;
                    if energy.0 <= 0.0 {
                        commands.entity(entity).remove::<MineOreAction>();
                        energy.0 = 0.0
                    }
                    // });
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
                // planner
                //     .state
                //     .fields
                //     .insert(HAS_METAL_KEY.to_string(), Field::Bool(true));
                has_metal.0 = true;

                // planner
                //     .state
                //     .fields
                //     .insert(HAS_ORE_KEY.to_string(), Field::Bool(false));
                has_ore.0 = false;

                // planner.state.fields.insert(
                //     LOCATION_KEY.to_string(),
                //     Field::Enum(Location::Outside as usize),
                // );

                at_location.0 = Location::Outside as usize;

                commands.entity(entity).remove::<SmeltOreAction>();
            } else {
                let mut rng = rand::thread_rng();
                // Smelting consumes even more energy!
                // planner
                //     .state
                //     .fields
                //     .entry(ENERGY_KEY.to_string())
                //     .and_modify(|e| {
                let r = rng.gen_range(10.0..15.0);
                let val: f64 = r * time.delta_seconds_f64();
                energy.0 -= val;
                if energy.0 <= 0.0 {
                    commands.entity(entity).remove::<SmeltOreAction>();
                    energy.0 = 0.0
                }
                // });
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
                // planner
                //     .state
                //     .fields
                //     .insert(HAS_METAL_KEY.to_string(), Field::Bool(false));
                has_metal.0 = false;

                // planner
                //     .state
                //     .fields
                //     .entry(GOLD_KEY.to_string())
                //     .and_modify(|e| {
                //         *e += Field::I64(1);
                //     });
                gold_amount.0 += 1;

                // planner.state.fields.insert(
                //     LOCATION_KEY.to_string(),
                //     Field::Enum(Location::Outside as usize),
                // );
                at_location.0 = Location::Outside as usize;

                commands.entity(entity).remove::<SellMetalAction>();
            } else {
                // Do nothing while we perform the selling!
            }
        });
    }
}

fn on_remove_sleep_action(_trigger: Trigger<OnRemove, SleepAction>) {
    println!("It seems SleepAction was removed");
}

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
    // let planner = query.get_single().unwrap();
    for (entity, hunger, energy, has_ore, has_metal, gold_amount, children) in query.iter() {
        // let hunger = match planner.state.fields.get(HUNGER_KEY).unwrap() {
        //     Field::F64(v) => v,
        //     _ => panic!("unimplemented!"),
        // };

        // let energy = match planner.state.fields.get(ENERGY_KEY).unwrap() {
        //     Field::F64(v) => v,
        //     _ => panic!("unimplemented!"),
        // };

        let hunger = hunger.0;
        let energy = energy.0;
        let has_ore = has_ore.0;
        let has_metal = has_metal.0;
        let gold_amount = gold_amount.0;

        // let gold = match planner.state.fields.get(GOLD_KEY).unwrap() {
        //     Field::I64(v) => v,
        //     _ => panic!("unimplemented!"),
        // };

        // let has_ore = match planner.state.fields.get(HAS_ORE_KEY).unwrap() {
        //     Field::Bool(v) => v,
        //     _ => panic!("unimplemented!"),
        // };

        // let has_metal = match planner.state.fields.get(HAS_METAL_KEY).unwrap() {
        //     Field::Bool(v) => v,
        //     _ => panic!("unimplemented!"),
        // };

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

fn vec3_to_vec2(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}

// Worlds shittiest graphics incoming, beware and don't copy, massively wasteful
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
        gizmos.circle_2d(vec3_to_vec2(miner_transform.translation), 16., NAVY);
    }

    gizmos.rect_2d(
        vec3_to_vec2(q_house.get_single().unwrap().translation),
        0.0,
        Vec2::new(40.0, 80.0),
        AQUAMARINE,
    );

    gizmos.rect_2d(
        vec3_to_vec2(q_smelter.get_single().unwrap().translation),
        0.0,
        Vec2::new(30.0, 30.0),
        YELLOW_GREEN,
    );

    gizmos.circle_2d(
        vec3_to_vec2(q_merchant.get_single().unwrap().translation),
        16.,
        GOLD,
    );

    for mushroom_transform in q_mushrooms.iter() {
        gizmos.circle_2d(
            vec3_to_vec2(mushroom_transform.translation),
            4.,
            GREEN_YELLOW,
        );
    }

    for ore_transform in q_ore.iter() {
        gizmos.circle_2d(vec3_to_vec2(ore_transform.translation), 4., ROSY_BROWN);
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(DogoapPlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, draw_gizmos)
        .add_systems(
            Update,
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
        .observe(on_remove_sleep_action)
        .add_systems(
            Update,
            spawn_random_mushroom.run_if(on_timer(Duration::from_secs(5))),
        )
        .add_systems(
            Update,
            over_time_needs_change.run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(
            Update,
            print_current_local_state.run_if(on_timer(Duration::from_millis(50))),
        );

    register_components!(
        app,
        vec![Hunger, Energy, AtLocation, HasOre, HasMetal, GoldAmount]
    );

    app.run();
}
