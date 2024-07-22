use crate::planner;
use bevy::prelude::*;

/// Setups the [`Planner`](planner::Planner) systems to run at
/// [`PreUpdate`](bevy::prelude::PreUpdate)
pub struct DogoapPlugin;

impl Plugin for DogoapPlugin {
    fn build(&self, app: &mut App) {
        app
            // TODO not entirely sure about using PreUpdate here
            // On one hand, we get to react to actions added in the same frame
            // On the other hand, feels a bit too much magical when actions can disappear really quickly
            .add_systems(
                PreUpdate,
                (
                    planner::update_planner_local_state,
                    planner::create_planner_tasks,
                    planner::handle_planner_tasks,
                )
                    .chain(),
            )
            .register_type::<planner::Planner>();
    }
}
