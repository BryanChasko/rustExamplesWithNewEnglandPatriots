// examples/file_io.rs — demonstrates CSV parsing and serde JSON file I/O
//
// run: cargo run --example file_io

use anyhow::Result;
use rust_examples_patriots::file_io;
use std::path::Path;

fn main() -> Result<()> {
    println!("=== 2004 New England Patriots — File I/O ===\n");

    // --- CSV reading ---
    let roster_path = Path::new("data/roster_2004.csv");
    let roster = file_io::read_roster_csv(roster_path)?;

    println!("roster loaded: {} players from CSV\n", roster.len());
    println!("{:<20} {:>4} {:<4} college", "player", "num", "pos");
    println!("{}", "-".repeat(60));
    for entry in &roster {
        println!(
            "{:<20} {:>4} {:<4} {}",
            entry.player, entry.number, entry.position, entry.college
        );
    }

    // filter by position
    println!("\n--- wide receivers ---");
    let wrs = file_io::filter_by_position(&roster, "WR");
    for wr in &wrs {
        println!("  #{} {} ({})", wr.number, wr.player, wr.college);
    }

    // --- JSON deserialization ---
    println!("\n--- 2004 regular season results (from JSON) ---\n");
    let season_path = Path::new("data/season_2004.json");
    let games = file_io::read_season_json(season_path)?;

    println!(
        "{:<5} {:<25} {:>5} {:>5} result",
        "week", "opponent", "NE", "OPP"
    );
    println!("{}", "-".repeat(55));
    for game in &games {
        let result = if game.patriots_score > game.opponent_score {
            "W"
        } else {
            "L"
        };
        let venue = if game.home { "" } else { "@" };
        println!(
            "{:<5} {}{:<24} {:>5} {:>5} {}",
            game.week, venue, game.opponent, game.patriots_score, game.opponent_score, result
        );
    }

    // --- compute and serialize summary ---
    let summary = file_io::summarize_season(&games);
    println!("\n--- season summary ---");
    println!("record: {}-{}", summary.wins, summary.losses);
    println!("points scored: {}", summary.total_points_scored);
    println!("points allowed: {}", summary.total_points_allowed);
    println!("point differential: {:+}", summary.point_differential);

    // --- write results back to JSON ---
    let output_dir = Path::new("data/output");
    std::fs::create_dir_all(output_dir)?;

    let summary_path = output_dir.join("season_summary_2004.json");
    file_io::write_summary_json(&summary_path, &summary)?;
    println!("\nwrote season summary to: {}", summary_path.display());

    // write filtered wins to a separate file
    let wins: Vec<_> = games
        .iter()
        .filter(|g| g.patriots_score > g.opponent_score)
        .cloned()
        .collect();
    let wins_path = output_dir.join("wins_2004.json");
    file_io::write_results_json(&wins_path, &wins)?;
    println!("wrote {} wins to: {}", wins.len(), wins_path.display());

    // demonstrate roundtrip: read the file we just wrote
    let reloaded = file_io::read_season_json(&wins_path)?;
    println!(
        "\nroundtrip verified: read back {} game results from {}",
        reloaded.len(),
        wins_path.display()
    );

    println!("\n=== dynasty confirmed: file I/O complete ===");
    Ok(())
}
