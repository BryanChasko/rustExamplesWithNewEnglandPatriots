// functions -- brady_calculator
//
// demonstrates: function definitions, parameters, return types, ownership of args
// python mirror: basic_function_tom_brady_versus_mac_jones_calculator.py
//
// 2004 patriots: tom brady's regular season + super bowl stats

fn passer_rating(completions: f64, attempts: f64, yards: f64, tds: f64, ints: f64) -> f64 {
    // nfl passer rating formula
    let a = ((completions / attempts) - 0.3) * 5.0;
    let b = ((yards / attempts) - 3.0) * 0.25;
    let c = (tds / attempts) * 20.0;
    let d = 2.375 - (ints / attempts * 25.0);

    let clamp = |x: f64| x.max(0.0).min(2.375);
    (clamp(a) + clamp(b) + clamp(c) + clamp(d)) / 6.0 * 100.0
}

fn print_qb_season(name: &str, completions: f64, attempts: f64, yards: f64, tds: f64, ints: f64) {
    let rating = passer_rating(completions, attempts, yards, tds, ints);
    println!(
        "{name}: {completions}/{attempts}, {yards} yds, {tds} td, {ints} int -- rating: {rating:.1}"
    );
}

fn main() {
    println!("2004 new england patriots -- quarterback analysis\n");

    // brady 2004 regular season
    print_qb_season(
        "tom brady (2004 regular season)",
        288.0,
        474.0,
        3692.0,
        28.0,
        14.0,
    );

    // brady super bowl xxxix
    print_qb_season("tom brady (super bowl xxxix)", 23.0, 33.0, 236.0, 2.0, 0.0);

    // comp% and ypa for context
    let comp_pct = 288.0 / 474.0 * 100.0;
    let ypa = 3692.0 / 474.0;
    println!("\nregular season breakdown:");
    println!("  completion %: {comp_pct:.1}%");
    println!("  yards per attempt: {ypa:.2}");
    println!("  td:int ratio: {:.2}", 28.0 / 14.0);
}
