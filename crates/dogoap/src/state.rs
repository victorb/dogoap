use std::collections::BTreeMap;
// use scc::HashMap as BTreeMap;
// use rustc_hash::FxHashMap as BTreeMap;
// use ahash::HashMapExt;
// use indexset::BTreeMap;
// use cow_hashmap::CowHashMap as BTreeMap;
use std::hash::{Hash, Hasher};

// use ahash::{AHasher, RandomState};
// use std::collections::HashMap as BTreeMap;

// use ahash::AHashMap as BTreeMap;

use crate::datum::Datum;
use crate::goal::Goal;

use bevy_reflect::*;
// use indexmap::IndexMap;

#[derive(Reflect, Debug, Clone, Eq, PartialEq, Default)]
pub struct LocalState {
    pub data: BTreeMap<String, Datum>,
}

impl LocalState {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn with_datum(mut self, key: &str, value: Datum) -> Self {
        self.data.insert(key.to_string(), value);
        self
    }

    pub fn distance_to_goal(&self, goal: &Goal) -> u64 {
        goal.requirements.iter().map(|(key, goal_val)| {
            match self.data.get(key) {
                Some(state_val) => state_val.distance(&goal_val.value()),
                None => 1, // Penalty for missing keys
            }
        }).sum()
    }
}

impl Hash for LocalState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.len().hash(state);
        for (key, value) in &self.data {
            key.hash(state);
            value.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{compare::Compare, goal::Goal};

    #[test]
    fn test_distance_to_goal() {
        let state = LocalState::new().with_datum("energy", Datum::I64(50));
        let goal_state = Goal::new().with_req("energy", Compare::Equals(Datum::I64(50)));
        let distance = state.distance_to_goal(&goal_state.clone());
        assert_eq!(distance, 0);

        let state = LocalState::new().with_datum("energy", Datum::I64(25));
        let goal_state = Goal::new().with_req("energy", Compare::Equals(Datum::I64(50)));
        let distance = state.distance_to_goal(&goal_state.clone());
        assert_eq!(distance, 25);

        let state = LocalState::new().with_datum("energy", Datum::I64(25)).with_datum("hunger", Datum::F64(25.0));
        let goal_state = Goal::new().with_req("energy", Compare::Equals(Datum::I64(50))).with_req("hunger", Compare::Equals(Datum::F64(50.0)));
        let distance = state.distance_to_goal(&goal_state.clone());
        assert_eq!(distance, 50);
    }
}
