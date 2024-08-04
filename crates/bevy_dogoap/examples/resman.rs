// A little restuarant manager

// Customer > Has Thirst that they want to fullfil
// Worker > Wants to fulfill orders to increase profits of business

// High-level, we have the following:

// Agent - Shared behaviour between Customer and Worker
// Customer - Has Thirst, wants to satisfy it somehow
// Worker - Wants to increase income of business

// Customer has Actions:
// - GoToServingDesk, MakeOrder, ConsumeOrder, ConsumeInventory

// Worker has Actions:
// - GoToServingDesk, TakeOrder, MakeProduct, MoveProduct, HandOverOrder

use std::collections::HashMap;

use bevy::{color::palettes::css::*, prelude::*, transform::commands};
use bevy_dogoap::{create_action_map_v2, prelude::*};
use dogoap::prelude::*;

fn main() {
    let mut app = App::new();
    register_components!(app, vec![Thirst, CarryingItem]);

    app.add_plugins(DefaultPlugins)
        .add_plugins(DogoapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_thirst, draw_state_debug, draw_ui))
        // Systems that handle actions
        .add_systems(Update, (handle_pickup_lemonade, handle_drink_lemonade))
        .run();
}

// LocalFields

#[derive(Component, Clone, DatumComponent)]
struct Thirst(f64);

#[derive(Component, Clone, DatumComponent)]
struct CarryingItem(usize); // `Items::Lemonade as usize` for example

// Actions

#[derive(Component, Clone, Default, ActionComponent)]
struct DrinkLemonade();

#[derive(Component, Clone, Default, ActionComponent)]
struct PickupLemonade();

// Markers

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Customer;

#[derive(Component)]
struct Worker;

#[derive(Clone, Default, Reflect)]
enum Item {
    #[default]
    Nothing,
    // Actual items:
    Lemonade,
}

#[derive(Component)]
struct LemonadeMaker;

#[derive(Component)]
struct OrderDesk;

#[derive(Component)]
struct StateDebugText;

fn setup(mut commands: Commands) {
    // Spawn customers
    for i in 0..1 {
        let goal = Goal::new().with_req(&Thirst::key(), Compare::LessThanEquals(Datum::F64(1.0)));

        let goals = vec![goal.clone()];

        // Requires us to carry a lemonade, results in us having 10 less thirst + carrying Nothing
        let drink_lemonade_action = Action::new(&DrinkLemonade::key())
            .with_precondition(
                &CarryingItem::key(),
                Compare::Equals(Datum::Enum(Item::Lemonade as usize)),
            )
            .with_effect(
                Effect::new(&DrinkLemonade::key())
                    .with_mutator(Mutator::Set(
                        CarryingItem::key(),
                        Datum::Enum(Item::Nothing as usize),
                    ))
                    .with_mutator(Mutator::Decrement(Thirst::key(), Datum::F64(10.0))),
                1,
            );

        // Requires us to not be carrying nothing, and leads to us having a lemonade
        let pickup_lemonade_action = Action::new(&PickupLemonade::key())
            .with_precondition(
                &CarryingItem::key(),
                Compare::Equals(Datum::Enum(Item::Nothing as usize)),
            )
            .with_effect(
                Effect::new(&PickupLemonade::key())
                    .with_mutator(Mutator::Set(
                        CarryingItem::key(),
                        Datum::Enum(Item::Lemonade as usize),
                    )),
                1,
            );

        let actions_map = create_action_map_v2!(
            (DrinkLemonade, drink_lemonade_action),
            (PickupLemonade, pickup_lemonade_action)
        );

        let initial_state = (Thirst(0.0), CarryingItem(Item::Nothing as usize));
        let state = create_state!(Thirst(0.0), CarryingItem(Item::Nothing as usize));

        let mut planner = Planner::new(state, goals, actions_map);

        planner.remove_goal_on_no_plan_found = false; // Don't remove the goal
        planner.always_plan = true; // Re-calculate our plan whenever we can
        planner.current_goal = Some(goal.clone());

        commands
            .spawn((
                Agent,
                Name::new("Customer"),
                Customer,
                planner,
                initial_state,
                TransformBundle::from(Transform::from_xyz(-200.0, -100.0, 1.0)),
            ))
            .with_children(|subcommands| {
                subcommands.spawn((
                    Text2dBundle {
                        transform: Transform::from_translation(Vec3::new(10.0, -10.0, 10.0)),
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font_size: 12.0,
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Left),
                        text_anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                    StateDebugText,
                ));
            });
    }

    // Spawn worker
    for i in 0..1 {
        commands
            .spawn((
                Agent,
                Name::new("Worker"),
                Worker,
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)),
            ))
            .with_children(|subcommands| {
                subcommands.spawn((
                    Text2dBundle {
                        transform: Transform::from_translation(Vec3::new(10.0, -10.0, 10.0)),
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font_size: 12.0,
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Left),
                        text_anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                    StateDebugText,
                ));
            });
    }

    commands.spawn((
        LemonadeMaker,
        TransformBundle::from(Transform::from_xyz(100.0, 0.0, 1.0)),
    ));

    commands.spawn((
        OrderDesk,
        TransformBundle::from(Transform::from_xyz(-100.0, 0.0, 1.0)),
    ));

    commands.spawn(Camera2dBundle::default());
}

