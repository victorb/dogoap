use std::fmt;

use bevy::prelude::{reflect_trait, Commands, Component, Entity};

use dogoap::prelude::{Action, Compare, Datum, Mutator};

/// A [`Component`] that can insert/remove itself to/from an Entity
/// Used for adding/removing current [`Action`] our planner tells us to perform
#[reflect_trait]
pub trait InserterComponent: Send + Sync + 'static {
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
    fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity);
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
}

impl fmt::Debug for dyn InserterComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MarkerComponent [DebugNotImplemented!]",)
    }
}

/// A [`Component` ] that can be used as a [`Mutator`] and [`Precondition`]
///
/// Example:
///
/// ```ignore
/// #[derive(DatumComponent)]
/// struct Hunger(f32);
///
/// // Used as a Mutator:
/// assert_eq!(
///     Hunger::increase(1.0),
///     Mutator::Increment("hunger".to_string(), Datum::F64(2.0))
/// );
///
/// // Used as a Precondition:
/// assert_eq!(
///     Hunger::is_less(10.0),
///     ("hunger".to_string(), Compare::LessThanEquals(Datum::F64(10.0)))
/// )
/// ```
#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait DatumComponent: Send + Sync {
    fn field_key(&self) -> String;
    fn field_value(&self) -> Datum;
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
/// struct DrinkAction;
///
/// // Used as a shorter way of creating a new Action with snake_case name
/// assert_eq!(
///     DrinkAction::new(),
///     Action::new("drink_action")
/// );
/// ```
///
/// Combined with [`DatumComponent`] to used as Mutator and Precondition
///
/// ```rust
/// # use bevy_dogoap::prelude::*;
/// # #[derive(ActionComponent)]
/// # struct DrinkAction;
///
/// #[derive(DatumComponent)]
/// struct Thirst(f64);
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
