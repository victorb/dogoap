// crate "dogoap" file action.rs
use std::hash::{Hash, Hasher};

use bevy_reflect::*;

use crate::compare::Compare;
use crate::effect::Effect;
use crate::mutator::Mutator;

/// An `Action` represents something your NPC can do, granted the LocalState
/// is as defined in the `preconditions`. It has a list of `Effect`s that apply
/// if the NPC successfully executed the task.
#[derive(Reflect, Clone, Debug, PartialEq, Default)]
pub struct Action {
    /// String like `eat_action`
    pub key: String,
    // TODO arguments coupled with Effects
    // pub argument: Option<Datum>,
    /// What preconditions need to be true before we can execute this action
    pub preconditions: Vec<(String, Compare)>,
    /// What is the outcome from doing this action
    pub effects: Vec<(Effect, usize)>,
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.preconditions.hash(state);
        self.effects.hash(state);
    }
}

impl Action {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            preconditions: vec![],
            effects: vec![],
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
        self.effects.push((effect, cost));
        self
    }

    pub fn with_mutator(&mut self, mutator: Mutator) -> Self {
        if self.effects.len() == 0 {
            self.effects = vec![(Effect::new(&self.key.clone()), 1)];
            self.clone()
        } else {
            let ref mut effect = &mut self.effects[0].0;
            effect.mutators.push(mutator);
            // }

            // let mut effect = self.options.first().unwrap().0.clone();
            // effect.mutators.push(mutator);
            self.clone()
        }
    }
}
