```
  +-[ rust examples with the new england patriots ]----------+
  |  ownership . traits . iterators . pattern matching       |
  |  2004 world champions as your compiler-enforced data     |
  +---------------------------------------------------------+
```

rust programming concepts through the lens of the 2004 new england patriots --
the greatest team ever assembled, now teaching you the greatest systems language
ever compiled.

mirrors [pythonExamplesWithNewEnglandPatriots](https://github.com/BryanChasko/pythonExamplesWithNewEnglandPatriots)
topic-for-topic where concepts apply in both languages, extended with rust-specific
concepts that have no clean python equivalent (ownership, borrowing, lifetimes,
pattern matching, enums, the trait system).

2004 patriots stats, rosters, and play data live in `datasets/`. same source as the python repo.

---

## modules

```
src/
  functions/         -- tom brady stat calculator, ownership of function args
  variables/         -- binding, shadowing, type inference with player data
  control_flow/      -- conditionals + pattern matching on game state
  loops_iterators/   -- Iterator trait, map/filter/fold on roster data
  error_handling/    -- Result<T,E> and Option<T> replacing exceptions
  structs_traits/    -- Player, Game, Drive -- OOP concepts via structs + impl
  enums/             -- Formation, PlayType, Quarter as type-safe enums
  closures/          -- stat aggregation via closures and higher-order functions
  generics/          -- generic roster queries, bounded type params
  lifetimes/         -- borrowed player references, lifetime annotations
  testing/           -- #[test], assert!, integration tests with real data
  file_io/           -- std::fs reading play-by-play CSV, serde for JSON
  regex/             -- regex crate: parsing box scores, play descriptions
  concurrency/       -- rayon parallel stats on full season data
datasets/            -- 2004 patriots rosters, game logs, play-by-play
```

---

## prerequisites

- rust stable via [rustup](https://rustup.rs)
- run any module: `cargo run --example <module_name>`
- run all tests: `cargo test`
- run with data: examples read from `datasets/` relative path

---

## relationship to the python repo

| topic | python module | rust module |
|-------|--------------|-------------|
| functions | basic_function_tom_brady_calculator | functions/ |
| loops + dicts | basic_loops_dictionaries | loops_iterators/ (Iterator trait) |
| exceptions | basic_function_exception_logging | error_handling/ (Result) |
| libraries | basic_packages_cowsay | -- (see crates in Cargo.toml) |
| tests | basic_test_calculator | testing/ (#[test]) |
| file i/o | basic_example_working_with_files | file_io/ (std::fs + serde) |
| regex | python_regex/ | regex/ (regex crate) |
| oop | object_oriented_programming/ | structs_traits/ + enums/ |
| -- | n/a | ownership/ (rust-only) |
| -- | n/a | lifetimes/ (rust-only) |
| -- | n/a | concurrency/ (rust-only) |

---

style guide: https://github.com/BryanChasko/heraldstack-mcp/blob/main/STYLE_GUIDE.md
