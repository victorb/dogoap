use crate::prelude::*;

pub fn simple_action<T>(name: &str, key_to_mutate: &str, from_value: T) -> Action
where
    Field: From<T>,
{
    simple_multi_mutate_action(name, vec![(key_to_mutate, from_value)])
}

pub fn simple_multi_mutate_action<T>(name: &str, muts: Vec<(&str, T)>) -> Action
where
    Field: From<T>,
{
    let mut mutators = vec![];

    for m in muts {
        mutators.push(Mutator::Set(m.0.to_string(), m.1.into()));
    }

    Action {
        key: name.to_string(),
        preconditions: None,
        options: vec![(
            Effect {
                action: name.to_string(),
                argument: None,
                mutators,
                state: LocalState::new(),
            },
            1,
        )],
    }
}

pub fn simple_increment_action<T>(name: &str, key_to_mutate: &str, from_value: T) -> Action
where
    Field: From<T>,
{
    let mut action = simple_multi_mutate_action(name, vec![]);
    action.options = vec![(
        Effect {
            action: name.to_string(),
            argument: None,
            mutators: vec![Mutator::Increment(
                key_to_mutate.to_string(),
                from_value.into(),
            )],
            state: LocalState::new(),
        },
        1,
    )];
    action
}

pub fn simple_decrement_action<T>(name: &str, key_to_mutate: &str, from_value: T) -> Action
where
    Field: From<T>,
{
    let mut action = simple_multi_mutate_action(name, vec![]);
    action.options = vec![(
        Effect {
            action: name.to_string(),
            argument: None,
            mutators: vec![Mutator::Decrement(
                key_to_mutate.to_string(),
                from_value.into(),
            )],
            state: LocalState::new(),
        },
        1,
    )];
    action
}

pub fn add_preconditions(action: &mut Action, preconds: Vec<(&str, Compare)>) {
    let mut new_preconds = vec![];
    for pc in preconds {
        new_preconds.push((pc.0.to_string(), pc.1));
    }
    action.preconditions = Some(new_preconds);
}
