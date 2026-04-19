//! This example is the same as examples/basic.rs but using the various `simple_*`
//! functions to create the data structures instead

use dogoap::prelude::*;

fn main() {
    let start = LocalState::new()
        .with_datum("walk_distance", 10i64)
        .with_datum("target_distance", 5i64);

    let goal = Goal::new().with_req("walk_distance", Compare::equals(0i64));
    
    let walk_action = Action::new("walk").with_mutator(Mutator::reference("walk_distance", "target_distance", ReferenceMutator::Decrement));

    let actions: Vec<Action> = vec![walk_action];

    let plan = make_plan(&start, &actions[..], &goal);

    println!("{plan:#?}");

    println!("{}", format_plan(plan.unwrap()));

    println!();
    println!("[Everything went as expected!]");
}
