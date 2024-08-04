use std::collections::HashMap;

use dogoap::prelude::*;

// One action that sets one field
#[test]
fn test_basic_bool_setting() {
    let start = LocalState::new().with_field("is_hungry", Field::from_bool(true));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Field::from_bool(false)));

    let eat_mutator = Mutator::Set("is_hungry".to_string(), Field::from_bool(false));

    let eat_consequence = Effect {
        action: "eat".to_string(),
        mutators: vec![eat_mutator.clone()],
        state: LocalState::new(),
    };

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: None,
        options: vec![(eat_consequence, 1)],
    };

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(1, plan.len());

    let cons = plan.get(0).unwrap();
    assert_eq!("eat", cons.action);
    assert_eq!(1, cons.mutators.len());
    assert_eq!(eat_mutator, cons.mutators.get(0).unwrap().clone());

    let expected_state = LocalState::new().with_field("is_hungry", Field::from_bool(false));
    assert_eq!(expected_state, cons.state);
}

// Anything that implements Reflect could be a key
#[test]
fn test_action_keys() {
    let start = LocalState::new().with_field("is_hungry", Field::from_bool(true));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Field::from_bool(false)));

    let eat_mutator = Mutator::Set("is_hungry".to_string(), Field::from_bool(false));

    let eat_consequence = Effect {
        action: "eat".to_string(),
        mutators: vec![eat_mutator.clone()],
        state: LocalState::new(),
    };

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: None,
        options: vec![(eat_consequence, 1)],
    };

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(1, plan.len());

    let cons = plan.get(0).unwrap();
    assert_eq!("eat", cons.action);
    assert_eq!(1, cons.mutators.len());
    assert_eq!(eat_mutator, cons.mutators.get(0).unwrap().clone());

    let expected_state = LocalState::new().with_field("is_hungry", Field::from_bool(false));
    assert_eq!(expected_state, cons.state);
}

// // The state is already what we need!
#[test]
fn test_no_actions_needed() {
    let start = LocalState::new().with_field("is_hungry", Field::from_bool(false));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Field::from_bool(false)));

    let eat_mutator = Mutator::Set("is_hungry".to_string(), Field::from_bool(false));

    let eat_consequence = Effect {
        action: "eat".to_string(),
        mutators: vec![eat_mutator.clone()],
        state: LocalState::new(),
    };

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: None,
        options: vec![(eat_consequence, 1)],
    };

    let actions: Vec<Action> = vec![eat_action];

    let (plan, plan_cost) = make_plan(&start, &actions[..], &goal).unwrap();
    assert_eq!(1, plan.len());
    assert_eq!(0, plan_cost);

    let expected_state = LocalState::new().with_field("is_hungry", Field::from_bool(false));
    assert_eq!(expected_state, plan.first().unwrap().state().clone());
}
//
// Shorthand for one action that sets one field
#[test]
fn test_simple_action() {
    let start = LocalState::new().with_field("is_hungry", Field::from_bool(true));
    let expected_state = LocalState::new().with_field("is_hungry", Field::from_bool(false));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Field::from_bool(false)));

    let eat_action = simple_action("eat", "is_hungry", Field::from_bool(false));
    let eat_mutator = Mutator::Set("is_hungry".to_string(), Field::from_bool(false));

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(1, plan.len());

    let cons = plan.get(0).unwrap();
    assert_eq!("eat", cons.action);
    assert_eq!(1, cons.mutators.len());
    assert_eq!(eat_mutator, cons.mutators.get(0).unwrap().clone());
    assert_eq!(expected_state, cons.state);
}

// //
// // // State with two fields + two actions each mutating their fields
#[test]
fn test_two_bools() {
    let start = LocalState::new()
        .with_field("is_hungry", Field::from_bool(true))
        .with_field("is_tired", Field::from_bool(true));

    let expected_state = LocalState::new()
        .with_field("is_hungry", Field::from_bool(false))
        .with_field("is_tired", Field::from_bool(false));

    let goal = Goal::new()
        .with_req("is_hungry", Compare::Equals(Field::from_bool(false)))
        .with_req("is_tired", Compare::Equals(Field::from_bool(false)));

    let eat_action = simple_action("eat", "is_hungry", Field::from_bool(false));
    let sleep_action = simple_action("sleep", "is_tired", Field::from_bool(false));

    let actions: Vec<Action> = vec![eat_action, sleep_action];

    let plan = make_plan(&start, &actions[..], &goal).unwrap();

    let cons = get_effects_from_plan(plan.0);
    assert_eq!(2, cons.len());

    let first_cons = cons.get(0).unwrap();
    assert_eq!("eat", first_cons.action);
    assert_eq!(1, first_cons.mutators.len());

    let second_cons = cons.get(1).unwrap();
    assert_eq!("sleep", second_cons.action);
    assert_eq!(1, second_cons.mutators.len());

    assert_eq!(expected_state, second_cons.state);
}

