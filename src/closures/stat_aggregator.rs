// closures -- stat_aggregator
//
// demonstrates: closure syntax, capturing by ref + by value (move), Fn/FnMut/FnOnce,
//               closures as arguments, returning closures, higher-order functions
// python mirror: libraries_2_statistics_2023_patriots_qb_shuffle.py
//
// 2004 patriots: build stat aggregators as first-class closures

#[derive(Debug, Clone)]
struct PlayerStat {
    name: &'static str,
    position: &'static str,
    yards: u32,
    tds: u32,
    games: u8,
}

impl PlayerStat {
    fn yards_per_game(&self) -> f64 {
        self.yards as f64 / self.games as f64
    }
}

// higher-order function -- takes a predicate closure and applies it
fn filter_and_rank<F>(players: &[PlayerStat], predicate: F) -> Vec<&PlayerStat>
where
    F: Fn(&PlayerStat) -> bool,
{
    let mut filtered: Vec<&PlayerStat> = players.iter().filter(|p| predicate(p)).collect();
    filtered.sort_by(|a, b| b.yards.cmp(&a.yards));
    filtered
}

// returns a closure -- the closure captures the threshold by value (move)
fn min_yards_filter(min: u32) -> impl Fn(&PlayerStat) -> bool {
    move |p| p.yards >= min
}

// FnMut -- the closure mutates state it captured
fn running_total_printer(players: &[PlayerStat]) {
    let mut total = 0u32;
    let mut print_and_accumulate = |p: &PlayerStat| {
        total += p.yards;
        println!("  {} -- {} yds | running total: {total}", p.name, p.yards);
    };
    for p in players {
        print_and_accumulate(p);
    }
    println!("  season total: {total} yards");
}

fn main() {
    let players = vec![
        PlayerStat { name: "Corey Dillon",  position: "RB", yards: 1635, tds: 12, games: 15 },
        PlayerStat { name: "David Givens",  position: "WR", yards: 874,  tds: 3,  games: 15 },
        PlayerStat { name: "David Patten",  position: "WR", yards: 800,  tds: 4,  games: 16 },
        PlayerStat { name: "Daniel Graham", position: "TE", yards: 364,  tds: 7,  games: 16 },
        PlayerStat { name: "Deion Branch",  position: "WR", yards: 454,  tds: 4,  games: 13 },
        PlayerStat { name: "Kevin Faulk",   position: "RB", yards: 248,  tds: 0,  games: 16 },
        PlayerStat { name: "Troy Brown",    position: "WR", yards: 184,  tds: 4,  games: 16 },
    ];

    println!("2004 patriots -- closure-based stat aggregation\n");

    // closure as a variable -- captures nothing, pure function
    let td_efficiency = |p: &PlayerStat| -> f64 {
        if p.yards == 0 { return 0.0; }
        p.tds as f64 / (p.yards as f64 / 100.0)  // tds per 100 yards
    };

    println!("td efficiency (tds per 100 yards):");
    for p in &players {
        println!("  {}: {:.2}", p.name, td_efficiency(p));
    }

    // factory closure -- returns a reusable filter
    let high_volume = min_yards_filter(400);
    let contributors = filter_and_rank(&players, high_volume);
    println!("\nhigh-volume contributors (400+ yards):");
    for p in contributors {
        println!("  {} ({}) -- {} yds, {} td, {:.1} ypg", 
            p.name, p.position, p.yards, p.tds, p.yards_per_game());
    }

    // inline closure passed directly
    let td_scorers = filter_and_rank(&players, |p| p.tds > 0);
    println!("\ntd scorers ranked by total yards:");
    for p in td_scorers {
        println!("  {} -- {} td, {} yds", p.name, p.tds, p.yards);
    }

    // FnMut closure with captured mutable state
    println!("\nrunning totals (receiving + rushing yards):");
    running_total_printer(&players);

    // fold with a closure -- equivalent to reduce in python
    let total_tds: u32 = players.iter().fold(0, |acc, p| acc + p.tds);
    let avg_ypg: f64 = players.iter().map(|p| p.yards_per_game()).sum::<f64>() / players.len() as f64;
    println!("\nteam totals: {total_tds} tds | avg ypg across tracked players: {avg_ypg:.1}");
}
