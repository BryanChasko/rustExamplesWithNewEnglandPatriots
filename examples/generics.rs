// examples/generics.rs
//
// generics let one function work across multiple stat types.
// the compiler monomorphizes: one machine-code copy per concrete type used.

use rust_examples_patriots::generics::roster_query::{
    above_threshold, find_leader, top_n, top_n_float, total_stat, PlayerStat, StatLeader,
};

fn main() {
    println!("=== generics: 2004 patriots stat queries ===\n");

    // rushing yards -- u32 stats
    let rushing = vec![
        PlayerStat::new("Corey Dillon", 1635u32),
        PlayerStat::new("Kevin Faulk", 255),
        PlayerStat::new("Patrick Pass", 141),
        PlayerStat::new("Cedric Cobbs", 90),
        PlayerStat::new("Tom Brady", 28),
    ];

    println!("-- top_n<u32>: rushing yards leaders --");
    let top3 = top_n(&rushing, 3);
    for (i, ps) in top3.iter().enumerate() {
        println!("  {}. {} — {} yards", i + 1, ps.name, ps.value);
    }

    let leader = find_leader(&rushing, "rushing yards").unwrap();
    println!("\nrushing leader: {leader}");
    println!("team total: {} yards", total_stat(&rushing));

    // receiving touchdowns -- u8 stats (same generic function)
    let rec_tds = vec![
        PlayerStat::new("David Patten", 7u8),
        PlayerStat::new("Daniel Graham", 7),
        PlayerStat::new("Deion Branch", 4),
        PlayerStat::new("David Givens", 3),
        PlayerStat::new("Corey Dillon", 1),
    ];

    println!("\n-- top_n<u8>: receiving td leaders --");
    let top2 = top_n(&rec_tds, 2);
    for (i, ps) in top2.iter().enumerate() {
        println!("  {}. {} — {} tds", i + 1, ps.name, ps.value);
    }

    // passer rating -- f32 (needs top_n_float because f32 is not Ord)
    let ratings = vec![
        PlayerStat::new("Tom Brady", 92.6f32),
        PlayerStat::new("Rohan Davey", 0.0),
    ];

    println!("\n-- top_n_float: passer rating --");
    let best = top_n_float(&ratings, 1);
    println!(
        "  best passer: {} — {:.1} rating",
        best[0].name, best[0].value
    );

    // above_threshold: generic filter
    let over_100 = above_threshold(&rushing, &100);
    println!("\n-- above_threshold<u32>: rushers over 100 yards --");
    for ps in &over_100 {
        println!("  {} — {} yards", ps.name, ps.value);
    }

    // StatLeader<T> display
    println!("\n-- StatLeader<T> struct instances --");
    let passing: StatLeader<u32> = StatLeader::new("Tom Brady", "passing yards", 3692);
    let rating: StatLeader<f32> = StatLeader::new("Tom Brady", "passer rating", 92.6);
    println!("  {passing}");
    println!("  {rating}");

    println!("\n=== key takeaway ===");
    println!("one `top_n` function handles u8, u16, u32, u64 -- the compiler");
    println!("generates optimized machine code for each type actually used.");
    println!("python achieves similar syntax with duck typing but pays a");
    println!("runtime cost on every comparison (dynamic dispatch, type checks).");
}
