// generics — type-safe polymorphism without runtime cost
//
// generics let functions and structs work across multiple types.
// rust monomorphizes at compile time: one machine-code version per concrete type.

pub mod roster_query;

pub use roster_query::{top_n, StatLeader};
