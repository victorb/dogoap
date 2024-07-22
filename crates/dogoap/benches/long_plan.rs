#![feature(test)]

#[cfg(test)]
mod tests {
    use dogoap::prelude::*;

    extern crate test;
    use test::Bencher;

    fn long_plan(strategy: PlanningStrategy) {
        let start = LocalState::new()
            .with_field("energy", Field::from_i64(30))
            .with_field("hunger", Field::from_i64(70))
            .with_field("gold", Field::from_i64(0));

        let expected_state = LocalState::new()
            .with_field("energy", Field::from_i64(50))
            .with_field("hunger", Field::from_i64(50))
            .with_field("gold", Field::from_i64(7));

        let goal = Goal::new().with_req("gold", Compare::Equals(Field::from_i64(7)));

        // TOOD should keep the `10 as 64` syntax with .from somehow
        let sleep_action = simple_increment_action("sleep", "energy", Field::from_i64(10));

        let eat_action = simple_decrement_action("eat", "hunger", Field::from_i64(10))
            .with_precondition("energy", Compare::GreaterThanEquals(Field::from_i64(25)));

        let rob_people = simple_increment_action("rob", "gold", Field::from_i64(1))
            .with_effect(
                Effect {
                    action: "rob".to_string(),
                    argument: None,
                    mutators: vec![
                        Mutator::Decrement("energy".to_string(), Field::from_i64(5)),
                        Mutator::Increment("hunger".to_string(), Field::from_i64(5)),
                    ],
                    state: LocalState::default(),
                },
                1,
            )
            .with_precondition("hunger", Compare::LessThanEquals(Field::from_i64(50)))
            .with_precondition("energy", Compare::GreaterThanEquals(Field::from_i64(50)));

        let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

        let plan = get_effects_from_plan(make_plan_with_strategy(strategy, &start, &actions[..], &goal).unwrap().0);
        assert_eq!(11, plan.len());

        // for cons in &plan {
        //     assert_eq!("eat", cons.action);
        //     assert_eq!(None, cons.argument);
        //     assert_eq!(1, cons.mutators.len());
        // }

        assert_eq!(expected_state, plan.last().unwrap().state);
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
