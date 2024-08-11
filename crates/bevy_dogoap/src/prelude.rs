// TODO change to upstream once available
pub use bevy_trait_query_0_14_0::RegisterExt;

pub use dogoap::prelude::{Action, Compare, Datum, Goal, LocalState, Mutator};

pub use crate::{
    create_action_map, create_goal, create_planner, create_state, planner::IsPlanning,
    planner::Planner, register_actions, register_components,
};

pub use crate::plugin::DogoapPlugin;

pub use crate::traits::{
    ActionComponent, DatumComponent, EnumDatum, InserterComponent, MutatorTrait, Precondition,
};

pub use dogoap_macros::{ActionComponent, DatumComponent, EnumComponent, EnumDatum};
