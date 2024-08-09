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

use bevy::{color::palettes::css::*, prelude::*};
use bevy_dogoap::prelude::*;
use dogoap::prelude::*;

fn main() {
    let mut app = App::new();
    // Customer components + actions
    register_components!(app, vec![Thirst, CarryingItem, PlacedOrder, OrderReady]);
    register_actions!(
        app,
        vec![DrinkLemonade, PickupLemonade, WaitForOrder, PlaceOrder]
    );
    // Worker components + actions
    register_components!(app, vec![Energy]);
    register_actions!(app, vec![Rest]);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#example-canvas".into()),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(DogoapPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, (update_thirst, draw_state_debug, draw_ui))
    // Systems that handle actions
    .add_systems(Update, (handle_pickup_lemonade, handle_drink_lemonade))
    .run();
}

// LocalFields for customer

#[derive(Component, Clone, DatumComponent)]
struct Thirst(f64);

#[derive(Component, Clone, EnumComponent)]
struct CarryingItem(Item);

#[derive(Component, Clone, DatumComponent)]
struct PlacedOrder(bool);

#[derive(Component, Clone, DatumComponent)]
struct OrderReady(bool);

// Actions for customer

#[derive(Component, Clone, Default, ActionComponent)]
struct DrinkLemonade;

#[derive(Component, Clone, Default, ActionComponent)]
struct PickupLemonade;

#[derive(Component, Clone, Default, ActionComponent)]
struct WaitForOrder;

#[derive(Component, Clone, Default, ActionComponent)]
struct PlaceOrder;

// DatumComponents for worker

#[derive(Component, Clone, DatumComponent)]
struct Energy(f64);

#[derive(Component, Clone, Default, ActionComponent)]
struct Rest;

// Markers

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Customer;

#[derive(Component)]
struct Worker;

