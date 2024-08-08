use bevy::{color::palettes::css::*, prelude::*, time::common_conditions::on_timer};
use bevy_dogoap::prelude::*;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

// This is a basic example on how you can use Dogoap while moving your agent around

/// This is our marker components, so we can keep track of the various in-game entities
#[derive(Component)]
struct Cell {
    speed: f32,
    age: usize,
}

#[derive(Component)]
struct DeadCell;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct MoveTo(Vec3, Entity);

//
// Various actions our Cell can perform
//

// When cell is at food, the cell can consume the food, decreasing hunger
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct EatAction;

// When we're not hungry, our cell can replicate itself
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct ReplicateAction;

// This will make the cell seek out the closest food
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToFoodAction;

//
// All of our State fields
//

#[derive(Component, Clone, DatumComponent)]
struct Hunger(f64);

#[derive(Component, Clone, DatumComponent)]
struct AtFood(bool);

#[derive(Component, Clone, DatumComponent)]
struct IsReplicating(bool);

// UI elements
#[derive(Component)]
struct StateDebugText;

fn spawn_cell(commands: &mut Commands, position: Vec3, speed: f32) {
    let goal = Goal::from_reqs(&[IsReplicating::is(true)]);

    let eat_action = EatAction::new()
        .add_precondition(AtFood::is(true))
        .add_mutator(Hunger::decrease(10.0))
        .add_mutator(AtFood::set(true))
        .set_cost(1);

    let replicate_action = ReplicateAction::new()
        .add_precondition(Hunger::is_less(10.0))
        .add_mutator(IsReplicating::set(true))
        .add_mutator(Hunger::increase(25.0))
        .set_cost(10);

    let go_to_food_action = GoToFoodAction::new()
        .add_precondition(AtFood::is(false))
        .add_mutator(AtFood::set(true))
        .add_mutator(Hunger::increase(1.0))
        .set_cost(2);

    let mut rng = rand::thread_rng();
    let starting_hunger = rng.gen_range(20.0..45.0);

    let (mut planner, components) = create_planner!({
        actions: [
            (EatAction, eat_action),
            (GoToFoodAction, go_to_food_action),
            (ReplicateAction, replicate_action)
        ],
        state: [Hunger(starting_hunger), AtFood(false), IsReplicating(false)],
        goals: [goal],
    });

    planner.remove_goal_on_no_plan_found = false; // Don't remove the goal
    planner.always_plan = true; // Re-calculate our plan whenever we can
    planner.current_goal = Some(goal.clone());

    let text_style = TextStyle {
        font_size: 12.0,
        ..default()
    };

    commands
        .spawn((
            Name::new("Cell"),
            Cell { speed, age: 0 },
            planner,
            components,
            Transform::from_translation(position),
            GlobalTransform::from_translation(position),
            InheritedVisibility::default(),
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
        spawn_cell(&mut commands, Vec3::from_array([x, y, 1.0]), 128.0);
    }

    // Begin with three food
    for _i in 0..30 {
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Food"),
            Food,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
    // Misc stuff we want somewhere
    commands.spawn(Camera2dBundle::default());
}

fn spawn_random_food(
    windows: Query<&Window>,
    mut commands: Commands,
    q_food: Query<Entity, With<Food>>,
) {
    let window = windows.get_single().expect("Expected only one window! Wth");
    let window_height = window.height() / 2.0;
    let window_width = window.width() / 2.0;

    if q_food.iter().len() < 100 {
        let mut rng = rand::thread_rng();
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Food"),
            Food,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
}

fn handle_move_to(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Cell, &MoveTo, &mut Transform)>,
) {
    for (entity, cell, move_to, mut transform) in query.iter_mut() {
        let destination = move_to.0;
        let destination_entity = move_to.1;

        // Check first if destination entity exists, otherwise cancel the MoveTo,
        match commands.get_entity(destination_entity) {
            Some(_) => {
                if transform.translation.distance(destination) > 5.0 {
                    let direction = (destination - transform.translation).normalize();
                    transform.translation += direction * cell.speed * time.delta_seconds();
                } else {
                    commands.entity(entity).remove::<MoveTo>();
                    // commands.entity(destination_entity).remove::<BusyObject>();
                }
            }
            None => {
                // Cancel the MoveTo order as the destination no longer exists...
                commands.entity(entity).remove::<MoveTo>();
            }
        }
    }
}

fn handle_go_to_food_action(
    mut commands: Commands,
    mut query: Query<
        (Entity, &GoToFoodAction, &Transform, &mut AtFood),
        (Without<Food>, Without<MoveTo>),
    >,
    q_food: Query<(Entity, &Transform), With<Food>>,
    mut targeted_food: Local<HashMap<Entity, Entity>>,
) {
    for (entity, _action, t_entity, mut at_food) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_food.iter().map(|(e, t)| (e, *t)).collect();

        let foods = find_closest(origin, items);

        let mut selected_food = None;
        for (e_food, t_food, distance) in foods.iter() {
            match targeted_food.get(e_food) {
                Some(cell_entity) if *cell_entity == entity => {
                    // This food is targeted by us, select it
                    selected_food = Some((e_food, t_food, distance));
                    break;
                }
                Some(_) => {
                    // This food is targeted by another entity, skip it
                    continue;
                }
                None => {
                    // This food is not targeted, select it
                    selected_food = Some((e_food, t_food, distance));
                    break;
                }
            }
        }

        let (e_food, t_food, distance) = match selected_food {
            Some(v) => v,
            None => {
                // No available food found, do nothing
                continue;
            }
        };

        targeted_food.insert(*e_food, entity);

        if *distance > 5.0 {
            // commands.entity(e_food).insert(BusyObject(entity));
            commands.entity(entity).insert(MoveTo(*t_food, *e_food));
        } else {
            // Consume food!
            at_food.0 = true;
            commands.entity(entity).remove::<GoToFoodAction>();
            targeted_food.remove(&e_food);
        }
    }
}

fn find_closest(origin: Vec3, items: Vec<(Entity, Transform)>) -> Vec<(Entity, Vec3, f32)> {
    let mut closest: Vec<(Entity, Vec3, f32)> = items
        .into_iter()
        .map(|(entity, transform)| {
            let distance = transform.translation.distance(origin);
            (entity, transform.translation, distance)
        })
        .collect();

    closest.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    closest.truncate(10);
    closest
}

fn handle_replicate_action(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &ReplicateAction,
        &mut IsReplicating,
        &mut Hunger,
        &mut Planner,
        &Cell,
        &Transform,
    )>,
    mut timers: Local<HashMap<Entity, Timer>>,
    time: Res<Time>,
) {
    for (entity, _action, _field, mut hunger, mut planner, cell, transform) in query.iter_mut() {
        match timers.get_mut(&entity) {
            Some(progress) => {
                if progress.tick(time.delta()).just_finished() {
                    let new_transform = transform.translation + Vec3::from_array([25., 0., 0.]);
                    spawn_cell(&mut commands, new_transform, cell.speed);
                    commands.entity(entity).remove::<ReplicateAction>();
                    hunger.0 += 20.0;
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
    mut query: Query<(Entity, &EatAction, &Transform, &mut Hunger, &mut AtFood), Without<Food>>,
    q_food: Query<(Entity, &Transform), With<Food>>,
) {
    // println!("Query hits: {}", query.iter().len());
    for (entity, _action, t_entity, mut hunger, mut at_food) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_food.iter().map(|(e, t)| (e, *t)).collect();
        let foods = find_closest(origin, items);
        let food = foods.first();

        // println!("Eating food we found at {:?}", food);

        let (e_food, _t_food, distance) = match food {
            Some(v) => v,
            None => panic!("No food could be found, HOW?!"),
        };

        // Make sure we're actually in range to consume this food
        // If not, remove the EatAction to cancel it, and the planner
        // will figure out what to do next
        if *distance < 5.0 {
            // Before we consume this food, make another query to ensure
            // it's still there, as it could have been consumed by another
            // Cell in the same frame, during the query.iter() loop
            match q_food.get(*e_food) {
                Ok(_) => {
                    hunger.0 -= 10.0;

                    if hunger.0 < 0.0 {
                        hunger.0 = 0.0;
                    }
                    commands.entity(*e_food).despawn_recursive();
                }
                // Don't consume as it doesn't exists
                Err(_) => {
                    warn!("Tried to consume non-existing food");
                }
            }
        }

        commands.entity(entity).remove::<EatAction>();
        at_food.0 = false;
    }
}

fn print_cell_count(query: Query<Entity, With<Cell>>) {
    println!("Active Cells: {}", query.iter().len());
}

fn over_time_needs_change(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Hunger, &Transform)>,
) {
    let mut rng = rand::thread_rng();
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
                DeadCell,
                Transform::from_translation(translation),
                GlobalTransform::from_translation(translation),
            ));
            println!("Removed starving Cell");
        }
    }
}

