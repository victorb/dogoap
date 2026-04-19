use crate::{datum::Datum, localstate::InternalData};

/// Describes a change in [`LocalState`](crate::localstate::LocalState), based on
/// the String key + a [`Datum`]
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum Mutator {
    /// Set a value for a key
    Set(String, Datum), // :key, :value
    /// Increment a value for a key by a given amount
    Increment(String, Datum), // :key, :increment-by
    /// Decrement a value for a key by a given amount
    Decrement(String, Datum), // :key, :decrement-by
    /// Mutate a key based on another key
    Reference(String, String, ReferenceMutator),
}

/// Describes how a reference [`Mutator`] should be applied.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum ReferenceMutator {
    /// Set a value for a key to the reference
    Set, // :key, :value
    /// Increment a value for a key by the referenced amount
    Increment, // :key, :increment-by
    /// Decrement a value for a key by the referenced amount
    Decrement, // :key, :decrement-by
}

impl Mutator {
    /// Convenience method for creating a [`Mutator::Set`]
    pub fn set(key: impl Into<String>, value: impl Into<Datum>) -> Self {
        Mutator::Set(key.into(), value.into())
    }

    /// Convenience method for creating a [`Mutator::Increment`]
    pub fn increment(key: impl Into<String>, value: impl Into<Datum>) -> Self {
        Mutator::Increment(key.into(), value.into())
    }

    /// Convenience method for creating a [`Mutator::Decrement`]
    pub fn decrement(key: impl Into<String>, value: impl Into<Datum>) -> Self {
        Mutator::Decrement(key.into(), value.into())
    }

    /// Convenience method for creating a [`Mutator::Reference`]
    pub fn reference(key: impl Into<String>, other_key: impl Into<String>, mutator: ReferenceMutator) -> Self {
        Mutator::Reference(key.into(), other_key.into(), mutator)
    }
}

pub fn apply_mutator(data: &mut InternalData, mutator: &Mutator) {
    match mutator {
        Mutator::Set(key, value) => {
            data.insert(key.to_string(), *value);
        }
        Mutator::Increment(key, value) => {
            if let Some(current_value) = data.get_mut(key) {
                *current_value += *value;
            }
        }
        Mutator::Decrement(key, value) => {
            if let Some(current_value) = data.get_mut(key) {
                *current_value -= *value;
            }
        }
        Mutator::Reference(key, other_key, mutator) => {
            let Some(other_value) = data.get(other_key) else {
                return;
            };

            let other_value = *other_value;

            let Some(current_value) = data.get_mut(key) else {
                return;
            };

            match mutator {
                ReferenceMutator::Set => match (current_value.clone(), other_value) {
                    (Datum::Bool(..), Datum::Bool(..))
                    | (Datum::F64(..), Datum::F64(..))
                    | (Datum::I64(..), Datum::I64(..))
                    | (Datum::Enum(..), Datum::Enum(..)) => {
                        *current_value = other_value;
                    }
                    _ => return,
                },
                ReferenceMutator::Increment => {
                    *current_value += other_value;
                }
                ReferenceMutator::Decrement => {
                    *current_value -= other_value;
                }
            }
        }
    }
}

/// Formats a human-readable version of a list of [`Mutator`]s.
/// Used in [`format_plan`](crate::prelude::format_plan).
pub fn format_mutators(mutators: Vec<Mutator>) -> String {
    let mut output = String::new();
    for mutator in mutators {
        match mutator {
            Mutator::Set(k, v) => {
                output.push_str(&format!("\t\t{k} = {v}\n"));
            }
            Mutator::Increment(k, v) => {
                output.push_str(&format!("\t\t{k} + {v}\n"));
            }
            Mutator::Decrement(k, v) => {
                output.push_str(&format!("\t\t{k} - {v}\n"));
            }
            Mutator::Reference(k, ok, m) => match m {
                ReferenceMutator::Set => {
                    output.push_str(&format!("\t\t{k} = {ok}\n"));
                }
                ReferenceMutator::Increment => {
                    output.push_str(&format!("\t\t{k} + {ok}\n"));
                }
                ReferenceMutator::Decrement => {
                    output.push_str(&format!("\t\t{k} - {ok}\n"));
                }
            },
        }
    }
    output
}
