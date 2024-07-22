use crate::field::Field;

use bevy_reflect::*;

#[derive(Reflect, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Mutator {
    Set(String, Field),       // :key, :value
    Increment(String, Field), // :key, :increment-by
    Decrement(String, Field), // :key, :decrement-by
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