// State with two fields + two actions each mutating their fields
#[test]
fn test_four_bools() {
    let start = LocalState::new()
        .with_field("is_hungry", Field::from_bool(true))
        .with_field("is_tired", Field::from_bool(true))
        .with_field("is_fit", Field::from_bool(false))
        .with_field("is_dirty", Field::from_bool(false));

    // We want to be fit, but not hungry, tired or dirty
    let goal = Goal::new()
        .with_req("is_hungry", Compare::Equals(Field::from_bool(false)))
        .with_req("is_tired", Compare::Equals(Field::from_bool(false)))
        .with_req("is_fit", Compare::Equals(Field::from_bool(true)))
        .with_req("is_dirty", Compare::Equals(Field::from_bool(false)));

    // Actions
    // eat => not hungry
    // sleep => not tired but now hungry
    // train => fit but now dirty and tired
    // shower => not dirty but now tired
    let eat_action = simple_action("eat", "is_hungry", Field::from_bool(false));

    let sleep_action = simple_multi_mutate_action(
        "sleep",
        vec![
            ("is_tired", Field::from_bool(false)),
            ("is_hungry", Field::from_bool(true)),
        ],
    );
    let train_action = simple_multi_mutate_action(
        "train",
        vec![
            ("is_tired", Field::from_bool(true)),
            ("is_dirty", Field::from_bool(true)),
            ("is_fit", Field::from_bool(true)),
        ],
    );
    let shower_action = simple_multi_mutate_action(
        "shower",
        vec![
            ("is_tired", Field::from_bool(true)),
            ("is_dirty", Field::from_bool(false)),
        ],
    );

    let actions: Vec<Action> = vec![eat_action, sleep_action, train_action, shower_action];

    let plan = make_plan(&start, &actions[..], &goal).unwrap();

    let cons = get_effects_from_plan(plan.0);
    assert_eq!(4, cons.len());

    let first_cons = cons.get(0).unwrap();
    assert_eq!("train", first_cons.action);
    assert_eq!(3, first_cons.mutators.len());

    let second_cons = cons.get(1).unwrap();
    assert_eq!("shower", second_cons.action);
    assert_eq!(2, second_cons.mutators.len());

    let third_cons = cons.get(2).unwrap();
    assert_eq!("sleep", third_cons.action);
    assert_eq!(2, third_cons.mutators.len());

    let fourth_cons = cons.get(3).unwrap();
    assert_eq!("eat", fourth_cons.action);
    assert_eq!(1, fourth_cons.mutators.len());

    let expected_state = LocalState::new()
        .with_field("is_hungry", Field::from_bool(false))
        .with_field("is_tired", Field::from_bool(false))
        .with_field("is_fit", Field::from_bool(true))
        .with_field("is_dirty", Field::from_bool(false));
    assert_eq!(expected_state, cons.last().unwrap().state);
}

enum TestLocation {
    House,
    Outside,
    Market,
    RamenShop,
}

#[test]
fn test_enums() {
    let loc_house = Field::from_enum(TestLocation::House as usize);
    let loc_outside = Field::from_enum(TestLocation::Outside as usize);
    let loc_market = Field::from_enum(TestLocation::Market as usize);
    let loc_ramen = Field::from_enum(TestLocation::RamenShop as usize);

    let start = LocalState::new().with_field("at_location", loc_house.clone());

    let expected_state = LocalState::new().with_field("at_location", loc_ramen.clone());

    let goal = Goal::new().with_req("at_location", Compare::Equals(loc_ramen.clone()));

    let go_outside_action = simple_action("go_outside", "at_location", loc_outside.clone())
        .with_precondition("at_location", Compare::Equals(loc_house.clone()));

    let go_to_market_action = simple_action("go_to_market", "at_location", loc_market.clone())
        .with_precondition("at_location", Compare::Equals(loc_outside.clone()));

    let go_to_ramen_action = simple_action("go_to_ramen", "at_location", loc_ramen.clone())
        .with_precondition("at_location", Compare::Equals(loc_market.clone()));

    let actions: Vec<Action> = vec![go_outside_action, go_to_market_action, go_to_ramen_action];

    let plan = make_plan(&start, &actions[..], &goal);
    let effects = get_effects_from_plan(plan.unwrap().0);

    assert_eq!(3, effects.len());

    let cons = effects.get(0).unwrap();
    assert_eq!("go_outside", cons.action);
    assert_eq!(1, cons.mutators.len());

    let cons = effects.get(1).unwrap();
    assert_eq!("go_to_market", cons.action);
    assert_eq!(1, cons.mutators.len());

    let cons = effects.get(2).unwrap();
    assert_eq!("go_to_ramen", cons.action);
    assert_eq!(1, cons.mutators.len());

    // Take only the last one
    assert_eq!(expected_state, cons.state);
}

