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

/* Sequence Diagram for the full flow of actions (paste into https://sequencediagram.org/)

Customer->Order Desk: GoToOrderDesk
Order Desk->Worker: RequestWorker
Worker->Order Desk: GoToOrderDesk
Customer->Order Desk: PlaceOrder
Worker->Order Desk: TakeOrder
Customer->Order Desk: WaitForOrder
Worker->Lemonade Maker: GoToLemonadeMaker
Lemonade Maker->Worker: MakeLemonade
Worker->Order Desk: FinishOrder
Customer->Order Desk: PickupLemonade
Customer->Customer: DrinkLemonade

*/

use std::collections::{HashMap, VecDeque};

use bevy::{color::palettes::css::*, input::common_conditions::input_toggle_active, prelude::*};
use bevy_dogoap::prelude::*;

fn main() {
    let mut app = App::new();
    // Customer components + actions
    register_components!(
        app,
        vec![
            Thirst,
            CarryingItem,
            PlacedOrder,
            OrderReady,
            AtOrderDesk,
            ShouldGoToOrderDesk
        ]
    );
    register_actions!(
        app,
        vec![
            DrinkLemonade,
            PickupLemonade,
            WaitForOrder,
            PlaceOrder,
            GoToOrderDesk
        ]
    );
    // Worker components + actions
    register_components!(
        app,
        vec![Energy, AtLemonadeMaker, ServedOrder, ShouldGoToOrderDesk]
    );
    register_actions!(
        app,
        vec![Rest, ServeOrder, ProduceLemonade, GoToLemonadeMaker]
    );

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#example-canvas".into()),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(DogoapPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, (draw_state_debug, draw_ui))
    // Systems that always affects needs
    .add_systems(FixedUpdate, update_thirst)
    // Systems that handle actions
    .add_systems(
        FixedUpdate,
        (
            handle_pickup_lemonade,
            handle_drink_lemonade,
            handle_place_order,
            handle_wait_for_order,
            handle_go_to_order_desk,
            handle_move_to,
            handle_call_worker_to_empty_order_desk,
        ),
    )
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

#[derive(Component, Clone, DatumComponent)]
struct AtOrderDesk(bool);

#[derive(Component, Clone, DatumComponent)]
struct ShouldGoToOrderDesk(bool);

// Actions for customer

#[derive(Component, Clone, Default, ActionComponent)]
struct DrinkLemonade;

#[derive(Component, Clone, Default, ActionComponent)]
struct PickupLemonade;

#[derive(Component, Clone, Default, ActionComponent)]
struct WaitForOrder;

#[derive(Component, Clone, Default, ActionComponent)]
struct PlaceOrder;

#[derive(Component, Clone, Default, ActionComponent)]
struct GoToOrderDesk;

// DatumComponents for worker

#[derive(Component, Clone, DatumComponent)]
struct Energy(f64);

#[derive(Component, Clone, DatumComponent)]
struct ServedOrder(bool);

#[derive(Component, Clone, DatumComponent)]
struct AtLemonadeMaker(bool);

// Actions for worker

#[derive(Component, Clone, Default, ActionComponent)]
struct Rest;

#[derive(Component, Clone, Default, ActionComponent)]
struct ServeOrder;

#[derive(Component, Clone, Default, ActionComponent)]
struct ProduceLemonade;

#[derive(Component, Clone, Default, ActionComponent)]
struct GoToLemonadeMaker;

// Markers

#[derive(Component)]
struct Agent;

#[derive(Component, Default)]
struct Customer {
    order: Option<Entity>,
}

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
struct Order {
    items_to_produce: VecDeque<Item>,
    items: Vec<Item>,
    owner: Entity,
}

#[derive(Component, Default)]
struct OrderDesk {
    assigned_customer: Option<Entity>,
    assigned_worker: Option<Entity>,
    can_take_order: bool, // set to true when both customer and worker present
    current_order: Option<Entity>,
}

#[derive(Component)]
struct MoveTo(Vec3);

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
            .add_precondition(OrderReady::is(true))
            .add_precondition(AtOrderDesk::is(true))
            .add_mutator(CarryingItem::set(Item::Lemonade));

        // Requires us to having placed an order, order not yet ready and we're at the order desk
        let wait_for_order_action = WaitForOrder::new()
            .add_precondition(PlacedOrder::is(true))
            .add_precondition(OrderReady::is(false))
            .add_precondition(AtOrderDesk::is(true))
            .add_mutator(OrderReady::set(true));

        // Requires us to not having placed an order previously, and we're at the ordering desk
        let place_order_action = PlaceOrder::new()
            .add_precondition(PlacedOrder::is(false))
            .add_precondition(AtOrderDesk::is(true))
            .add_mutator(PlacedOrder::set(true));

        let go_to_order_desk_action = GoToOrderDesk::new()
            .add_precondition(AtOrderDesk::is(false))
            .add_mutator(AtOrderDesk::set(true));

        let (mut planner, components) = create_planner!({
            actions: [
                (DrinkLemonade, drink_lemonade_action),
                (PickupLemonade, pickup_lemonade_action),
                (WaitForOrder, wait_for_order_action),
                (PlaceOrder, place_order_action),
                (GoToOrderDesk, go_to_order_desk_action),
            ],
            state: [
                Thirst(0.0),
                CarryingItem(Item::Nothing),
                PlacedOrder(false),
                OrderReady(false),
                AtOrderDesk(false),
            ],
            goals: [goal],
        });

        planner.remove_goal_on_no_plan_found = false; // Don't remove the goal
        planner.always_plan = true; // Re-calculate our plan whenever we can
        planner.current_goal = Some(goal.clone());

        commands
            .spawn((
                Agent,
                Name::new("Customer"),
                Customer::default(),
                planner,
                components,
                TransformBundle::from(Transform::from_xyz(-200.0, -100.0, 1.0)),
            ))
            .with_children(|subcommands| {
                subcommands.spawn((
                    Text2dBundle {
                        transform: Transform::from_translation(Vec3::new(-70.0, 0.0, 10.0)),
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
        // Now for the worker

        // Final outcome for the worker is increasing the amount of money, always
        // We trick the agent into performing our actions forever by having a:
        // ServedOrder DatumComponent that the agent wants to set to true,
        // but at runtime it can never actually get there.

        // In order to set ServedOrder to true, the agent needs to run ServeOrder

        // let goal = Goal::from_reqs(&[Energy::is_more(1.0), ServedOrder::is(true)]);
        let goal = Goal::from_reqs(&[AtOrderDesk::is(true)]);

        let serve_order_action = ServeOrder::new()
            .add_precondition(CarryingItem::is(Item::Lemonade))
            .add_precondition(AtOrderDesk::is(true))
            .add_mutator(ServedOrder::set(true));

        let produce_lemonade_action = ProduceLemonade::new()
            .add_precondition(CarryingItem::is(Item::Nothing))
            .add_precondition(AtLemonadeMaker::is(true))
            .add_mutator(CarryingItem::set(Item::Lemonade));

        let go_to_lemonade_maker_action = GoToLemonadeMaker::new()
            .add_precondition(AtLemonadeMaker::is(false))
            .add_mutator(AtLemonadeMaker::set(true));

        let rest_action = Rest::new()
            .add_precondition(Energy::is_less(10.0))
            .add_mutator(Energy::increase(50.0));

        let go_to_order_desk_action = GoToOrderDesk::new()
            .add_precondition(AtOrderDesk::is(false))
            .add_precondition(ShouldGoToOrderDesk::is(true))
            .add_mutator(AtOrderDesk::set(true));

        let (planner, components) = create_planner!({
            actions: [
                (Rest, rest_action),
                (ServeOrder, serve_order_action),
                (ProduceLemonade, produce_lemonade_action),
                (GoToLemonadeMaker, go_to_lemonade_maker_action),
                (GoToOrderDesk, go_to_order_desk_action),
            ],
            state: [
                Energy(50.0),
                ServedOrder(false),
                AtLemonadeMaker(false),
                AtOrderDesk(false),
                CarryingItem(Item::Nothing),
                ShouldGoToOrderDesk(false),
            ],
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

    commands
        .spawn((
            Name::new("Lemonade Maker"),
            LemonadeMaker,
            TransformBundle::from(Transform::from_xyz(100.0, 0.0, 1.0)),
        ))
        .with_children(|subcommands| {
            subcommands.spawn((
                Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 25.0, 10.0)),
                    text: Text::from_section(
                        "Lemonade Maker",
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

    commands
        .spawn((
            Name::new("Order Desk"),
            OrderDesk::default(),
            TransformBundle::from(Transform::from_xyz(-100.0, 0.0, 1.0)),
        ))
        .with_children(|subcommands| {
            subcommands.spawn((
                Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 50.0, 10.0)),
                    text: Text::from_section(
                        "Order Desk",
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

    commands.spawn(Camera2dBundle::default());
}

fn handle_call_worker_to_empty_order_desk(
    mut commands: Commands,
    mut q_order_desks: Query<(&mut OrderDesk, &Transform)>,
    mut q_workers: Query<
        (Entity, &mut ShouldGoToOrderDesk, &Transform),
        (With<Worker>, Without<GoToOrderDesk>),
    >,
) {
    for (mut order_desk, t_order_desk) in q_order_desks.iter_mut() {
        if order_desk.assigned_customer.is_some() && order_desk.assigned_worker.is_none() {
            // This order desk needs a worker!
            // TODO continue here!
            let (mut worker, mut should_go, t_worker) =
                q_workers.iter_mut().next().expect("no workers");
            should_go.0 = true;
            // Do we want to assign this directly? Or move there first
            // if t_worker.translation.distance(t_order_desk.translation) < 5.0 {
            order_desk.assigned_worker = Some(worker);
            // }
        }
    }
}

fn handle_move_to(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &MoveTo, &mut Transform)>,
) {
    for (entity, move_to, mut transform) in query.iter_mut() {
        let destination = move_to.0;

        if transform.translation.distance(destination) > 5.0 {
            // If we're further away than 5 units, move closer
            let direction = (destination - transform.translation).normalize();
            transform.translation += direction * 96.0 * time.delta_seconds();
        } else {
            // If we're within 5 units, assume the MoveTo completed
            commands.entity(entity).remove::<MoveTo>();
        }
    }
}

fn handle_go_to_order_desk(
    mut commands: Commands,
    mut q_order_desks: Query<(&Transform, &mut OrderDesk)>,
    mut query: Query<
        (
            Entity,
            &Transform,
            &GoToOrderDesk,
            &mut AtOrderDesk,
            Option<&Customer>,
        ),
        Without<MoveTo>,
    >,
) {
    for (entity, transform, _action, mut state, customer) in query.iter_mut() {
        let (t_order_desk, mut order_desk) = q_order_desks
            .get_single_mut()
            .expect("Only one order desk expected!");

        // Offset to the left for customer, to the right for worker
        let with_offset = match customer {
            Some(_) => t_order_desk.translation + Vec3::new(-50.0, 0.0, 0.0),
            None => t_order_desk.translation + Vec3::new(50.0, 0.0, 0.0),
        };

        let distance = with_offset.distance(transform.translation);

        if distance > 5.0 {
            commands.entity(entity).insert(MoveTo(with_offset));
        } else {
            state.0 = true;
            commands.entity(entity).remove::<GoToOrderDesk>();

            match customer {
                Some(_) => order_desk.assigned_customer = Some(entity),
                None => order_desk.can_take_order = true,
            };
        }
    }
}

fn handle_wait_for_order(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Customer, &WaitForOrder, &mut OrderReady)>,
    q_order: Query<&Order>,
    // mut progresses: Local<HashMap<Entity, Timer>>,
) {
    for (entity, customer, _action, mut state) in query.iter_mut() {
        match customer.order {
            Some(e_order) => {
                let order = q_order.get(e_order).expect("Impossible!");
                if order.items_to_produce.is_empty() {
                    // ORder is ready! Destroy and move on
                } else {
                    // Order not yet ready
                }
            }
            None => {
                // Shouldn't be possible!
            }
        }
        // match progresses.get_mut(&entity) {
        //     Some(progress) => {
        //         if progress.tick(time.delta()).just_finished() {
        //             state.0 = true;
        //             commands.entity(entity).remove::<WaitForOrder>();
        //             progresses.remove(&entity);
        //         } else {
        //             // In progress...
        //             println!("WaitOrder Progress: {}", progress.fraction());
        //         }
        //     }
        //     None => {
        //         progresses.insert(entity, Timer::from_seconds(1.0, TimerMode::Once));
        //     }
        // }
    }
}

fn handle_place_order(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Customer, &PlaceOrder, &mut PlacedOrder)>,
    mut q_order_desks: Query<&mut OrderDesk>,
    mut progresses: Local<HashMap<Entity, Timer>>,
) {
    for (entity, mut customer, _action, mut placed_order) in query.iter_mut() {
        let mut order_desk = q_order_desks
            .get_single_mut()
            .expect("Only one order desk expected!");
        // Need to make sure the serving counter has a worker at it before we
        // can place an order
        if order_desk.assigned_worker.is_some() && order_desk.can_take_order {
            match progresses.get_mut(&entity) {
                Some(progress) => {
                    if progress.tick(time.delta()).just_finished() {
                        // state.0 = true;
                        // commands.entity(entity).remove::<PlaceOrder>();
                        // progresses.remove(&entity);
                        println!("PlaceOrder complete!");
                        // Produce Order with one Lemonade, assign to OrderDesk
                        let new_order = Order {
                            items_to_produce: VecDeque::from([Item::Lemonade]),
                            items: vec![],
                            owner: entity,
                        };

                        let e_order = commands.spawn((Name::new("Order"), new_order)).id();
                        order_desk.current_order = Some(e_order);
                        customer.order = Some(e_order);

                        placed_order.0 = true;
                    } else {
                        // In progress...
                        println!("PlaceOrder Progress: {}", progress.fraction());
                    }
                }
                None => {
                    progresses.insert(entity, Timer::from_seconds(1.0, TimerMode::Once));
                }
            }
        }
    }
}

fn handle_pickup_lemonade(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &PickupLemonade,
        &mut CarryingItem,
        &mut OrderReady,
        &mut PlacedOrder,
        &mut AtOrderDesk,
    )>,
    mut progresses: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, mut state, mut order_ready, mut placed_order, mut at_order_desk) in
        query.iter_mut()
    {
        match progresses.get_mut(&entity) {
            Some(progress) => {
                if progress.tick(time.delta()).just_finished() {
                    state.0 = Item::Lemonade;

                    // Reset order status
                    order_ready.0 = false;
                    placed_order.0 = false;
                    at_order_desk.0 = false;

                    commands
                        .entity(entity)
                        .remove::<PickupLemonade>()
                        .insert(MoveTo(Vec3::new(-222.0, 0.0, 0.0)));

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

                    commands.entity(entity).remove::<DrinkLemonade>();
                    progresses.remove(&entity);
                } else {
                    // In progress...
                    thirst.0 = (thirst.0 - 0.05).max(0.0);
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
    q_actions: Query<(Entity, &dyn ActionComponent)>,
    q_datums: Query<(Entity, &dyn DatumComponent)>,
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
            Vec2::new(40.0, 40.0),
            BLUE_VIOLET,
        );
    }
}
