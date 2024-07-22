use crate::{action::Action, field::Field, state::LocalState};
use bevy_reflect::Reflect;
use std::hash::{Hash, Hasher};

// TODO rename these to more rust familiar names?
// https://doc.rust-lang.org/reference/expressions/operator-expr.html#comparison-operators
#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum Compare {
    Equals(Field),
    NotEquals(Field),
    GreaterThanEquals(Field),
    LessThanEquals(Field),
}

impl Compare {
    pub fn value(&self) -> Field {
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
            Compare::Equals(field) => {
                0_u8.hash(state);
                field.hash(state);
            },
            Compare::NotEquals(field) => {
                1_u8.hash(state);
                field.hash(state);
            },
            Compare::GreaterThanEquals(field) => {
                2_u8.hash(state);
                field.hash(state);
            },
            Compare::LessThanEquals(field) => {
                3_u8.hash(state);
                field.hash(state);
            },
        }
    }
}

// fn compare_values(key: &str, value: &Value, state: &LocalState, comparison: &Compare) -> bool {
//     match comparison {
//         Compare::Equals(v) => state.fields.get(key) == Some(v),
//         Compare::GreaterThanEquals(v) => state.fields.get(key) >= Some(v),
//     }
// }

pub fn compare_values(comparison: &Compare, value: &Field) -> bool {
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
                .fields
                .get(key)
                .unwrap_or_else(|| panic!("Couldn't find key {:#?} in LocalState", key));
            compare_values(value, state_value)
        })
        // pre.iter().all(|(key, value)| match value {
        //     Compare::Equals(v) => state.fields.get(key) == Some(v),
        //     Compare::GreaterThanEquals(v) => state.fields.get(key) >= Some(v),
        // })
    })
}

#[cfg(test)]
mod test {
    use crate::compare::check_preconditions;
    use crate::compare::compare_values;
    use crate::prelude::*;

    #[test]
    fn test_check_preconditions_empty() {
        let state = LocalState::default().with_field("is_hungry", Field::from(true));
        let action = Action::default();

        let result = check_preconditions(&state, &action);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_preconditions_true() {
        let state = LocalState::default().with_field("is_hungry", Field::from(true));
        let action =
            Action::default().with_precondition("is_hungry", Compare::Equals(Field::from(true)));

        let result = check_preconditions(&state, &action);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_preconditions_false() {
        let state = LocalState::default().with_field("is_hungry", Field::from(true));
        let action =
            Action::default().with_precondition("is_hungry", Compare::Equals(Field::from(false)));

        let result = check_preconditions(&state, &action);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_preconditions_conflicting_preconditions() {
        let state = LocalState::default().with_field("is_hungry", Field::from(true));

        // False + True
        let action = Action::default()
            .with_precondition("is_hungry", Compare::Equals(Field::from(false)))
            .with_precondition("is_hungry", Compare::Equals(Field::from(true)));

        let result = check_preconditions(&state, &action);
        assert_eq!(result, false);

        // True + False
        let action = Action::default()
            .with_precondition("is_hungry", Compare::Equals(Field::from(true)))
            .with_precondition("is_hungry", Compare::Equals(Field::from(false)));

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
                &Compare::GreaterThanEquals(Field::from(val1)),
                &Field::from(val2),
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
                &Compare::LessThanEquals(Field::from(val1)),
                &Field::from(val2),
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
                &Compare::NotEquals(Field::from(val1)),
                &Field::from(val2),
            );
            assert_eq!(
                ret, expected,
                "Expected {} to not be equal to {}, but compare_values returned {:#?}",
                val1, val2, ret
            );
        }
    }
}
