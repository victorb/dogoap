use dogoap::prelude::*;

fn main() {
    let start = LocalState::new()
        .with_datum("energy", Datum::I64(30))
        .with_datum("hunger", Datum::I64(70))
        .with_datum("gold", Datum::I64(0));

    let expected_state = LocalState::new()
        .with_datum("energy", Datum::I64(45))
        .with_datum("hunger", Datum::I64(55))
        .with_datum("gold", Datum::I64(7));

    let goal = Goal::new().with_req("gold", Compare::Equals(Datum::I64(7)));

    let sleep_action = simple_increment_action("sleep", "energy", Datum::I64(10));

    let eat_action = simple_decrement_action("eat", "hunger", Datum::I64(10))
        .with_precondition("energy", Compare::GreaterThanEquals(Datum::I64(25)));

    let rob_people = Action::new("rob")
        .with_effect(
            Effect {
                action: "rob".to_string(),
                mutators: vec![
                    Mutator::Decrement("energy".to_string(), Datum::I64(5)),
                    Mutator::Increment("gold".to_string(), Datum::I64(1)),
                    Mutator::Increment("hunger".to_string(), Datum::I64(5)),
                ],
                state: LocalState::default(),
            },
            1,
        )
        .with_precondition("hunger", Compare::LessThanEquals(Datum::I64(50)))
        .with_precondition("energy", Compare::GreaterThanEquals(Datum::I64(50)));

    let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

    let plan = make_plan(&start, &actions[..], &goal);
    let effects = get_effects_from_plan(plan.clone().unwrap().0);
    assert_eq!(17, effects.len());

    println!("{:#?}", effects);

    print_plan(plan.unwrap());

    assert_eq!(expected_state, effects.last().unwrap().state);
}
