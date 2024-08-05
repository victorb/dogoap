// crate "dogoap" file action.rs
use std::hash::{Hash, Hasher};

use bevy_reflect::*;

use crate::compare::Compare;
use crate::effect::Effect;

/// An `Action` represents something your NPC can do, granted the LocalState
/// is as defined in the `preconditions`. It has a list of `Effect`s that apply
/// if the NPC successfully executed the task.
#[derive(Reflect, Clone, Debug, PartialEq, Default)]
pub struct Action {
    /// String like `eat_action`
    pub key: String,
    /// What preconditions need to be true before we can execute this action
    pub preconditions: Vec<(String, Compare)>,
    /// What is the outcome from doing this action
    pub options: Vec<(Effect, usize)>,
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.preconditions.hash(state);
        self.options.hash(state);
    }
}

impl Action {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            preconditions: vec![],
            options: vec![],
        }
    }

    pub fn build() -> Self {
        Self::default()
    }

    pub fn with_precondition(mut self, key: &str, compare: Compare) -> Self {
        self.preconditions.push((key.to_string(), compare));
        self
    }

    pub fn with_effect(mut self, effect: Effect, cost: usize) -> Self {
        self.options.push((effect, cost));
        self
    }
}
