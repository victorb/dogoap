use bevy::{color::palettes::css::*, prelude::*, time::common_conditions::on_timer};
use bevy_dogoap::{create_action_map_v2, prelude::*};
use dogoap::prelude::*;
use rand::Rng;
use std::{collections::HashMap, time::Duration};

// This is a basic example on how you can use Dogoap while moving your agent around

/// This is our marker components, so we can keep track of the various in-game entities
#[derive(Component)]
struct Miner {
    speed: f32,
}

#[derive(Component)]
struct DeadMiner;

#[derive(Component)]
struct BusyObject(Entity);

#[derive(Component)]
struct Mushroom;

#[derive(Component)]
struct MoveTo(Vec3, Entity);

// Various actions our Miner can perform

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct EatAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct ReplicateAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToMushroomAction;

// All of our State fields

#[derive(Component, Clone, DatumComponent)]
struct Hunger(f64);

#[derive(Component, Clone, DatumComponent)]
struct AtMushroom(bool);

#[derive(Component, Clone, DatumComponent)]
struct IsReplicating(bool);

// UI elements
#[derive(Component)]
struct StateDebugText;

fn spawn_miner(commands: &mut Commands, position: Vec3, speed: f32) {
    let goal = Goal::new().with_req(&IsReplicating::key(), Compare::Equals(Datum::Bool(true)));

    let goals = vec![goal.clone()];

    let eat_action = Action::new(&EatAction::key())
        .with_precondition(&AtMushroom::key(), Compare::Equals(Datum::Bool(true)))
        .with_effect(
            Effect::new(&EatAction::key())
                .with_mutator(Mutator::Decrement(Hunger::key(), Datum::F64(10.0)))
                .with_mutator(Mutator::Set(AtMushroom::key(), Datum::Bool(false))),
            1,
        );

    let replicate_action = Action::new(&ReplicateAction::key())
        .with_precondition(&Hunger::key(), Compare::LessThanEquals(Datum::F64(10.0)))
        .with_effect(
            Effect::new(&ReplicateAction::key())
                .with_mutator(Mutator::Set(IsReplicating::key(), Datum::Bool(true)))
                .with_mutator(Mutator::Increment(Hunger::key(), Datum::F64(25.0))),
            10,
        );

    let go_to_mushroom_action = Action::new(&GoToMushroomAction::key()).with_effect(
        Effect::new(&GoToMushroomAction::key())
            .with_mutator(Mutator::Set(AtMushroom::key(), Datum::Bool(true)))
            .with_mutator(Mutator::Increment(Hunger::key(), Datum::F64(1.0))),
        2,
    );

    let actions_map = create_action_map_v2!(
        (EatAction, eat_action),
        (GoToMushroomAction, go_to_mushroom_action),
        (ReplicateAction, replicate_action)
    );

    let mut rng = rand::thread_rng();
    let hunger = rng.gen_range(20.0..45.0);
    let initial_state = (Hunger(hunger), AtMushroom(false), IsReplicating(false));
    let state = create_state!(Hunger(hunger), AtMushroom(false), IsReplicating(false));

    let mut planner = Planner::new(state, goals, actions_map);

    planner.remove_goal_on_no_plan_found = false; // Don't remove the goal
    planner.always_plan = true; // Re-calculate our plan whenever we can
    planner.current_goal = Some(goal.clone());

    let text_style = TextStyle {
        font_size: 12.0,
        ..default()
    };

    commands
        .spawn((
            Name::new("Miner"),
            Miner {speed},
            planner,
            initial_state,
            Transform::from_translation(position),
            GlobalTransform::from_translation(position),
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
                StateDebugText,
            ));
        });
}

