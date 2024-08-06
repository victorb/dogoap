// bevy_dogoap/src/traits.rs
use std::any::Any;
use std::fmt;

use bevy::prelude::*;

use dogoap::prelude::*;

#[reflect_trait]
pub trait InserterComponent: Send + Sync {
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
    fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity);
    fn clone_box(&self) -> Box<dyn InserterComponent>;
    fn as_any(&self) -> &dyn Any;
}

impl<T> InserterComponent for T
where
    T: Component + Clone + Send + Sync,
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
    fn as_any(&self) -> &dyn Any {
        self
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
    fn set_value(&mut self, new_val: Datum);
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
}

pub trait ActionComponent: Send + Sync {
    fn key() -> String;
}

pub trait ActionBuilder {
    fn new() -> Action;
}

pub trait EnumDatum: Send + Sync {
    fn datum(self) -> Datum;
}

pub trait ActionTrait {
    fn add_precondition(self, precondition: (String, Compare)) -> Self;
    fn add_mutator(self, mutator: Mutator) -> Self;
}

impl ActionTrait for Action {
    fn add_precondition(mut self, precondition: (String, Compare)) -> Self {
        self.preconditions.push(precondition);
        self
    }
    fn add_mutator(mut self, mutator: Mutator) -> Self {
        if self.effects.len() == 0 {
            self.effects = vec![(Effect::new(&self.key.clone()).with_mutator(mutator), 1)];
        } else {
            let mut effect = self.effects[0].0.clone();
            effect.mutators.push(mutator);
            self.effects[0] = (effect, 1);
        }
        self
    }
}

pub trait Precondition<T> {
    fn is(val: T) -> (String, Compare);
    fn is_not(val: T) -> (String, Compare);
    fn is_more(val: T) -> (String, Compare);
    // fn IsLess(val: T) -> (String, Compare);
}

pub trait MutatorTrait<T> {
    fn set(val: T) -> Mutator;
    fn increase(val: T) -> Mutator;
}