#![feature(test)]

#[cfg(test)]
mod tests {
    use dogoap::{
        prelude::*,
        simple::{simple_decrement_action, simple_increment_action},
    };

    extern crate test;
    use test::Bencher;

    fn long_plan(strategy: PlanningStrategy) {
        let start = LocalState::new()
            .with_datum("energy", Datum::I64(30))
            .with_datum("hunger", Datum::I64(70))
            .with_datum("gold", Datum::I64(0));

        let expected_state = LocalState::new()
            .with_datum("energy", Datum::I64(50))
            .with_datum("hunger", Datum::I64(50))
            .with_datum("gold", Datum::I64(7));

        let goal = Goal::new().with_req("gold", Compare::Equals(Datum::I64(7)));

        let sleep_action = simple_increment_action("sleep", "energy", Datum::I64(10));

        let eat_action = simple_decrement_action("eat", "hunger", Datum::I64(10))
            .with_precondition("energy", Compare::GreaterThanEquals(Datum::I64(25)));

        let rob_people = simple_increment_action("rob", "gold", Datum::I64(1))
            .with_effect(Effect {
                action: "rob".to_string(),
                mutators: vec![
                    Mutator::Decrement("energy".to_string(), Datum::I64(5)),
                    Mutator::Increment("hunger".to_string(), Datum::I64(5)),
                ],
                state: LocalState::default(),
                cost: 1,
            })
            .with_precondition("hunger", Compare::LessThanEquals(Datum::I64(50)))
            .with_precondition("energy", Compare::GreaterThanEquals(Datum::I64(50)));

        let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

        let plan = make_plan_with_strategy(strategy, &start, &actions[..], &goal);
        let effects = get_effects_from_plan(plan.clone().unwrap().0);

        assert_eq!(11, effects.len());
        assert_eq!(expected_state, effects.last().unwrap().state);
    }

    #[bench]
    fn bench_start_to_goal_strategy(b: &mut Bencher) {
        b.iter(|| long_plan(PlanningStrategy::StartToGoal));
    }
}
