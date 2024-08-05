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

pub trait EnumDatum: Send + Sync {
    fn datum(self) -> Datum;
}
