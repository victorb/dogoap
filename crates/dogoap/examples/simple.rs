use dogoap::prelude::*;

// This example is the same as examples/basic.rs but using the various `simple_*`
// functions to create the data structures instead

fn main() {
    let start = LocalState::new().with_datum("is_hungry", Datum::from_bool(true));

    let goal = Goal::new().with_req("is_hungry", Compare::Equals(Datum::from_bool(false)));

    // NOTE This is the "simple" part, where we create an action with just
    // two strings + a field
    let eat_action = simple_action("eat", "is_hungry", Datum::from_bool(false));

    let actions: Vec<Action> = vec![eat_action];

    let plan = make_plan(&start, &actions[..], &goal);

    print_plan(plan.unwrap());

    println!("");
    println!("[Everything went as expected!]");
}