// // eat action can only be done with not tired
#[test]
fn test_preconditions() {
    let start = LocalState::new()
        .with_field("is_hungry", Field::from_bool(true))
        .with_field("is_tired", Field::from_bool(true));

    let expected_state = LocalState::new()
        .with_field("is_hungry", Field::from_bool(false))
        .with_field("is_tired", Field::from_bool(false));

    let goal = Goal::new()
        .with_req("is_hungry", Compare::Equals(Field::from_bool(false)))
        .with_req("is_tired", Compare::Equals(Field::from_bool(false)));

    let mut eat_action = simple_multi_mutate_action(
        "eat",
        vec![
            ("is_hungry", Field::from_bool(false)),
            ("is_tired", Field::from_bool(true)),
        ],
    );
    add_preconditions(
        &mut eat_action,
        vec![("is_tired", Compare::Equals(Field::from_bool(false)))],
    );

    let sleep_action = simple_action("sleep", "is_tired", Field::from_bool(false));

    let actions: Vec<Action> = vec![eat_action, sleep_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(3, plan.len());

    let first_cons = plan.get(0).unwrap();
    assert_eq!("sleep", first_cons.action);
    assert_eq!(1, first_cons.mutators.len());

    let second_cons = plan.get(1).unwrap();
    assert_eq!("eat", second_cons.action);
    assert_eq!(2, second_cons.mutators.len());

    let third_cons = plan.get(2).unwrap();
    assert_eq!("sleep", third_cons.action);
    assert_eq!(1, third_cons.mutators.len());

    assert_eq!(
        expected_state, third_cons.state,
        "Final state wasn't what we expected"
    );
}

// We can use ints too!
#[test]
fn test_int_increment() {
    let start = LocalState::new().with_field("energy", Field::from_i64(50));
    let expected_state = LocalState::new().with_field("energy", Field::from_i64(100));

    let goal = Goal::new().with_req("energy", Compare::Equals(Field::from_i64(100)));

    // TOOD should keep the `10 as 64` syntax with .from somehow
    let eat_action = simple_increment_action("eat", "energy", Field::from_i64(10));
    let eat_mutator = Mutator::Increment("energy".to_string(), Field::from_i64(10 as i64));

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(5, plan.len());

    for cons in &plan {
        assert_eq!("eat", cons.action);
        assert_eq!(1, cons.mutators.len());
        assert_eq!(eat_mutator, cons.mutators.get(0).unwrap().clone());
    }

    assert_eq!(expected_state, plan.last().unwrap().state);
}

#[test]
fn test_int_decrement() {
    let start = LocalState::new().with_field("hunger", Field::from_i64(80 as i64));
    let expected_state = LocalState::new().with_field("hunger", Field::from_i64(10 as i64));

    let goal = Goal::new().with_req("hunger", Compare::Equals(Field::from_i64(10 as i64)));

    let eat_action = simple_decrement_action("eat", "hunger", Field::from_i64(10 as i64));
    let eat_mutator = Mutator::Decrement("hunger".to_string(), Field::from_i64(10 as i64));

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(7, plan.len());

    for cons in &plan {
        assert_eq!("eat", cons.action);
        assert_eq!(1, cons.mutators.len());
        assert_eq!(eat_mutator, cons.mutators.get(0).unwrap().clone());
    }

    assert_eq!(expected_state, plan.last().unwrap().state);
}

#[test]
fn test_float_increment() {
    let start = LocalState::new().with_field("energy", Field::from(50.0));
    let expected_state = LocalState::new().with_field("energy", Field::from(100.0));

    let goal = Goal::new().with_req("energy", Compare::Equals(Field::from(100.0)));

    let eat_action = simple_increment_action("eat", "energy", Field::from(10.0));
    let eat_mutator = Mutator::Increment("energy".to_string(), Field::from(10.0));

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(5, plan.len());

    for cons in &plan {
        assert_eq!("eat", cons.action);
        assert_eq!(1, cons.mutators.len());
        assert_eq!(eat_mutator, cons.mutators.get(0).unwrap().clone());
    }

    assert_eq!(expected_state, plan.last().unwrap().state);
}

// GreaterThanEquals can be useful sometimes too!
#[test]
fn test_greater_than_equals() {
    let start = LocalState::new().with_field("energy", Field::from(0));
    let expected_state = LocalState::new().with_field("energy", Field::from(54));

    let goal = Goal::new().with_req("energy", Compare::GreaterThanEquals(Field::from(50)));

    let eat_action = simple_increment_action("eat", "energy", Field::from(6));

    let actions: Vec<Action> = vec![eat_action];

    let plan = make_plan(&start, &actions[..], &goal).unwrap();
    let effects = get_effects_from_plan(plan.0.clone());

    println!("Found plan:");
    println!("{:#?}", plan);

    assert_eq!(9, effects.len());

    for cons in &effects {
        assert_eq!("eat", cons.action);
        assert_eq!(1, cons.mutators.len());
        assert_eq!(
            Mutator::Increment("energy".to_string(), Field::from(6)),
            cons.mutators.get(0).unwrap().clone()
        );
    }

    assert_eq!(expected_state, effects.last().unwrap().state);
}

#[test]
fn test_long_plan() {
    let start = LocalState::new()
        .with_field("energy", Field::from_i64(30))
        .with_field("hunger", Field::from_i64(70))
        .with_field("gold", Field::from_i64(0));

    let expected_state = LocalState::new()
        .with_field("energy", Field::from_i64(50))
        .with_field("hunger", Field::from_i64(50))
        .with_field("gold", Field::from_i64(10));

    let goal = Goal::new().with_req("gold", Compare::Equals(Field::from_i64(10)));

    // TOOD should keep the `10 as 64` syntax with .from somehow
    let sleep_action = simple_increment_action("sleep", "energy", Field::from_i64(1));

    let eat_action = simple_decrement_action("eat", "hunger", Field::from_i64(1))
        .with_precondition("energy", Compare::GreaterThanEquals(Field::from_i64(50)));

    let rob_people = simple_increment_action("rob", "gold", Field::from_i64(1))
        .with_effect(
            Effect {
                action: "rob".to_string(),
                mutators: vec![
                    Mutator::Decrement("energy".to_string(), Field::from_i64(20)),
                    Mutator::Increment("hunger".to_string(), Field::from_i64(20)),
                ],
                state: LocalState::default(),
            },
            1,
        )
        .with_precondition("hunger", Compare::LessThanEquals(Field::from_i64(50)))
        .with_precondition("energy", Compare::GreaterThanEquals(Field::from_i64(50)));

    let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0);
    assert_eq!(50, plan.len());

    // for cons in &plan {
    //     assert_eq!("eat", cons.action);
    //     assert_eq!(None, cons.argument);
    //     assert_eq!(1, cons.mutators.len());
    // }

    assert_eq!(expected_state, plan.last().unwrap().state);
}

// TODO haven't implemented `PlanningStrategy::GoalToStart` yet
#[test]
#[should_panic]
fn test_reverse_strategy() {
    let start = LocalState::new().with_field("is_hungry", Field::from_bool(true));
    let expected_state = LocalState::new().with_field("is_hungry", Field::from_bool(false));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Field::from_bool(false)));

    let eat_action = simple_action("eat", "is_hungry", Field::from_bool(false));
    let eat_mutator = Mutator::Set("is_hungry".to_string(), Field::from_bool(false));

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(
        make_plan_with_strategy(PlanningStrategy::GoalToStart, &start, &actions[..], &goal)
            .unwrap()
            .0,
    );
    assert_eq!(1, plan.len());

    let cons = plan.get(0).unwrap();
    assert_eq!("eat", cons.action);
    assert_eq!(1, cons.mutators.len());
    assert_eq!(eat_mutator, cons.mutators.get(0).unwrap().clone());
    assert_eq!(expected_state, cons.state);
}