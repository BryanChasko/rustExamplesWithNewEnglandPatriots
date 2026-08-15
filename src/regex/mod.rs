// regex — compile-once pattern matching
//
// the regex crate compiles patterns at startup for zero per-call overhead.
// parse play-by-play text into structured data.

pub mod play_parser;

pub use play_parser::{parse_play, BoxScore, PlayResult};
