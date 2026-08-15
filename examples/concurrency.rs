// examples/concurrency.rs
//
// rust's ownership model makes data races a compile-time error.
// three patterns: channels, mutex, rayon -- all produce identical results.

use rust_examples_patriots::concurrency::parallel_stats::{GameStats, SeasonAggregator};
use std::time::Instant;

fn main() {
    println!("=== concurrency: 2004 patriots season aggregation ===\n");

    let games = vec![
        GameStats::new(1, "Colts", 27, 24, 335, 117),
        GameStats::new(2, "Cardinals", 23, 12, 213, 155),
        GameStats::new(3, "Bills", 31, 17, 298, 128),
        GameStats::new(4, "Dolphins", 24, 10, 225, 163),
        GameStats::new(5, "Seahawks", 30, 20, 281, 137),
        GameStats::new(6, "Jets", 13, 7, 192, 120),
        GameStats::new(7, "Steelers", 34, 20, 280, 175),
        GameStats::new(8, "Rams", 40, 22, 354, 152),
        GameStats::new(9, "Bills", 29, 6, 248, 168),
        GameStats::new(10, "Chiefs", 27, 19, 265, 141),
        GameStats::new(11, "Ravens", 24, 3, 218, 153),
        GameStats::new(12, "Browns", 42, 15, 320, 178),
        GameStats::new(13, "Bengals", 35, 28, 260, 175),
        GameStats::new(14, "Dolphins", 28, 29, 290, 130),
        GameStats::new(15, "49ers", 21, 7, 195, 162),
        GameStats::new(16, "Jets", 23, 7, 235, 148),
    ];

    let agg = SeasonAggregator::new(games);

    // sequential baseline
    let start = Instant::now();
    let seq = agg.aggregate_sequential();
    let seq_time = start.elapsed();

    println!("-- sequential --");
    println!("  record: {}-{}", seq.wins, seq.games - seq.wins);
    println!("  points for: {}", seq.total_points_for);
    println!("  points against: {}", seq.total_points_against);
    println!("  point differential: +{}", seq.point_differential());
    println!("  passing yards: {}", seq.total_passing_yards);
    println!("  rushing yards: {}", seq.total_rushing_yards);
    if let Some((week, rating)) = seq.best_offensive_game {
        println!("  best offensive game: week {week} (rating {rating:.1})");
    }
    println!("  time: {:?}", seq_time);

    // channels pattern
    let start = Instant::now();
    let chan = agg.aggregate_with_channels();
    let chan_time = start.elapsed();
    println!("\n-- channels (thread::spawn + mpsc) --");
    println!("  record: {}-{}", chan.wins, chan.games - chan.wins);
    println!("  points for: {}", chan.total_points_for);
    println!("  time: {:?}", chan_time);

    // mutex pattern
    let start = Instant::now();
    let mtx = agg.aggregate_with_mutex();
    let mtx_time = start.elapsed();
    println!("\n-- Arc<Mutex<T>> --");
    println!("  record: {}-{}", mtx.wins, mtx.games - mtx.wins);
    println!("  points for: {}", mtx.total_points_for);
    println!("  time: {:?}", mtx_time);

    // rayon par_iter
    let start = Instant::now();
    let ray = agg.aggregate_with_rayon();
    let ray_time = start.elapsed();
    println!("\n-- rayon par_iter --");
    println!("  record: {}-{}", ray.wins, ray.games - ray.wins);
    println!("  points for: {}", ray.total_points_for);
    println!("  time: {:?}", ray_time);

    // verify all methods agree
    println!("\n-- consistency check --");
    let all_agree = seq.total_points_for == chan.total_points_for
        && seq.total_points_for == mtx.total_points_for
        && seq.total_points_for == ray.total_points_for;
    println!("  all methods agree: {all_agree}");

    // parallel ratings
    let ratings = agg.ratings_parallel();
    println!("\n-- per-game offensive ratings (rayon par_iter map) --");
    for (week, rating) in &ratings {
        println!("  week {week:2}: {rating:.1}");
    }

    println!("\n=== key takeaway ===");
    println!("all three patterns produce identical results. rayon is idiomatic");
    println!("for data parallelism. channels shine for producer/consumer patterns.");
    println!("Arc<Mutex> is the escape hatch when shared mutable state is unavoidable.");
    println!("python's GIL prevents true thread parallelism for CPU-bound work --");
    println!("multiprocessing adds IPC overhead that rust avoids entirely.");
}
