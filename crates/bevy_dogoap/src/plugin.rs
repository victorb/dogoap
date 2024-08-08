use std::time::Duration;

use crate::planner;
use bevy::{prelude::*, time::common_conditions::on_timer};

pub struct DogoapPlugin;

impl Plugin for DogoapPlugin {
    fn build(&self, app: &mut App) {
        app
            // TODO not entirely sure about using PreUpdate here
            // On one hand, we get to react to actions added in the same frame
            // On the other hand, feels a bit too much magical when actions can disappear really quickly
            // .add_systems(PreUpdate, (planner::update_planner_local_state, planner::update_planner).chain())
            // .add_systems(PreUpdate, (planner::update_planner_local_state, planner::update_planner))
            // .add_systems(Update, (planner::update_planner_local_state, planner::update_planner))
            // .add_systems(PreUpdate, planner::update_planner_local_state)
            .add_systems(
                PreUpdate,
                (
                    planner::update_planner_local_state,
                    planner::create_planner_tasks,
                    planner::handle_planner_tasks,
                )
                    .chain(), // .run_if(on_timer(Duration::from_millis(100))),
            )
            // .add_systems(PlanningSchedule,
            //     (planner::update_planner_local_state,
            //      planner::update_planner))
            // .add_systems(Update, .in_set(MyGoapSet))
            .register_type::<planner::Planner>();

        // TODO how to be able to call this with generate types passed in to the creation of DogoapPlugin?
        //     app.register_component_as::<dyn DatumComponent, IsTired>();
        // Right now users have to manually call `register_components!(app, vec![IsHungry, IsTired]);`
        // somewhere

        // app.world_mut().resource_mut::<MainScheduleOrder>()
        //     .insert_after(PreUpdate, DogoapSchedule);
    }
}
