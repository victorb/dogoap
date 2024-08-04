use dogoap::prelude::*;

// This example shows the most basic use of dogoap
// It's a bit overly verbose (check examples/simple.rs for a "not as verbose" example)
// but shows the data structures needed for the planner

fn main() {
    let start = LocalState::new().with_datum("is_hungry", Datum::from_bool(true));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Datum::from_bool(false)));

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: None,
        options: vec![(
            Effect {
                action: "eat".to_string(),
                mutators: vec![Mutator::Set(
                    "is_hungry".to_string(),
                    Datum::from_bool(false),
                )],
                state: LocalState::new(),
            },
            1,
        )],
    };

    let actions: Vec<Action> = vec![eat_action];

    let plan = make_plan(&start, &actions[..], &goal);

    print_plan(plan.unwrap());

    println!("");
    println!("[Everything went as expected!]");
}
