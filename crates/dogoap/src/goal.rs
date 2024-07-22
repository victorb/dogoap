use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use bevy_reflect::*;

use crate::compare::Compare;

/// Goal is a map of what we want our final [`LocalState`](crate::localstate::LocalState) to be, using String as
/// keys and [`Compare`] to assert what we want the [`Datum`](crate::datum::Datum) to be
#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct Goal {
    /// All the requirements needed to be met in order to consider us to be at our final state
    pub requirements: BTreeMap<String, Compare>,
}

impl Hash for Goal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.requirements.len().hash(state);
        for (key, value) in &self.requirements {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Goal {
    pub fn new() -> Self {
        Self {
            requirements: BTreeMap::new(),
        }
    }

    pub fn with_req(mut self, key: &str, compare: Compare) -> Self {
        self.requirements.insert(key.to_string(), compare);
        self
    }

    pub fn from_reqs(preconditions: &[(String, Compare)]) -> Goal {
        let mut goal = Goal::new();
        for (k, v) in preconditions {
            goal = goal.with_req(k, v.clone());
        }
        goal
    }
}
