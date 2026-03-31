// variables -- binding
//
// demonstrates: let bindings, mut, shadowing, type inference, constants, statics
// python mirror: beginner_python_concepts/ files
//
// 2004 patriots: player numbers, yard totals, snap counts

const SUPER_BOWL_XXXIX_SCORE: (u8, u8) = (24, 21); // (patriots, eagles)
const REGULAR_SEASON_WINS: u8 = 14;
static SEASON: &str = "2004";

fn main() {
    println!("2004 new england patriots -- variable binding demo\n");

    // immutable binding -- compiler enforces this
    let quarterback = "Tom Brady";
    let jersey_number: u8 = 12;
    println!("{quarterback} #{jersey_number}");

    // type inference -- rust knows these are u32 without annotation
    let passing_yards = 3692;
    let touchdowns = 28;
    let interceptions = 14;
    println!("regular season: {passing_yards} yds, {touchdowns} td, {interceptions} int");

    // mutable binding -- required to change a value
    let mut rushing_yards = 0u32;
    rushing_yards += 417;  // corey dillon through first 4 weeks (example)
    rushing_yards += 1218; // remaining games
    println!("corey dillon rushing yards: {rushing_yards}");

    // shadowing -- rebind a name to a new value or type
    // common pattern: parse a string into a number
    let yards_per_game = "230.8";
    let yards_per_game: f64 = yards_per_game.parse().expect("invalid number");
    println!("passing yards per game: {yards_per_game:.1}");

    // shadow to derive new value from old
    let record = REGULAR_SEASON_WINS;
    let record = format!("{record}-2"); // record is now a String, not u8
    println!("{SEASON} record: {record}");

    // constants -- evaluated at compile time, always immutable
    let (our_score, their_score) = SUPER_BOWL_XXXIX_SCORE;
    println!("super bowl xxxix: patriots {our_score}, eagles {their_score}");
    println!("margin of victory: {} points", our_score - their_score);

    // tuple destructuring
    let (completions, attempts) = (288u32, 474u32);
    let completion_pct = completions as f64 / attempts as f64 * 100.0;
    println!("completion %: {completion_pct:.1}%");
}
