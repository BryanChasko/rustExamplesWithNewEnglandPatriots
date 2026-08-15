// lifetimes — rust-only concept, no python equivalent
//
// lifetimes let the compiler verify that borrowed references don't outlive
// the data they point to. the 2004 patriots roster demonstrates borrowed
// references with explicit lifetime annotations.

pub mod scout_report;

pub use scout_report::{Player, RosterView, ScoutReport};
