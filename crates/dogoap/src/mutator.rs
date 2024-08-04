use crate::datum::Datum;

use bevy_reflect::*;

#[derive(Reflect, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Mutator {
    Set(String, Datum),       // :key, :value
    Increment(String, Datum), // :key, :increment-by
    Decrement(String, Datum), // :key, :decrement-by
}

pub fn print_mutators(mutators: Vec<Mutator>) {
    for mutator in mutators {
        match mutator {
            Mutator::Set(k, v) => {
                println!("\t\t{} = {}", k, v);
            }
            Mutator::Increment(k, v) => {
                println!("\t\t{} + {}", k, v);
            }
            Mutator::Decrement(k, v) => {
                println!("\t\t{} - {}", k, v);
            }
        }
    }
}
