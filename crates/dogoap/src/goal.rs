use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use bevy_reflect::*;

// use crate::requirement::DynRequirement;
use crate::compare::Compare;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct Goal {
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
    // why both new and build?
    pub fn new() -> Self {
        Self {
            requirements: BTreeMap::new(),
        }
    }

    pub fn build() -> Self {
        Self {
            requirements: BTreeMap::new(),
        }
    }

    pub fn with_req(mut self, key: &str, compare: Compare) -> Self {
        self.requirements.insert(key.to_string(), compare);
        self
    }
}


impl Default for Goal {
    fn default() -> Self {
        Goal::new()
    }
}
