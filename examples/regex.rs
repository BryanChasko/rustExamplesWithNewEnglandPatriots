// examples/regex.rs
//
// regex crate compiles patterns once for zero per-call overhead.
// parse play-by-play text into a structured box score.

use rust_examples_patriots::regex::play_parser::{parse_play, BoxScore};

fn main() {
    println!("=== regex: parsing 2004 patriots play-by-play ===\n");

    let plays = vec![
        "Brady pass complete to Branch for 23 yards",
        "Dillon rush for 8 yards",
        "Brady pass incomplete to Givens",
        "Brady pass complete to Patten for 45 yards",
        "Dillon rush for 12 yards",
        "Brady pass complete to Givens for 5 yards TOUCHDOWN",
        "Dillon rush for 2 yards TOUCHDOWN",
        "Brady sacked by Freeney for -8 yards",
        "Brady pass complete to Graham for 18 yards",
        "Faulk rush for 6 yards",
        "Brady pass complete to Branch for 31 yards TOUCHDOWN",
        "timeout called by indianapolis",
    ];

    println!("-- parsing {} plays --\n", plays.len());

    for play_text in &plays {
        let result = parse_play(play_text);
        println!("  input:  {play_text}");
        println!("  parsed: {result:?}\n");
    }

    // build box score from all plays
    let score = BoxScore::from_plays(&plays);

    println!("-- box score --");
    println!(
        "  passing: {}/{} ({:.1}%)",
        score.completions,
        score.pass_attempts,
        score.completion_pct()
    );
    println!("  passing yards: {}", score.passing_yards);
    println!("  passing tds: {}", score.passing_tds);
    println!("  rushing attempts: {}", score.rush_attempts);
    println!("  rushing yards: {}", score.rushing_yards);
    println!("  rushing tds: {}", score.rushing_tds);
    println!("  sacks taken: {}", score.sacks_taken);

    println!("\n-- targets --");
    let mut targets: Vec<_> = score.targets.iter().collect();
    targets.sort_by(|a, b| b.1.cmp(a.1));
    for (name, count) in &targets {
        let yards = score.receiving_yards.get(*name).unwrap_or(&0);
        println!("  {name}: {count} targets, {yards} yards");
    }

    println!("\n=== key takeaway ===");
    println!("OnceLock compiles each regex exactly once (thread-safe, zero-cost");
    println!("after init). named capture groups make extraction readable.");
    println!("python's re.compile() is similar but lacks compile-time guarantees");
    println!("about pattern validity.");
}
