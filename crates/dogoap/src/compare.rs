use crate::{action::Action, datum::Datum, state::LocalState};
use bevy_reflect::Reflect;
use std::hash::{Hash, Hasher};

// TODO rename these to more rust familiar names?
// https://doc.rust-lang.org/reference/expressions/operator-expr.html#comparison-operators
#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum Compare {
    Equals(Datum),
    NotEquals(Datum),
    GreaterThanEquals(Datum),
    LessThanEquals(Datum),
}

impl Compare {
    pub fn value(&self) -> Datum {
        match self {
            Compare::Equals(f) => *f,
            Compare::NotEquals(f) => *f,
            Compare::GreaterThanEquals(f) => *f,
            Compare::LessThanEquals(f) => *f,
        }
    }
}

impl Hash for Compare {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Compare::Equals(datum) => {
                0_u8.hash(state);
                datum.hash(state);
            },
            Compare::NotEquals(datum) => {
                1_u8.hash(state);
                datum.hash(state);
            },
            Compare::GreaterThanEquals(datum) => {
                2_u8.hash(state);
                datum.hash(state);
            },
            Compare::LessThanEquals(datum) => {
                3_u8.hash(state);
                datum.hash(state);
            },
        }
    }
}

pub fn compare_values(comparison: &Compare, value: &Datum) -> bool {
    match comparison {
        Compare::Equals(v) => value == v,
        Compare::NotEquals(v) => value != v,
        Compare::GreaterThanEquals(v) => value >= v,
        Compare::LessThanEquals(v) => value <= v,
    }
}

// TODO should be in Action or LocalState instead perhaps?
/// Checks all the preconditions from the `Action` against passed in `LocalState`
/// Returns `true` if all the preconditions pass, otherwise `false`
pub fn check_preconditions(state: &LocalState, action: &Action) -> bool {
    action.preconditions.as_ref().map_or(true, |pre| {
        pre.iter().all(|(key, value)| {
            let state_value = state
                .data
                .get(key)
                .unwrap_or_else(|| panic!("Couldn't find key {:#?} in LocalState", key));
            compare_values(value, state_value)
        })
    })
}

#[cfg(test)]
mod test {
    use crate::compare::check_preconditions;
    use crate::compare::compare_values;
    use crate::prelude::*;

    #[test]
    fn test_check_preconditions_empty() {
        let state = LocalState::default().with_datum("is_hungry", Datum::from(true));
        let action = Action::default();

        let result = check_preconditions(&state, &action);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_preconditions_true() {
        let state = LocalState::default().with_datum("is_hungry", Datum::from(true));
        let action =
            Action::default().with_precondition("is_hungry", Compare::Equals(Datum::from(true)));

        let result = check_preconditions(&state, &action);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_preconditions_false() {
        let state = LocalState::default().with_datum("is_hungry", Datum::from(true));
        let action =
            Action::default().with_precondition("is_hungry", Compare::Equals(Datum::from(false)));

        let result = check_preconditions(&state, &action);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_preconditions_conflicting_preconditions() {
        let state = LocalState::default().with_datum("is_hungry", Datum::from(true));

        // False + True
        let action = Action::default()
            .with_precondition("is_hungry", Compare::Equals(Datum::from(false)))
            .with_precondition("is_hungry", Compare::Equals(Datum::from(true)));

        let result = check_preconditions(&state, &action);
        assert_eq!(result, false);

        // True + False
        let action = Action::default()
            .with_precondition("is_hungry", Compare::Equals(Datum::from(true)))
            .with_precondition("is_hungry", Compare::Equals(Datum::from(false)));

        let result = check_preconditions(&state, &action);
        assert_eq!(result, false);
    }

    #[test]
    fn test_greater_than_equals() {
        let cases = vec![
            // is X greater than or equal to Y?
            (10, 10, true),
            (10, 9, false),
            (11, 10, false),
        ];

        for (val1, val2, expected) in cases {
            let ret = compare_values(
                &Compare::GreaterThanEquals(Datum::from(val1)),
                &Datum::from(val2),
            );
            assert_eq!(
                ret, expected,
                "Expected {} to be greater than or equal to {}, but compare_values returned {:#?}",
                val1, val2, ret
            );
        }
    }

    #[test]
    fn test_less_than_equals() {
        let cases = vec![
            // is X less than or equal to Y?
            (10, 10, true),
            (10, 9, true),
            (11, 10, true),
        ];

        for (val1, val2, expected) in cases {
            let ret = compare_values(
                &Compare::LessThanEquals(Datum::from(val1)),
                &Datum::from(val2),
            );
            assert_eq!(
                ret, expected,
                "Expected {} to be less than or equal to {}, but compare_values returned {:#?}",
                val1, val2, ret
            );
        }
    }

    #[test]
    fn test_not_equals() {
        let cases = vec![
            // is X less than or equal to Y?
            (10, 10, false),
            (10, 9, true),
            (11, 10, true),
        ];

        for (val1, val2, expected) in cases {
            let ret = compare_values(
                &Compare::NotEquals(Datum::from(val1)),
                &Datum::from(val2),
            );
            assert_eq!(
                ret, expected,
                "Expected {} to not be equal to {}, but compare_values returned {:#?}",
                val1, val2, ret
            );
        }
    }
}
