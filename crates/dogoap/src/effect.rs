use crate::{field::Field, mutator::Mutator, state::LocalState};
use bevy_reflect::Reflect;
use std::hash::{Hash, Hasher};

#[derive(Reflect, Clone, Debug, PartialEq, Eq, Default)]
pub struct Effect {
    pub action: String,
    pub argument: Option<Field>,
    pub mutators: Vec<Mutator>,
    pub state: LocalState, // TODO do we really need LocalState here?
}

impl Effect {
    pub fn new(action_name: &str) -> Self {
        Self {
            action: action_name.to_string(),
            argument: None,
            mutators: vec![],
            state: LocalState::new(),
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
        self.argument.hash(state);
        self.mutators.hash(state);
        self.state.hash(state);
    }
}
