use bevy::{color::palettes::css::*, prelude::*, time::common_conditions::on_timer};
use bevy_dogoap::prelude::*;
use dogoap::prelude::*;
use rand::Rng;
use std::time::Duration;

// This is a basic example on how you can use Dogoap while moving your agent around

// These are just handy strings so we don't fuck it up later.
const HUNGER_KEY: &str = "hunger";
const AT_MUSHROOM: &str = "at_mushroom";

// All the keys for our actions
const EAT_ACTION: &str = "eat";
const GO_TO_MUSHROOM: &str = "go_to_mushroom";

/// This is our marker components, so we can keep track of the various in-game entities
#[derive(Component)]
struct Miner;

#[derive(Component)]
struct Mushroom;

#[derive(Component)]
struct MoveTo(Vec3, Entity);

// Various actions our Miner can perform

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct EatAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct GoToMushroomAction;

// All of our State fields

#[derive(Component, Clone, LocalFieldComponent)]
struct Hunger(f64);

#[derive(Component, Clone, LocalFieldComponent)]
struct AtMushroom(bool);

// UI elements
#[derive(Component)]
struct NeedsText;

fn startup(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.get_single().expect("Expected only one window! Wth");
    let window_height = window.height() / 2.0;
    let window_width = window.width() / 2.0;

    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let goal = Goal::new().with_req(HUNGER_KEY, Compare::LessThanEquals(Field::F64(50.0)));

        let goals = vec![goal.clone()];

        let eat_action = Action::new(EAT_ACTION)
            .with_precondition(AT_MUSHROOM, Compare::Equals(Field::Bool(true)))
            .with_effect(
                Effect::new(EAT_ACTION)
                    .with_mutator(Mutator::Decrement(HUNGER_KEY.to_string(), Field::F64(50.0)))
                    .with_mutator(Mutator::Set(AT_MUSHROOM.to_string(), Field::Bool(false))),
                1,
            );

        let go_to_mushroom_action = simple_action(GO_TO_MUSHROOM, AT_MUSHROOM, Field::Bool(true));

        let actions_map = create_action_map!(
            (EAT_ACTION, eat_action, EatAction),
            (GO_TO_MUSHROOM, go_to_mushroom_action, GoToMushroomAction),
        );

        let hunger = rng.gen_range(0.0..45.0);
        let initial_state = (Hunger(hunger), AtMushroom(false));
        let state = create_state!(Hunger(hunger), AtMushroom(false));

        let mut planner = Planner::new(state, goals, actions_map);

        planner.remove_goal_on_no_plan_found = false; // Don't remove the goal
        planner.always_plan = true; // Re-calculate our plan whenever we can
        planner.current_goal = Some(goal.clone());

        let text_style = TextStyle {
            font_size: 12.0,
            ..default()
        };

        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);

        commands
            .spawn((
                Name::new("Miner"),
                Miner,
                planner,
                initial_state,
                Transform::from_xyz(x, y, 1.0),
                GlobalTransform::from_xyz(x, y, 1.0),
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

    let mut rng = rand::thread_rng();

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

fn spawn_random_mushroom(windows: Query<&Window>, mut commands: Commands, mushrooms: Query<Entity, With<Mushroom>>) {
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

// fn go_to_location<T>(
//     at_location: &mut AtLocation,
//     delta: f32,
//     origin: &mut Transform,
//     destination: Vec3,
//     destination_enum: Location,
//     entity: Entity,
//     commands: &mut Commands,
// ) where
//     T: Component,
// {
//     if origin.translation.distance(destination) > 5.0 {
//         let direction = (destination - origin.translation).normalize();
//         origin.translation += direction * 128.0 * delta;
//     } else {
//         // We're there!
//         at_location.0 = destination_enum as usize;

//         commands.entity(entity).remove::<T>();
//     }
// }

fn handle_move_to(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &MoveTo, &mut Transform)>) {
    for (entity, move_to, mut transform) in query.iter_mut() {
        let destination = move_to.0;
        let destination_entity = move_to.1;

        // Check first if destination entity exists, otherwise cancel the MoveTo,
        match commands.get_entity(destination_entity) {
            Some(_) => {
                if transform.translation.distance(destination) > 5.0 {
                    let direction = (destination - transform.translation).normalize();
                    transform.translation += direction * 128.0 * time.delta_seconds();
                } else {
                    commands.entity(entity).remove::<MoveTo>();
                }
            },
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
    q_mushrooms: Query<(Entity, &Transform), With<Mushroom>>,
) {
    for (entity, _action, t_entity, mut at_mushroom) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_mushrooms.iter().map(|(e, t)| (e, *t)).collect();
        let mushroom = find_closest(origin, items);

        let (e_mushroom, t_mushroom, distance) = match mushroom {
            Some(v) => v,
            None => panic!("No mushroom could be found, HOW?!"),
        };

        if distance > 5.0 {
            commands.entity(entity).insert(MoveTo(t_mushroom, e_mushroom));
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
    items.into_iter().fold(None, |closest, (entity, transform)| {
        let distance = transform.translation.distance(origin);
        match closest {
            Some((_, _, d)) if distance >= d => closest,
            _ => Some((entity, transform.translation, distance)),
        }
    })
}

fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &EatAction,
            &Transform,
            &mut Hunger,
            &mut AtMushroom,
        ),
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
                    hunger.0 -= 25.0;
                    
                    if hunger.0 < 0.0 {
                        hunger.0 = 0.0;
                    }
                    commands.entity(e_mushroom).despawn_recursive();
                },
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

fn over_time_needs_change(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Hunger)>) {
    let mut rng = rand::thread_rng();
    println!("Entities: {}", query.iter().len());
    for (entity, mut hunger) in query.iter_mut() {
        // Increase hunger
        let r = rng.gen_range(10.0..20.0);
        let val: f64 = r * time.delta_seconds_f64();
        hunger.0 += val;
        if hunger.0 > 100.0 {
            // hunger.0 = 100.0;
            commands.entity(entity).despawn_recursive();
            println!("Removed starving Miner");
        }
    }
}

fn print_current_local_state(
    query: Query<(Entity, &Hunger, &Children)>,
    q_actions: Query<(Option<&IsPlanning>, Option<&EatAction>, Option<&GoToMushroomAction>)>,
    mut q_child: Query<&mut Text, With<NeedsText>>,
) {
    // let planner = query.get_single().unwrap();
    for (entity, hunger, children) in query.iter() {
        let hunger = hunger.0;

        let mut current_action = "Idle";

        let (is_planning, eat, go_to_mushroom) = q_actions.get(entity).unwrap();

        if is_planning.is_some() {
            current_action = "Planning...";
        }

        if eat.is_some() {
            current_action = "Eating";
        }

        if go_to_mushroom.is_some() {
            current_action = "Going to mushroom";
        }

        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value = format!("{current_action}\nHunger: {hunger:.0}\nEntity: {entity}");
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
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(DogoapPlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, draw_gizmos)
        .add_systems(Update, (handle_move_to, handle_go_to_mushroom_action, handle_eat_action).chain())
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
        )
        ;

    register_components!(app, vec![Hunger, AtMushroom]);

    app.run();
}
