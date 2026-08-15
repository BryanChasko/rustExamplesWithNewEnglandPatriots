// concurrency — data-race-free parallelism via ownership
//
// rust's ownership model makes data races a compile-time error.
// rayon adds effortless data parallelism via par_iter().
// no python equivalent: python's GIL prevents true thread parallelism.

pub mod parallel_stats;

pub use parallel_stats::{GameStats, SeasonAggregator};
