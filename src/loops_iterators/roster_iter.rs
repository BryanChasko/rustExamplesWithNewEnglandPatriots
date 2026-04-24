// loops_iterators -- roster_iter
//
// demonstrates: for loops, Iterator trait, map/filter/fold/collect, method chaining
// python mirror: basic_loops_dictionaries_patriots_legends.py
//
// rust iterators are lazy -- nothing runs until consumed (collect, sum, count, etc.)

fn main() {
    // 2004 patriots receiving leaders -- (name, receptions, yards, tds)
    let receivers: Vec<(&str, u32, u32, u32)> = vec![
        ("David Givens", 56, 874, 3),
        ("Deion Branch", 35, 454, 4),
        ("David Patten", 44, 800, 4),
        ("Daniel Graham", 30, 364, 7),
        ("Corey Dillon", 15, 103, 0),
        ("Kevin Faulk", 26, 248, 0),
        ("Troy Brown", 17, 184, 4),
    ];

    println!("2004 patriots -- receiving leaders\n");

    // basic for loop
    for (name, rec, yds, tds) in &receivers {
        println!("  {name}: {rec} rec, {yds} yds, {tds} td");
    }

    // iterator chain: filter td scorers, sort by yds, show names
    println!("\ntouchdown scorers sorted by yards:");
    let mut td_scorers: Vec<_> = receivers.iter().filter(|(_, _, _, tds)| *tds > 0).collect();
    td_scorers.sort_by(|a, b| b.2.cmp(&a.2));
    for (name, rec, yds, tds) in &td_scorers {
        println!("  {name}: {yds} yds, {tds} td ({rec} rec)");
    }

    // fold: total receiving yards
    let total_yards: u32 = receivers.iter().map(|(_, _, yds, _)| yds).sum();
    let total_tds: u32 = receivers.iter().map(|(_, _, _, tds)| tds).sum();
    println!("\nteam receiving totals: {total_yards} yds, {total_tds} td");

    // map to ownership-independent strings (demonstrates map + collect)
    let names: Vec<String> = receivers
        .iter()
        .map(|(name, _, _, _)| name.to_uppercase())
        .collect();
    println!("\nroster (uppercase): {}", names.join(", "));
}
