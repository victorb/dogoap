#![feature(trivial_bounds)]
#![doc = include_str!("../README.md")]
mod action;
mod compare;
mod datum;
mod effect;
mod goal;
mod localstate;
mod mutator;

pub mod planner;
pub mod prelude;
pub mod simple;
