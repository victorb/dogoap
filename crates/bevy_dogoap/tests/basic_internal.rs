use bevy::prelude::*;
use bevy_dogoap::prelude::*;

// These are just handy strings so we don't fuck it up later.
const IS_HUNGRY_KEY: &str = "is_hungry";
const EAT_ACTION: &str = "eat_action";

const IS_TIRED_KEY: &str = "is_tired";
const SLEEP_ACTION: &str = "sleep_action";

// This is our component we want to be able to use
#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct EatAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
struct SleepAction;

#[derive(Component, Clone, DatumComponent)]
struct IsHungry(bool);

#[derive(Component, Clone, DatumComponent)]
struct IsTired(bool);

fn startup(mut commands: Commands) {
    // First we define our initial state
    // let state = LocalState::new()
    //     .with_field(IS_HUNGRY_KEY, Field::from(true))
    //     .with_field(IS_TIRED_KEY, Field::from(true));
    let components = vec![
        Box::new(IsHungry(true)) as Box<dyn DatumComponent>,
        Box::new(IsTired(true)) as Box<dyn DatumComponent>,
    ];

    // Then we decide a goal of not being hungry nor tired
    let goal = Goal::new()
        .with_req(IS_HUNGRY_KEY, Compare::Equals(Datum::Bool(false)))
        .with_req(IS_TIRED_KEY, Compare::Equals(Datum::Bool(false)));

    // All goals our planner could use
    let goals = vec![goal.clone()];

    // The verbose way of defining our action
    // let eat_action = Action {
    //     key: EAT_ACTION.to_string(),
    //     // We need to not be tired in order to eat
    //     preconditions: Some(vec![(
    //         IS_TIRED_KEY.to_string(),
    //         Compare::Equals(Field::from(false)),
    //     )]),
    //     options: vec![(
    //         Effect {
    //             action: EAT_ACTION.to_string(),
    //             argument: None,
    //             // The "Effect" of our EatAction is that "is_hungry" gets set to "false"
    //             mutators: vec![Mutator::Set(
    //                 IS_HUNGRY_KEY.to_string(),
    //                 Field::Bool(false),
    //             )],
    //             state: DogoapState::new(),
    //         },
    //         1,
    //     )],
    // };

    // Alternatively, the `simple` functions can help you create things a bit smoother
    let eat_action = simple_action(EAT_ACTION, IS_HUNGRY_KEY, Datum::Bool(false))
        .with_precondition(IS_TIRED_KEY, Compare::Equals(Datum::Bool(false)));

    // Here we define our SleepAction
    let sleep_action = simple_action(SLEEP_ACTION, IS_TIRED_KEY, Datum::Bool(false));

    // Verbose way of defining an actions_map that the planner needs
    // let actions_map = HashMap::from([
    //     (
    //         EAT_ACTION.to_string(),
    //         (
    //             eat_action.clone(),
    //             Box::new(EatAction {}) as Box<dyn MarkerComponent>,
    //         ),
    //     ),
    //     (
    //         SLEEP_ACTION.to_string(),
    //         (
    //             sleep_action.clone(),
    //             Box::new(SleepAction {}) as Box<dyn MarkerComponent>,
    //         ),
    //     ),
    // ]);

    // But we have a handy macro that kind of makes it a lot easier for us!
    let actions_map = create_action_map!((EatAction, eat_action), (SleepAction, sleep_action));

    let entity = commands.spawn_empty().id();

    let mut planner = Planner::new(components, goals, actions_map);

    planner.current_goal = Some(goal.clone());

    planner.insert_datum_components(&mut commands, entity);

    commands.entity(entity).insert(planner);
}

fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut IsHungry)>,
) {
    for (entity, _action, mut is_hungry) in query.iter_mut() {
        println!("We're doing EatAction!");
        is_hungry.0 = false;
        // planner
        //     .state
        //     .fields
        //     .insert(IS_HUNGRY_KEY.to_string(), Field::Bool(false));
        commands.entity(entity).remove::<EatAction>();
        println!("Removed EatAction from our Entity {}", entity);
    }
}

fn handle_sleep_action(
    mut commands: Commands,
    mut query: Query<(Entity, &SleepAction, &mut IsTired)>,
) {
    for (entity, _action, mut is_tired) in query.iter_mut() {
        println!("We're doing SleepAction!");
        // *is_tired = IsTired(false);
        is_tired.0 = false;
        // planner
        //     .state
        //     .fields
        //     .insert(IS_TIRED_KEY.to_string(), Field::Bool(false));
        commands.entity(entity).remove::<SleepAction>();
        println!("Removed SleepAction from our Entity {}", entity);
    }
}

mod test {
    use super::*;

    // Test utils
    fn get_planner(app: &mut App) -> &Planner {
        let mut query = app.world_mut().query::<&Planner>();
        let planners: Vec<&Planner> = query.iter(&app.world()).map(|v| v).collect();

        planners.first().unwrap()
    }

    fn get_state(app: &mut App) -> LocalState {
        let planner = get_planner(app);
        // planner.field_components_to_localstate()
        planner.state.clone()
    }

    fn assert_key_is_bool(app: &mut App, key: &str, expected_bool: bool) {
        let state = get_state(app);
        let expected_val = Datum::Bool(expected_bool);
        let found_val = state.data.get(key).unwrap();
        assert_eq!(*found_val, expected_val);
    }

    #[allow(dead_code)]
    fn assert_component_exists<T>(app: &mut App)
    where
        T: bevy::prelude::Component,
    {
        let mut query = app.world_mut().query::<&T>();
        let c = query.iter(&app.world()).len();
        assert_eq!(c > 0, true);
    }

    fn assert_component_not_exists<T>(app: &mut App)
    where
        T: bevy::prelude::Component,
    {
        let mut query = app.world_mut().query::<&T>();
        let c = query.iter(&app.world()).len();
        assert_eq!(c == 0, true);
    }

    #[test]
    fn test_basic_bevy_integration_internal() {
        let mut app = App::new();

        // TODO get rid of this somehow?
        app.register_component_as::<dyn DatumComponent, IsHungry>();
        app.register_component_as::<dyn DatumComponent, IsTired>();

        app.add_plugins(DogoapPlugin);
        app.add_plugins(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions::with_num_threads(1),
        });
        app.init_resource::<Time>();

        app.add_systems(Startup, startup);
        app.add_systems(Update, (handle_eat_action, handle_sleep_action));

        // TODO this could be flaky because of the AsyncTaskPool, should force planning
        // to be sync in testing. Ideally should be 3 updates, but setting it to 4 to be safe(r)
        for _i in 0..4 {
            app.update();
        }

        assert_key_is_bool(&mut app, IS_HUNGRY_KEY, false);
        assert_key_is_bool(&mut app, IS_TIRED_KEY, false);
        assert_component_not_exists::<EatAction>(&mut app);
        assert_component_not_exists::<SleepAction>(&mut app);

        println!("Final State:\n{:#?}", get_state(&mut app));
    }
}