fn print_current_local_state(
    query: Query<(Entity, &Cell, &Hunger, &Children)>,
    q_actions: Query<(
        Option<&IsPlanning>,
        Option<&EatAction>,
        Option<&GoToFoodAction>,
        Option<&ReplicateAction>,
    )>,
    mut q_child: Query<&mut Text, With<StateDebugText>>,
) {
    // let planner = query.get_single().unwrap();
    for (entity, cell, hunger, children) in query.iter() {
        let age = cell.age;
        let hunger = hunger.0;

        let mut current_action = "Idle";

        let (is_planning, eat, go_to_food, replicate) = q_actions.get(entity).unwrap();

        if is_planning.is_some() {
            current_action = "Planning...";
        }

        if eat.is_some() {
            current_action = "Eating";
        }

        if go_to_food.is_some() {
            current_action = "Going to food";
        }

        if replicate.is_some() {
            current_action = "Replicating";
        }

        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value =
                format!("{current_action}\nAge: {age}\nHunger: {hunger:.0}\nEntity: {entity}");
        }
    }
}

// Worlds shittiest graphics incoming, beware and don't copy
fn draw_gizmos(
    mut gizmos: Gizmos,
    q_cell: Query<(&Transform, &Cell)>,
    q_dead: Query<&Transform, With<DeadCell>>,
    q_food: Query<&Transform, With<Food>>,
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

    for (cell_transform, cell) in q_cell.iter() {
        let color = NAVY;
        color.lighter((cell.age / 100) as f32);
        gizmos.circle_2d(cell_transform.translation.truncate(), 12., color);
    }

    for food_transform in q_food.iter() {
        gizmos.circle_2d(food_transform.translation.truncate(), 4., GREEN_YELLOW);
    }

    for cell_transform in q_dead.iter() {
        gizmos.circle_2d(
            cell_transform.translation.truncate(),
            12.,
            Srgba::new(1.0, 0.0, 0.0, 0.1),
        );
    }
}

fn increment_age(mut query: Query<&mut Cell>) {
    for mut cell in query.iter_mut() {
        cell.age += 1;
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
            FixedUpdate,
            (
                handle_move_to,
                handle_go_to_food_action,
                handle_eat_action,
                handle_replicate_action,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            spawn_random_food.run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(
            FixedUpdate,
            over_time_needs_change.run_if(on_timer(Duration::from_millis(100))),
        )
        .add_systems(
            FixedUpdate,
            print_cell_count.run_if(on_timer(Duration::from_millis(1000))),
        )
        .add_systems(
            FixedUpdate,
            increment_age.run_if(on_timer(Duration::from_millis(1000))),
        )
        .add_systems(
            FixedUpdate,
            print_current_local_state.run_if(on_timer(Duration::from_millis(50))),
        );

    register_components!(app, vec![Hunger, AtFood]);

    app.run();
}
