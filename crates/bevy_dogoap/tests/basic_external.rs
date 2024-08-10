use bevy::prelude::*;
use bevy_dogoap::prelude::*;

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
    // Then we decide a goal of not being hungry nor tired
    let goal = Goal::from_reqs(&[IsHungry::is(false), IsTired::is(false)]);

    // Alternatively, the `simple` functions can help you create things a bit smoother
    let eat_action = EatAction::new()
        .add_precondition(IsTired::is(false))
        .add_mutator(IsHungry::set(false));

    // Here we define our SleepAction
    let sleep_action = SleepAction::new().add_mutator(IsTired::set(false));

    // But we have a handy macro that kind of makes it a lot easier for us!
    // let actions_map = create_action_map!((EatAction, eat_action), (SleepAction, sleep_action));

    let entity = commands.spawn_empty().id();

    let (mut planner, components) = create_planner!({
        actions: [
            (EatAction, eat_action),
            (SleepAction, sleep_action),
        ],
        state: [IsHungry(true), IsTired(true)],
        goals: [goal.clone()],
    });

    planner.remove_goal_on_no_plan_found = false;
    planner.always_plan = true;
    planner.current_goal = Some(goal.clone());

    commands.entity(entity).insert((planner, components));
}

fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<(Entity, &EatAction, &mut IsHungry)>,
) {
    for (entity, _action, mut is_hungry) in query.iter_mut() {
        println!("We're doing EatAction!");
        is_hungry.0 = false;
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
        is_tired.0 = false;
        commands.entity(entity).remove::<SleepAction>();
        println!("Removed SleepAction from our Entity {}", entity);
    }
}

mod test {
    use super::*;

    // Test utils
    fn get_state(app: &mut App) -> LocalState {
        let mut query = app.world_mut().query::<&Planner>();
        let planners: Vec<&Planner> = query.iter(&app.world()).map(|v| v).collect();

        let planner = planners.first().unwrap();

        planner.state.clone()
    }

    fn assert_key_is_bool(app: &mut App, key: &str, expected_bool: bool, msg: &str) {
        let state = get_state(app);
        let expected_val = Datum::Bool(expected_bool);
        let found_val = state.data.get(key).unwrap();
        assert_eq!(*found_val, expected_val, "{}", msg);
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
    fn test_basic_bevy_integration_external() {
        let mut app = App::new();

        register_components!(app, vec![IsHungry, IsTired]);

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

        println!("Final State:\n{:#?}", get_state(&mut app));

        assert_key_is_bool(&mut app, "is_hungry", false, "is_hungry wasn't false");
        assert_key_is_bool(&mut app, "is_tired", false, "is_tired wasn't false");
        assert_component_not_exists::<EatAction>(&mut app);
        assert_component_not_exists::<SleepAction>(&mut app);

        println!("Final State:\n{:#?}", get_state(&mut app));
    }
}
