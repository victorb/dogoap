use std::fmt;

use bevy::prelude::{reflect_trait, Commands, Component, Entity};

use dogoap::prelude::{Action, Compare, Datum, Mutator};

#[reflect_trait]
pub trait InserterComponent: Send + Sync + 'static {
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
    fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity);
    fn clone_box(&self) -> Box<dyn InserterComponent>;
}

impl<T> InserterComponent for T
where
    T: Component + Clone + Send + Sync + 'static,
{
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity) {
        commands.entity(entity_to_insert_to).insert(T::clone(self));
    }
    fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity) {
        commands.entity(entity_to_remove_from).remove::<T>();
    }
    fn clone_box(&self) -> Box<dyn InserterComponent> {
        Box::new(self.clone())
    }
}

impl fmt::Debug for dyn InserterComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MarkerComponent [DebugNotImplemented!]",)
    }
}

#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait DatumComponent: Send + Sync {
    fn field_key(&self) -> String;
    fn field_value(&self) -> Datum;
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
}

/// ActionComponent allows you to create Actions directly from your action struct
///
/// Can be derived with `#derive(ActionComponent)`
///
/// Example:
///
/// ```rust
/// # use bevy_dogoap::prelude::*;
/// #[derive(ActionComponent)]
/// struct MyAction;
///
/// // Used as a shorter way of creating a new Action with snake_case name
/// assert_eq!(
///     MyAction::new(),
///     Action::new("my_action")
/// );
/// ```
#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait ActionComponent: Send + Sync {
    /// Gets the action key but in snake_case ("AtLocation" becomes "at_location")
    fn key() -> String
    where
        Self: Sized;
    /// Creates a new [`Action`] with our snake_case key
    fn new() -> Action
    where
        Self: Sized;
    /// Returns the type name
    fn action_type_name(&self) -> &'static str;
}

pub trait EnumDatum: Send + Sync {
    fn datum(self) -> Datum;
}

// Implemented by derive DatumComponent
pub trait Precondition<T> {
    fn is(val: T) -> (String, Compare);
    fn is_not(val: T) -> (String, Compare);
    fn is_more(val: T) -> (String, Compare);
    fn is_less(val: T) -> (String, Compare);
}

// Implemented by derive DatumComponent in order to mutate
pub trait MutatorTrait<T> {
    fn set(val: T) -> Mutator;
    fn increase(val: T) -> Mutator;
    fn decrease(val: T) -> Mutator;
}