fn startup(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.get_single().expect("Expected only one window! Wth");
    let window_height = window.height() / 2.0;
    let window_width = window.width() / 2.0;

    let mut rng = rand::thread_rng();

    for _i in 0..1 {
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        spawn_miner(&mut commands, Vec3::from_array([x, y, 1.0]), 128.0);
    }

    // Begin with three mushrooms
    for _i in 0..30 {
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Mushroom"),
            Mushroom,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
    // Misc stuff we want somewhere
    commands.spawn(Camera2dBundle::default());
}

fn spawn_random_mushroom(
    windows: Query<&Window>,
    mut commands: Commands,
    mushrooms: Query<Entity, With<Mushroom>>,
) {
    let window = windows.get_single().expect("Expected only one window! Wth");
    let window_height = window.height() / 2.0;
    let window_width = window.width() / 2.0;

    if mushrooms.iter().len() < 100 {
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

fn handle_move_to(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Miner, &MoveTo, &mut Transform)>,
) {
    for (entity, miner, move_to, mut transform) in query.iter_mut() {
        let destination = move_to.0;
        let destination_entity = move_to.1;

        // Check first if destination entity exists, otherwise cancel the MoveTo,
        match commands.get_entity(destination_entity) {
            Some(_) => {
                if transform.translation.distance(destination) > 5.0 {
                    let direction = (destination - transform.translation).normalize();
                    transform.translation += direction * miner.speed * time.delta_seconds();
                } else {
                    commands.entity(entity).remove::<MoveTo>();
                    commands.entity(destination_entity).remove::<BusyObject>();
                }
            }
            None => {
                // Cancel the MoveTo order as the destination no longer exists...
                commands.entity(entity).remove::<MoveTo>();
            }
        }
    }
}

fn handle_go_to_mushroom_action(
    mut commands: Commands,
    mut query: Query<
        (Entity, &GoToMushroomAction, &Transform, &mut AtMushroom),
        (Without<Mushroom>, Without<MoveTo>),
    >,
    q_mushrooms: Query<(Entity, &Transform), (With<Mushroom>, Without<BusyObject>)>,
    q_busy: Query<&BusyObject>
) {
    for (entity, _action, t_entity, mut at_mushroom) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_mushrooms.iter().map(|(e, t)| (e, *t)).collect();
        let mushroom = find_closest(origin, items);

        let (e_mushroom, t_mushroom, distance) = match mushroom {
            Some(v) => v,
            None => {
                // Do nothing...
                continue
            },
        };

        match q_busy.get(e_mushroom) {
            Ok(busy) => {
                if busy.0 != entity {
                    continue
                }
            },
            Err(_) => {}
        }

        if distance > 5.0 {
            commands.entity(e_mushroom).insert(BusyObject(entity));
            commands
                .entity(entity)
                .insert(MoveTo(t_mushroom, e_mushroom));
        } else {
            // Consume mushroom!
            // println!("We're at a mushroom, eat it!");
            at_mushroom.0 = true;
            commands.entity(entity).remove::<GoToMushroomAction>();
        }

        // go_to_location::<GoToMushroomAction>(
        //     &mut at_location,
        //     time.delta_seconds(),
        //     &mut t_entity,
        //     mushroom.1,
        //     Location::Mushroom,
        //     entity,
        //     &mut commands,
        // )
    }
}

fn find_closest(origin: Vec3, items: Vec<(Entity, Transform)>) -> Option<(Entity, Vec3, f32)> {
    items
        .into_iter()
        .fold(None, |closest, (entity, transform)| {
            let distance = transform.translation.distance(origin);
            match closest {
                Some((_, _, d)) if distance >= d => closest,
                _ => Some((entity, transform.translation, distance)),
            }
        })
}

fn handle_replicate_action(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &ReplicateAction,
        &mut IsReplicating,
        &mut Hunger,
        &mut Planner,
        &Miner,
        &Transform,
    )>,
    mut timers: Local<HashMap<Entity, Timer>>,
    time: Res<Time>,
) {
    for (entity, action, field, mut hunger, mut planner, miner, transform) in query.iter_mut() {
        match timers.get_mut(&entity) {
            Some(progress) => {
                if progress.tick(time.delta()).just_finished() {
                    println!("Progress: {:.2}", progress.fraction());
                    let new_transform = transform.translation + Vec3::from_array([25., 0., 0.]);
                    spawn_miner(&mut commands, new_transform, miner.speed);
                    commands.entity(entity).remove::<ReplicateAction>();
                    hunger.0 += 50.0;
                    timers.remove(&entity);
                    planner.always_plan = true;
                } else {
                    hunger.0 += 6.0 * time.delta_seconds_f64();
                }
            }
            None => {
                timers.insert(entity, Timer::from_seconds(3.0, TimerMode::Once));
                planner.always_plan = false;
            }
        }
    }
}

fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<
        (Entity, &EatAction, &Transform, &mut Hunger, &mut AtMushroom),
        Without<Mushroom>,
    >,
    q_mushrooms: Query<(Entity, &Transform), With<Mushroom>>,
) {
    // println!("Query hits: {}", query.iter().len());
    for (entity, _action, t_entity, mut hunger, mut at_mushroom) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_mushrooms.iter().map(|(e, t)| (e, *t)).collect();
        let mushroom = find_closest(origin, items);

        // println!("Eating mushroom we found at {:?}", mushroom);

        let (e_mushroom, _t_mushroom, distance) = match mushroom {
            Some(v) => v,
            None => panic!("No mushroom could be found, HOW?!"),
        };

        // Make sure we're actually in range to consume this mushroom
        // If not, remove the EatAction to cancel it, and the planner
        // will figure out what to do next
        if distance < 5.0 {
            // Before we consume this mushroom, make another query to ensure
            // it's still there, as it could have been consumed by another
            // Miner in the same frame, during the query.iter() loop
            match q_mushrooms.get(e_mushroom) {
                Ok(_) => {
                    hunger.0 -= 10.0;

                    if hunger.0 < 0.0 {
                        hunger.0 = 0.0;
                    }
                    commands.entity(e_mushroom).despawn_recursive();
                }
                // Don't consume as it doesn't exists
                Err(_) => {
                    warn!("Tried to consume non-existing mushroom");
                }
            }
        }

        commands.entity(entity).remove::<EatAction>();
        at_mushroom.0 = false;
    }
}

fn over_time_needs_change(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Hunger, &Transform)>,
) {
    let mut rng = rand::thread_rng();
    println!("Entities: {}", query.iter().len());
    for (entity, mut hunger, transform) in query.iter_mut() {
        // Increase hunger
        let r = rng.gen_range(10.0..20.0);
        let val: f64 = r * time.delta_seconds_f64();
        hunger.0 += val;
        if hunger.0 > 100.0 {
            // hunger.0 = 100.0;
            commands.entity(entity).despawn_recursive();
            let translation = transform.translation;
            commands.spawn((
                DeadMiner,
                Transform::from_translation(translation),
                GlobalTransform::from_translation(translation),
            ));
            println!("Removed starving Miner");
        }
    }
}

fn print_current_local_state(
    query: Query<(Entity, &Hunger, &Children)>,
    q_actions: Query<(
        Option<&IsPlanning>,
        Option<&EatAction>,
        Option<&GoToMushroomAction>,
        Option<&ReplicateAction>,
    )>,
    mut q_child: Query<&mut Text, With<StateDebugText>>,
) {
    // let planner = query.get_single().unwrap();
    for (entity, hunger, children) in query.iter() {
        let hunger = hunger.0;

        let mut current_action = "Idle";

        let (is_planning, eat, go_to_mushroom, replicate) = q_actions.get(entity).unwrap();

        if is_planning.is_some() {
            current_action = "Planning...";
        }

        if eat.is_some() {
            current_action = "Eating";
        }

        if go_to_mushroom.is_some() {
            current_action = "Going to mushroom";
        }

        if replicate.is_some() {
            current_action = "Replicating";
        }

        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value =
                format!("{current_action}\nHunger: {hunger:.0}\nEntity: {entity}");
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
    q_dead: Query<&Transform, With<DeadMiner>>,
    q_mushrooms: Query<&Transform, With<Mushroom>>,
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

    for mushroom_transform in q_mushrooms.iter() {
        gizmos.circle_2d(
            vec3_to_vec2(mushroom_transform.translation),
            4.,
            GREEN_YELLOW,
        );
    }

    for miner_transform in q_dead.iter() {
        gizmos.circle_2d(vec3_to_vec2(miner_transform.translation), 12., ORANGE_RED);
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
                handle_move_to,
                handle_go_to_mushroom_action,
                handle_eat_action,
                handle_replicate_action,
            )
                .chain(),
        )
        .add_systems(
            Update,
            spawn_random_mushroom.run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(
            Update,
            over_time_needs_change.run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(
            Update,
            print_current_local_state.run_if(on_timer(Duration::from_millis(50))),
        );

    register_components!(app, vec![Hunger, AtMushroom]);

    app.run();
}
