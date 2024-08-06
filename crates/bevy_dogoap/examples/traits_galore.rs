// bevy_dogoap/examples/traits_galore.rs

use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use dogoap::prelude::*;

#[derive(Clone, Copy, EnumDatum)]
enum Location {
    Home,
    Outside,
    Mushroom,
}

#[derive(Component, Clone, Copy, EnumComponent)]
struct AtLocation(Location);

#[derive(Component, Clone, DatumComponent)]
struct Energy(f64);

#[derive(Component, Clone, DatumComponent)]
struct Hunger(f64);

#[derive(Component, Clone, DatumComponent)]
struct IsAngry(bool);

#[derive(ActionComponent)]
struct EatAction;

#[derive(ActionComponent)]
struct SleepAction;

#[derive(ActionComponent)]
struct GoHomeAction;

fn main() {
    let goal = Goal::from_reqs(&[Energy::is_more(10.0)]);

    let eat_action = EatAction::new()
        .add_precondition(Energy::is_more(10.0))
        .add_mutator(Hunger::increase(25.0));

    let sleep_action = SleepAction::new() //
        .add_mutator(Energy::increase(25.0));

    let go_home_action = GoHomeAction::new() //
        .add_precondition(AtLocation::is_not(Location::Home));

    let actions: Vec<Action> = vec![eat_action, sleep_action, go_home_action];

    println!("{:#?}", actions);
}
