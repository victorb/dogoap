use crate::{mutator::Mutator, state::LocalState};
use bevy_reflect::Reflect;
use std::hash::{Hash, Hasher};

/// The effect is what happens when an Action is applied
/// It's separated from Action in order to separate the
/// data structures for the Planner's Node that is used
/// for the pathfinding part.
#[derive(Reflect, Clone, Debug, PartialEq, Eq, Default)]
pub struct Effect {
    pub action: String,
    pub mutators: Vec<Mutator>,
    pub state: LocalState,
    pub cost: usize,
}

impl Effect {
    pub fn new(action_name: &str) -> Self {
        Self {
            action: action_name.to_string(),
            mutators: vec![],
            state: LocalState::new(),
            cost: 1,
        }
    }
    pub fn with_mutator(mut self, mutator: Mutator) -> Self {
        self.mutators.push(mutator);
        self
    }
}

impl Hash for Effect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.action.hash(state);
        self.mutators.hash(state);
        self.state.hash(state);
    }
}