fn handle_pickup_lemonade(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &PickupLemonade, &mut CarryingItem)>,
    mut progresses: Local<HashMap<Entity, Timer>>,
) {
    for (entity, action, mut state) in query.iter_mut() {
        match progresses.get_mut(&entity) {
            Some(progress) => {
                if progress.tick(time.delta()).just_finished() {
                    state.0 = Item::Lemonade as usize;
                    commands.entity(entity).remove::<PickupLemonade>();
                    progresses.remove(&entity);
                    
                } else {
                    // In progress...
                    println!("Pickup Progress: {}", progress.fraction());
                }
            },
            None => {
                progresses.insert(entity, Timer::from_seconds(1.0, TimerMode::Once));
            }
        }

    }
}

fn handle_drink_lemonade(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &DrinkLemonade, &mut CarryingItem, &mut Thirst)>,
    mut progresses: Local<HashMap<Entity, Timer>>,
) {
    for (entity, action, mut state, mut thirst) in query.iter_mut() {
        match progresses.get_mut(&entity) {
            Some(progress) => {
                if progress.tick(time.delta()).just_finished() {

                    state.0 = Item::Nothing as usize;
                    thirst.0 = (thirst.0 - 5.0).max(0.0);                    

                    commands.entity(entity).remove::<DrinkLemonade>();
                    progresses.remove(&entity);
                } else {
                    // In progress...
                    println!("Drink Progress: {}", progress.fraction());
                }
            },
            None => {
                progresses.insert(entity, Timer::from_seconds(1.0, TimerMode::Once));
            }
        }

    }
}

fn update_thirst(time: Res<Time>, mut query: Query<(&mut Thirst)>) {
    for mut thirst in query.iter_mut() {
        thirst.0 += time.delta_seconds_f64() * 0.3;
        if thirst.0 > 100.0 {
            thirst.0 = 100.0;
        }
    }
}

fn draw_state_debug(
    q_customers: Query<(Entity, &Name, &Thirst, &Children), With<Customer>>,
    q_customer_actions: Query<(Option<&DrinkLemonade>, Option<&PickupLemonade>)>,
    q_workers: Query<(Entity, &Name, &Children), With<Worker>>,
    // q_worker_actions: Query<(Option<>)>
    // q_actions: Query<(
    //     Option<&IsPlanning>,
    //     Option<&EatAction>,
    //     Option<&GoToMushroomAction>,
    //     Option<&ReplicateAction>,
    // )>,
    mut q_child: Query<&mut Text, With<StateDebugText>>,
) {
    for (entity, name, thirst, children) in q_customers.iter() {
        let thirst = thirst.0;

        let mut current_action = "Idle";

        let (drink_lemonade_action, pickup_lemonade_action) = q_customer_actions.get(entity).unwrap();

        if drink_lemonade_action.is_some() {
            current_action = "Drinking Lemonade";
        }

        if pickup_lemonade_action.is_some() {
            current_action = "Picking up Lemonade";
        }

        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value =
                format!("{name}\n{current_action}\nThirst: {thirst:.2}\nEntity: {entity}");
        }
    }
    for (entity, name, children) in q_workers.iter() {
        let mut current_action = "Idle";

        // let (is_planning, eat, go_to_mushroom, replicate) = q_actions.get(entity).unwrap();

        // if is_planning.is_some() {
        //     current_action = "Planning...";
        // }

        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value = format!("{name}\n{current_action}\nEntity: {entity}");
        }
    }
}

fn draw_ui(
    mut gizmos: Gizmos,
    q_customer: Query<&Transform, With<Customer>>,
    q_workers: Query<&Transform, With<Worker>>,
    q_lemonade_makers: Query<&Transform, With<LemonadeMaker>>,
    q_order_desks: Query<&Transform, With<OrderDesk>>,
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

    for transform in q_customer.iter() {
        gizmos.circle_2d(transform.translation.xy(), 16., GREEN);
    }

    for transform in q_workers.iter() {
        gizmos.circle_2d(transform.translation.xy(), 16., BLUE);
    }

    for transform in q_lemonade_makers.iter() {
        gizmos.rect_2d(transform.translation.xy(), 0.0, Vec2::new(20.0, 20.0), GOLD);
    }

    for transform in q_order_desks.iter() {
        gizmos.rect_2d(
            transform.translation.xy(),
            0.0,
            Vec2::new(20.0, 20.0),
            BLUE_VIOLET,
        );
    }
}
