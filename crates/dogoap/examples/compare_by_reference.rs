//! This example is the same as examples/basic.rs but using the various `simple_*`
//! functions to create the data structures instead

use dogoap::prelude::*;

fn main() {
    let start = LocalState::new()
        .with_datum("walk_distance", 0i64)
        .with_datum("target_distance", 10i64);

    let goal = Goal::new().with_req("walk_distance", Compare::reference("target_distance", ReferenceCompare::Equals));

    let gather_strength_action = Action::new("gather_strength").with_mutator(Mutator::increment("walk_distance", 5i64));

    let actions: Vec<Action> = vec![gather_strength_action];

    let plan = make_plan(&start, &actions[..], &goal);

    println!("{plan:#?}");

    println!("{}", format_plan(plan.unwrap()));

    println!();
    println!("[Everything went as expected!]");
}
