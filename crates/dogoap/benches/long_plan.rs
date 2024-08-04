#![feature(test)]

#[cfg(test)]
mod tests {
    use dogoap::prelude::*;

    extern crate test;
    use test::Bencher;

    fn long_plan(strategy: PlanningStrategy) {
        let start = LocalState::new()
            .with_field("energy", Field::I64(30))
            .with_field("hunger", Field::I64(70))
            .with_field("gold", Field::I64(0));

        let expected_state = LocalState::new()
            .with_field("energy", Field::I64(50))
            .with_field("hunger", Field::I64(50))
            .with_field("gold", Field::I64(7));

        let goal = Goal::new().with_req("gold", Compare::Equals(Field::I64(7)));

        // TOOD should keep the `10 as 64` syntax with .from somehow
        let sleep_action = simple_increment_action("sleep", "energy", Field::I64(10));

        let eat_action = simple_decrement_action("eat", "hunger", Field::I64(10))
            .with_precondition("energy", Compare::GreaterThanEquals(Field::I64(25)));

        let rob_people = simple_increment_action("rob", "gold", Field::I64(1))
            .with_effect(
                Effect {
                    action: "rob".to_string(),
                    mutators: vec![
                        Mutator::Decrement("energy".to_string(), Field::I64(5)),
                        Mutator::Increment("hunger".to_string(), Field::I64(5)),
                    ],
                    state: LocalState::default(),
                },
                1,
            )
            .with_precondition("hunger", Compare::LessThanEquals(Field::I64(50)))
            .with_precondition("energy", Compare::GreaterThanEquals(Field::I64(50)));

        let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

        let plan = make_plan_with_strategy(strategy, &start, &actions[..], &goal);
        let effects = get_effects_from_plan(plan.clone().unwrap().0);
        assert_eq!(11, effects.len());

        print_plan(plan.unwrap());

        assert_eq!(expected_state, effects.last().unwrap().state);
    }

    #[bench]
    fn bench_start_to_goal_strategy(b: &mut Bencher) {
        b.iter(|| long_plan(PlanningStrategy::StartToGoal));
    }

    // #[bench]
    // fn bench_goal_to_start_strategy(b: &mut Bencher) {
    //     b.iter(|| long_plan(PlanningStrategy::GoalToStart));
    // }
}
