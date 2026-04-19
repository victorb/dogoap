use std::hash::{Hash, Hasher};

use std::collections::BTreeMap;

// List of other hashmap-likes we've tried, but none faster than BTreeMap
// use scc::HashMap as BTreeMap;
// use rustc_hash::FxHashMap as BTreeMap;
// use ahash::HashMapExt;
// use indexset::BTreeMap;
// use cow_hashmap::CowHashMap as BTreeMap;
// use ahash::{AHasher, RandomState};
// use std::collections::HashMap as BTreeMap;
// use ahash::AHashMap as BTreeMap;
// use indexmap::IndexMap; // 37,873.88 ns/iter
// use micromap::Map; // 30,480.55 ns/iter

use crate::datum::Datum;
use crate::goal::Goal;

pub type InternalData = BTreeMap<String, Datum>;

/// This is our internal state that the planner uses to progress in the path finding,
/// until we reach our [`Goal`]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct LocalState {
    /// The data stored in this local state
    pub data: InternalData,
}

impl LocalState {
    /// Create a new empty local state
    pub fn new() -> Self {
        Self {
            data: InternalData::new(),
        }
    }

    /// Create a new local state with a single datum
    pub fn with_datum(mut self, key: impl Into<String>, value: impl Into<Datum>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// The total distance to the goal in terms of differences between the goal's requirements and the local state's data
    pub fn distance_to_goal(&self, goal: &Goal) -> u64 {
        goal.requirements
            .iter()
            .map(|(key, goal_val)| {
                match self.data.get(key) {
                    Some(state_val) => state_val.distance(&goal_val.value(self.data.clone())),
                    None => 1, // Penalty for missing keys
                }
            })
            .sum()
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
        let state = LocalState::new().with_datum("energy", 50_i64);
        let goal_state = Goal::new().with_req("energy", Compare::equals(50_i64));
        let distance = state.distance_to_goal(&goal_state.clone());
        assert_eq!(distance, 0);

        let state = LocalState::new().with_datum("energy", 25_i64);
        let goal_state = Goal::new().with_req("energy", Compare::equals(50_i64));
        let distance = state.distance_to_goal(&goal_state.clone());
        assert_eq!(distance, 25);

        let state = LocalState::new()
            .with_datum("energy", 25_i64)
            .with_datum("hunger", 25.0_f64);
        let goal_state = Goal::new()
            .with_req("energy", Compare::equals(50_i64))
            .with_req("hunger", Compare::equals(50.0_f64));
        let distance = state.distance_to_goal(&goal_state.clone());
        assert_eq!(distance, 50);
    }
}
