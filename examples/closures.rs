// examples/closures.rs
//
// 2004 patriots -- closure-based stat aggregation
// demonstrates: filter by position, higher-order functions, FnMut tracking,
//               environment capture, and map/filter/fold chains

use rust_examples_patriots::closures::{
    compute_team_stats, min_tds_filter, min_yards_filter, patriots_2004_game_log,
    patriots_2004_roster, sample_game_stats, season_point_accumulator, top_performers,
    yards_per_game_tracker, Position,
};

fn main() {
    println!("2004 new england patriots -- closure demonstrations\n");
    println!("=== 1. filter roster by position (closure as predicate) ===\n");

    let roster = patriots_2004_roster();

    // closure that filters players by position
    let qbs: Vec<_> = roster
        .iter()
        .filter(|p| p.position == Position::QB)
        .collect();
    println!("quarterbacks:");
    for p in &qbs {
        println!(
            "  #{} {} ({} years)",
            p.number, p.name, p.years_with_patriots
        );
    }

    let receivers: Vec<_> = roster
        .iter()
        .filter(|p| p.position == Position::WR)
        .collect();
    println!("\nwide receivers:");
    for p in &receivers {
        println!(
            "  #{} {} ({} years)",
            p.number, p.name, p.years_with_patriots
        );
    }

    let defense: Vec<_> = roster
        .iter()
        .filter(|p| {
            matches!(
                p.position,
                Position::LB | Position::CB | Position::S | Position::DE | Position::DT
            )
        })
        .collect();
    println!("\ndefensive players:");
    for p in &defense {
        println!("  #{} {} ({:?})", p.number, p.name, p.position);
    }

    println!("\n=== 2. higher-order function with scoring predicate ===\n");

    let stats = sample_game_stats();

    // top_performers takes any predicate -- factory closures or inline
    let big_yardage = top_performers(&stats, min_yards_filter(100));
    println!("100+ yard performances:");
    for s in &big_yardage {
        println!(
            "  {} week {} -- {} yds, {} td",
            s.name, s.week, s.yards, s.tds
        );
    }

    let multi_td = top_performers(&stats, min_tds_filter(2));
    println!("\nmulti-td games:");
    for s in &multi_td {
        println!(
            "  {} week {} -- {} td, {} yds",
            s.name, s.week, s.tds, s.yards
        );
    }

    // inline closure -- custom composite predicate
    let brady_big_games = top_performers(&stats, |s| s.name == "Tom Brady" && s.yards >= 250);
    println!("\nbrady 250+ yard games:");
    for s in &brady_big_games {
        println!("  week {} -- {} yds, {} td", s.week, s.yards, s.tds);
    }

    println!("\n=== 3. closure capturing environment -- season point accumulator ===\n");

    let games = patriots_2004_game_log();
    let (games_played, total_points) = season_point_accumulator(&games);
    println!(
        "2004 season (reg + playoffs): {} games, {} total points scored",
        games_played, total_points
    );
    println!(
        "average: {:.1} points per game",
        total_points as f64 / games_played as f64
    );

    println!("\n=== 4. FnMut closure -- yards_per_game running average ===\n");

    // tracker mutates internal state on each call
    let mut tracker = yards_per_game_tracker();

    let brady_weekly_yards = [
        335u32, 230, 298, 260, 171, 225, 116, 258, 233, 327, 210, 264, 181, 144, 226, 236,
    ];
    println!("brady 2004 -- running average yards per game:");
    for (i, &yards) in brady_weekly_yards.iter().enumerate() {
        let avg = tracker(yards);
        if i < 5 || i >= brady_weekly_yards.len() - 3 {
            println!(
                "  week {:2}: {} yds | running avg: {:.1}",
                i + 1,
                yards,
                avg
            );
        } else if i == 5 {
            println!("  ...");
        }
    }

    println!("\n=== 5. map/filter/fold chain -- team season stats ===\n");

    let summary = compute_team_stats(&games);
    println!("2004 patriots season summary (iterator chains):");
    println!("  record: {}-{}", summary.wins, summary.losses);
    println!("  total points for: {}", summary.total_points_for);
    println!("  total points against: {}", summary.total_points_against);
    println!(
        "  point differential: +{}",
        summary.total_points_for as i32 - summary.total_points_against as i32
    );
    println!("  avg points/game: {:.1}", summary.avg_points_per_game);
    println!(
        "  avg margin of victory: {:.1}",
        summary.avg_margin_of_victory
    );
    println!(
        "  home wins: {} | away/neutral wins: {}",
        summary.home_wins, summary.away_wins
    );

    // bonus: fold chain computing per-opponent scoring
    println!("\npoints by game (fold accumulation):");
    let mut running = 0u32;
    let points_trace: Vec<(u8, u32)> = games
        .iter()
        .map(|g| {
            running += g.pts_patriots;
            (g.week, running)
        })
        .collect();
    for (week, cumulative) in points_trace.iter().take(5) {
        println!("  after week {}: {} cumulative points", week, cumulative);
    }
    if let Some((week, cumulative)) = points_trace.last() {
        println!("  ...");
        println!(
            "  after week {} (super bowl): {} cumulative points",
            week, cumulative
        );
    }
}
