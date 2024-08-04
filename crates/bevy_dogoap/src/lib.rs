#![doc = include_str!("../README.md")]
use std::any::Any;
use std::fmt;

use bevy::prelude::*;

// Public API
pub use dogoap::prelude::*;

mod planner;
pub mod prelude;

#[reflect_trait]
pub trait InserterComponent: Send + Sync {
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
    fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity);
    fn clone_box(&self) -> Box<dyn InserterComponent>;
    fn as_any(&self) -> &dyn Any;
}

impl<T> InserterComponent for T
where
    T: Component + Clone + Send + Sync,
{
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity) {
        commands.entity(entity_to_insert_to).insert(T::clone(self));
    }
    fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity) {
        commands.entity(entity_to_remove_from).remove::<T>();
    }
    fn clone_box(&self) -> Box<dyn InserterComponent> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Debug for dyn InserterComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MarkerComponent [DebugNotImplemented!]",)
    }
}

#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait DatumComponent: Send + Sync {
    fn field_key(&self) -> String;
    fn field_value(&self) -> Datum;
    fn set_value(&mut self, new_val: Datum);
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
}

pub trait ActionComponent: Send + Sync {
    fn key() -> String;
}

#[macro_export]
macro_rules! create_action_map {
    ($(($key:expr, $action:expr, $marker:ty)),* $(,)?) => {{
        use std::collections::HashMap;
        use bevy_dogoap::InserterComponent;
        let map: HashMap<String, (Action, Box<dyn InserterComponent>)> = HashMap::from([
            $(
                (
                    $key.to_string(),
                    (
                        $action.clone(),
                        Box::new(<$marker>::default()) as Box<dyn InserterComponent>,
                    ),
                )
            ),*
        ]);
        map
    }};
}

#[macro_export]
macro_rules! create_action_map_v2 {
    ($(($marker:ty, $action:expr)),* $(,)?) => {{
        use std::collections::HashMap;
        use bevy_dogoap::InserterComponent;
        let map: HashMap<String, (Action, Box<dyn InserterComponent>)> = HashMap::from([
            $(
                (
                    <$marker>::key(),
                    (
                        $action.clone(),
                        Box::new(<$marker>::default()) as Box<dyn InserterComponent>,
                    ),
                )
            ),*
        ]);
        map
    }};
}

#[macro_export]
macro_rules! create_state {
    ($( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Box::new($x) as Box<dyn DatumComponent>);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! register_components {
    ($app:ident, vec![$($comp:ty),*]) => {
        $(
            $app.register_component_as::<dyn DatumComponent, $comp>();
        )*
    };
}

#[macro_export]
macro_rules! create_goal {
    ($(($type:ident, $comp:path, $field:expr)),*) => {{
        let mut goal = Goal::new();

        $(
            goal = goal.with_req(&$type::key(), $comp($field));
        )*

        goal
    }};
}

pub fn goal_from_datumcomponents(components: Vec<(Box<dyn DatumComponent>, Compare)>) -> Goal {
    let mut new_goal = Goal::new();
    for (component, compare) in components {
        new_goal = new_goal.with_req(&component.field_key(), compare)
    }
    new_goal
}

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
                    .chain(),
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