#[derive(Clone, Default, Copy, Reflect)]
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
    for _i in 0..1 {
        let goal = Goal::from_reqs(&[Thirst::is_less(1.0)]);

        // Requires us to carry a lemonade, results in us having 10 less thirst + carrying Nothing
        let drink_lemonade_action = DrinkLemonade::new()
            .add_precondition(CarryingItem::is(Item::Lemonade))
            .add_mutator(CarryingItem::set(Item::Nothing))
            .add_mutator(Thirst::decrease(10.0));

        // Requires us to not be carrying nothing, and leads to us having a lemonade
        let pickup_lemonade_action = PickupLemonade::new()
            .add_precondition(CarryingItem::is(Item::Nothing))
            .add_mutator(CarryingItem::set(Item::Lemonade));

        let wait_for_order_action = WaitForOrder::new()
            .add_precondition(PlacedOrder::is(true))
            .add_precondition(OrderReady::is(false))
            .add_mutator(OrderReady::set(true));

        let place_order_action = PlaceOrder::new()
            .add_precondition(PlacedOrder::is(false))
            .add_mutator(PlacedOrder::set(true));

        // Drink Lemonade
        // Pickup Lemonade
        // Wait for Order
        // Place Order
        // Go To Order Desk

        let (mut planner, components) = create_planner!({
            actions: [
                (DrinkLemonade, drink_lemonade_action),
                (PickupLemonade, pickup_lemonade_action)
            ],
            state: [Thirst(0.0), CarryingItem(Item::Nothing)],
            goals: [goal],
        });

        planner.remove_goal_on_no_plan_found = false; // Don't remove the goal
        planner.always_plan = true; // Re-calculate our plan whenever we can
        planner.current_goal = Some(goal.clone());

        let t = Transform::from_scale(Vec3::splat(2.0));

        commands
            .spawn((
                Agent,
                Name::new("Customer"),
                Customer,
                planner,
                components,
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
    for _i in 0..1 {
        let goal = Goal::from_reqs(&[Energy::is_more(1.0)]);

        let rest_action = Rest::new()
            .add_precondition(Energy::is_less(10.0))
            .add_mutator(Energy::increase(50.0));

        let (planner, components) = create_planner!({
            actions: [(Rest, rest_action)],
            state: [Energy(50.0)],
            goals: [goal],
        });

        commands
            .spawn((
                Agent,
                Name::new("Worker"),
                Worker,
                planner,
                components,
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
    for (entity, _action, mut state) in query.iter_mut() {
        match progresses.get_mut(&entity) {
            Some(progress) => {
                if progress.tick(time.delta()).just_finished() {
                    state.0 = Item::Lemonade;
                    commands.entity(entity).remove::<PickupLemonade>();
                    progresses.remove(&entity);
                } else {
                    // In progress...
                    println!("Pickup Progress: {}", progress.fraction());
                }
            }
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
    for (entity, _action, mut state, mut thirst) in query.iter_mut() {
        match progresses.get_mut(&entity) {
            Some(progress) => {
                if progress.tick(time.delta()).just_finished() {
                    state.0 = Item::Nothing;
                    thirst.0 = (thirst.0 - 5.0).max(0.0);

                    commands.entity(entity).remove::<DrinkLemonade>();
                    progresses.remove(&entity);
                } else {
                    // In progress...
                    println!("Drink Progress: {}", progress.fraction());
                }
            }
            None => {
                progresses.insert(entity, Timer::from_seconds(1.0, TimerMode::Once));
            }
        }
    }
}

fn update_thirst(time: Res<Time>, mut query: Query<&mut Thirst>) {
    for mut thirst in query.iter_mut() {
        thirst.0 += time.delta_seconds_f64() * 0.3;
        if thirst.0 > 100.0 {
            thirst.0 = 100.0;
        }
    }
}

fn draw_state_debug(
    q_planners: Query<(Entity, &Name, &Children), With<Planner>>,
    // q_workers: Query<(Entity, &Name, &Children), With<Worker>>,
    q_actions: Query<(Entity, &dyn ActionComponent)>,
    q_datums: Query<(Entity, &dyn DatumComponent)>,
    // q_worker_actions: Query<(Option<>)>
    // q_actions: Query<(
    //     Option<&IsPlanning>,
    //     Option<&EatAction>,
    //     Option<&GoToMushroomAction>,
    //     Option<&ReplicateAction>,
    // )>,
    mut q_child: Query<&mut Text, With<StateDebugText>>,
) {
    for (entity, name, children) in q_planners.iter() {
        let mut current_action = "Idle";

        // Get current action, should always be one so grab the first one we find
        for (_entity, actions) in q_actions.get(entity).iter() {
            for action in actions.iter() {
                current_action = action.action_type_name();
                break;
            }
        }

        // Concat all the found DatumComponents for this entity
        let mut state: String = "".to_string();
        for (_entity, data) in q_datums.get(entity).iter() {
            for datum in data.iter() {
                state = format!(
                    "{}\n{}: {}",
                    state,
                    datum.field_key().to_string(),
                    match datum.field_value() {
                        Datum::Bool(v) => v.to_string(),
                        Datum::F64(v) => format!("{:.2}", v).to_string(),
                        Datum::I64(v) => format!("{}", v).to_string(),
                        Datum::Enum(v) => format!("{}", v).to_string(),
                    }
                );
            }
        }

        // Render it out
        for &child in children.iter() {
            let mut text = q_child.get_mut(child).unwrap();
            text.sections[0].value =
                format!("{name}\n{current_action}\nEntity: {entity}\n---\n{state}");
        }
    }
    // for (entity, name, children) in q_workers.iter() {
    //     let current_action = "Idle";

    //     // let (is_planning, eat, go_to_mushroom, replicate) = q_actions.get(entity).unwrap();

    //     // if is_planning.is_some() {
    //     //     current_action = "Planning...";
    //     // }

    //     for &child in children.iter() {
    //         let mut text = q_child.get_mut(child).unwrap();
    //         text.sections[0].value = format!("{name}\n{current_action}\nEntity: {entity}");
    //     }
    // }
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
