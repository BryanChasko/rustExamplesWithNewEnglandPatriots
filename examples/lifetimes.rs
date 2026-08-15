// examples/lifetimes.rs
//
// lifetimes are rust-only -- no python equivalent.
// the borrow checker uses lifetimes to guarantee references never dangle.
// this example shows how the 2004 patriots roster can be borrowed safely.

use rust_examples_patriots::lifetimes::{
    scout_report::{first_player, highest_number, longer_career, position_group, team_motto},
    Player, RosterView, ScoutReport,
};

fn main() {
    let roster = vec![
        Player::new("Tom Brady", "QB", 12, 5),
        Player::new("Corey Dillon", "RB", 28, 8),
        Player::new("Deion Branch", "WR", 83, 3),
        Player::new("David Givens", "WR", 87, 3),
        Player::new("Tedy Bruschi", "LB", 54, 9),
        Player::new("Richard Seymour", "DL", 93, 4),
        Player::new("Ty Law", "CB", 24, 10),
        Player::new("Adam Vinatieri", "K", 4, 9),
    ];

    println!("=== lifetimes: 2004 patriots scout reports ===\n");

    // 'static lifetime: string literals live for the entire program
    let motto: &'static str = team_motto();
    println!("team motto: {motto}\n");

    // ScoutReport borrows from the roster -- cannot outlive it
    let notes = "franchise qb, 2x super bowl champion";
    let brady_report = ScoutReport::new(&roster[0], 9.8, notes);
    println!("{brady_report}");

    let bruschi_report = ScoutReport::new(&roster[4], 9.2, "defensive captain, heart of the team");
    println!("{bruschi_report}\n");

    // longer_career: returns a reference with lifetime tied to both inputs
    let veteran = longer_career(&roster[0], &roster[6]);
    println!("longer career between brady and law: {veteran}");
    println!("  ({} years pro)\n", veteran.years_pro);

    // highest_number: lifetime elision from single slice input
    if let Some(highest) = highest_number(&roster) {
        println!("highest jersey number: {highest}");
    }

    // position_group: returns borrowed references from the slice
    let wrs = position_group(&roster, "WR");
    println!("\nwide receivers ({} total):", wrs.len());
    for wr in &wrs {
        println!("  {wr}");
    }

    // RosterView: struct with lifetime parameter on a borrowed slice
    let view = RosterView::new("2004 new england patriots", &roster);
    println!(
        "\nroster view: {} ({} players)",
        view.team,
        view.players.len()
    );

    if let Some(vet) = view.veteran() {
        println!("team veteran: {} ({} years)", vet.name, vet.years_pro);
    }
    println!("linebackers on roster: {}", view.count_position("LB"));

    // first_player: lifetime elision example (compiler infers from single input)
    if let Some(first) = first_player(&roster) {
        println!("\nfirst on roster: {first}");
    }

    println!("\n=== key takeaway ===");
    println!("rust's lifetime system guarantees at compile time that no reference");
    println!("outlives the data it points to. python has no equivalent -- it relies");
    println!("on garbage collection, which cannot prevent use-after-free in unsafe");
    println!("FFI or concurrent code.");
}
