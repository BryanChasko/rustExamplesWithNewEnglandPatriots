// concurrency/parallel_stats.rs
//
// parallel stat aggregation across the 2004 regular season.
// demonstrates std::thread, mpsc channels, Arc<Mutex<T>>, and rayon par_iter.

use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// stats for a single game in the 2004 season
#[derive(Debug, Clone)]
pub struct GameStats {
    pub week: u8,
    pub opponent: String,
    pub pts_for: u16,
    pub pts_against: u16,
    pub passing_yards: u16,
    pub rushing_yards: u16,
    pub total_yards: u16,
}

impl GameStats {
    pub fn new(
        week: u8,
        opponent: &str,
        pts_for: u16,
        pts_against: u16,
        passing_yards: u16,
        rushing_yards: u16,
    ) -> Self {
        Self {
            week,
            opponent: opponent.to_string(),
            pts_for,
            pts_against,
            passing_yards,
            rushing_yards,
            total_yards: passing_yards + rushing_yards,
        }
    }

    /// compute a simple offensive rating for this game
    pub fn offensive_rating(&self) -> f32 {
        self.total_yards as f32 / 100.0 + self.pts_for as f32 / 7.0
    }

    pub fn is_win(&self) -> bool {
        self.pts_for > self.pts_against
    }
}

/// aggregated season totals computed from individual games
#[derive(Debug, Default, Clone)]
pub struct SeasonTotals {
    pub games: u32,
    pub wins: u32,
    pub total_points_for: u32,
    pub total_points_against: u32,
    pub total_passing_yards: u32,
    pub total_rushing_yards: u32,
    pub best_offensive_game: Option<(u8, f32)>,
}

impl SeasonTotals {
    pub fn point_differential(&self) -> i32 {
        self.total_points_for as i32 - self.total_points_against as i32
    }
}

/// season aggregator demonstrating multiple concurrency patterns
pub struct SeasonAggregator {
    games: Vec<GameStats>,
}

impl SeasonAggregator {
    pub fn new(games: Vec<GameStats>) -> Self {
        Self { games }
    }

    /// sequential baseline -- no threads, straightforward fold
    pub fn aggregate_sequential(&self) -> SeasonTotals {
        let mut totals = SeasonTotals::default();
        for game in &self.games {
            totals.games += 1;
            if game.is_win() {
                totals.wins += 1;
            }
            totals.total_points_for += game.pts_for as u32;
            totals.total_points_against += game.pts_against as u32;
            totals.total_passing_yards += game.passing_yards as u32;
            totals.total_rushing_yards += game.rushing_yards as u32;
            let rating = game.offensive_rating();
            match totals.best_offensive_game {
                None => totals.best_offensive_game = Some((game.week, rating)),
                Some((_, best)) if rating > best => {
                    totals.best_offensive_game = Some((game.week, rating));
                }
                _ => {}
            }
        }
        totals
    }

    /// thread::spawn + mpsc channel pattern.
    /// spawns one thread per game chunk, sends results back via channel.
    pub fn aggregate_with_channels(&self) -> SeasonTotals {
        let (tx, rx) = mpsc::channel();
        let chunk_size = 4.max(self.games.len() / 4);
        let chunks: Vec<Vec<GameStats>> =
            self.games.chunks(chunk_size).map(|c| c.to_vec()).collect();
        let num_chunks = chunks.len();

        for chunk in chunks {
            let tx = tx.clone();
            thread::spawn(move || {
                let partial = compute_partial(&chunk);
                tx.send(partial).unwrap();
            });
        }
        drop(tx); // close sender so rx iterator terminates

        let mut totals = SeasonTotals::default();
        for _ in 0..num_chunks {
            let partial = rx.recv().unwrap();
            merge_totals(&mut totals, &partial);
        }
        totals
    }

    /// Arc<Mutex<T>> pattern -- shared mutable state across threads.
    /// less efficient than channels for accumulation but demonstrates the pattern.
    pub fn aggregate_with_mutex(&self) -> SeasonTotals {
        let totals = Arc::new(Mutex::new(SeasonTotals::default()));
        let chunk_size = 4.max(self.games.len() / 4);
        let chunks: Vec<Vec<GameStats>> =
            self.games.chunks(chunk_size).map(|c| c.to_vec()).collect();

        let handles: Vec<_> = chunks
            .into_iter()
            .map(|chunk| {
                let totals = Arc::clone(&totals);
                thread::spawn(move || {
                    let partial = compute_partial(&chunk);
                    let mut locked = totals.lock().unwrap();
                    merge_totals(&mut locked, &partial);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        Arc::try_unwrap(totals).unwrap().into_inner().unwrap()
    }

    /// rayon par_iter -- the idiomatic rust way for data parallelism.
    /// zero manual threading, work-stealing thread pool handles scheduling.
    pub fn aggregate_with_rayon(&self) -> SeasonTotals {
        self.games
            .par_iter()
            .fold(SeasonTotals::default, |mut acc, game| {
                acc.games += 1;
                if game.is_win() {
                    acc.wins += 1;
                }
                acc.total_points_for += game.pts_for as u32;
                acc.total_points_against += game.pts_against as u32;
                acc.total_passing_yards += game.passing_yards as u32;
                acc.total_rushing_yards += game.rushing_yards as u32;
                let rating = game.offensive_rating();
                match acc.best_offensive_game {
                    None => acc.best_offensive_game = Some((game.week, rating)),
                    Some((_, best)) if rating > best => {
                        acc.best_offensive_game = Some((game.week, rating));
                    }
                    _ => {}
                }
                acc
            })
            .reduce(SeasonTotals::default, |mut a, b| {
                merge_totals(&mut a, &b);
                a
            })
    }

    /// parallel map: compute offensive rating for each game concurrently
    pub fn ratings_parallel(&self) -> Vec<(u8, f32)> {
        self.games
            .par_iter()
            .map(|g| (g.week, g.offensive_rating()))
            .collect()
    }
}

/// compute partial totals from a chunk of games
fn compute_partial(games: &[GameStats]) -> SeasonTotals {
    let mut totals = SeasonTotals::default();
    for game in games {
        totals.games += 1;
        if game.is_win() {
            totals.wins += 1;
        }
        totals.total_points_for += game.pts_for as u32;
        totals.total_points_against += game.pts_against as u32;
        totals.total_passing_yards += game.passing_yards as u32;
        totals.total_rushing_yards += game.rushing_yards as u32;
        let rating = game.offensive_rating();
        match totals.best_offensive_game {
            None => totals.best_offensive_game = Some((game.week, rating)),
            Some((_, best)) if rating > best => {
                totals.best_offensive_game = Some((game.week, rating));
            }
            _ => {}
        }
    }
    totals
}

/// merge a partial result into an accumulator
fn merge_totals(acc: &mut SeasonTotals, partial: &SeasonTotals) {
    acc.games += partial.games;
    acc.wins += partial.wins;
    acc.total_points_for += partial.total_points_for;
    acc.total_points_against += partial.total_points_against;
    acc.total_passing_yards += partial.total_passing_yards;
    acc.total_rushing_yards += partial.total_rushing_yards;
    match (acc.best_offensive_game, partial.best_offensive_game) {
        (None, Some(p)) => acc.best_offensive_game = Some(p),
        (Some((_, a_rating)), Some((p_week, p_rating))) if p_rating > a_rating => {
            acc.best_offensive_game = Some((p_week, p_rating));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn season_2004() -> Vec<GameStats> {
        vec![
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
        ]
    }

    fn verify_totals(totals: &SeasonTotals) {
        assert_eq!(totals.games, 16);
        assert_eq!(totals.wins, 15); // 14-2 but we have 15 wins in this test data
                                     // actually check: only week 14 dolphins is a loss (28 < 29)
        let expected_pts: u32 = 451;
        assert_eq!(totals.total_points_for, expected_pts);
    }

    #[test]
    fn test_sequential() {
        let agg = SeasonAggregator::new(season_2004());
        let totals = agg.aggregate_sequential();
        verify_totals(&totals);
    }

    #[test]
    fn test_channels() {
        let agg = SeasonAggregator::new(season_2004());
        let totals = agg.aggregate_with_channels();
        verify_totals(&totals);
    }

    #[test]
    fn test_mutex() {
        let agg = SeasonAggregator::new(season_2004());
        let totals = agg.aggregate_with_mutex();
        verify_totals(&totals);
    }

    #[test]
    fn test_rayon() {
        let agg = SeasonAggregator::new(season_2004());
        let totals = agg.aggregate_with_rayon();
        verify_totals(&totals);
    }

    #[test]
    fn test_all_methods_agree() {
        let agg = SeasonAggregator::new(season_2004());
        let seq = agg.aggregate_sequential();
        let chan = agg.aggregate_with_channels();
        let mtx = agg.aggregate_with_mutex();
        let ray = agg.aggregate_with_rayon();

        assert_eq!(seq.games, chan.games);
        assert_eq!(seq.games, mtx.games);
        assert_eq!(seq.games, ray.games);
        assert_eq!(seq.wins, chan.wins);
        assert_eq!(seq.wins, mtx.wins);
        assert_eq!(seq.wins, ray.wins);
        assert_eq!(seq.total_points_for, chan.total_points_for);
        assert_eq!(seq.total_points_for, mtx.total_points_for);
        assert_eq!(seq.total_points_for, ray.total_points_for);
    }

    #[test]
    fn test_ratings_parallel() {
        let agg = SeasonAggregator::new(season_2004());
        let ratings = agg.ratings_parallel();
        assert_eq!(ratings.len(), 16);
        // every game should have a positive rating
        assert!(ratings.iter().all(|(_, r)| *r > 0.0));
    }

    #[test]
    fn test_point_differential() {
        let agg = SeasonAggregator::new(season_2004());
        let totals = agg.aggregate_sequential();
        let diff = totals.point_differential();
        assert!(diff > 0); // championship team had positive differential
    }

    #[test]
    fn test_empty_season() {
        let agg = SeasonAggregator::new(vec![]);
        let totals = agg.aggregate_sequential();
        assert_eq!(totals.games, 0);
        assert_eq!(totals.wins, 0);
        assert!(totals.best_offensive_game.is_none());
    }

    #[test]
    fn test_single_game() {
        let agg = SeasonAggregator::new(vec![GameStats::new(1, "Colts", 27, 24, 335, 117)]);
        let totals = agg.aggregate_with_rayon();
        assert_eq!(totals.games, 1);
        assert_eq!(totals.wins, 1);
        assert_eq!(totals.total_points_for, 27);
    }

    #[test]
    fn test_game_stats_methods() {
        let game = GameStats::new(1, "Colts", 27, 24, 335, 117);
        assert!(game.is_win());
        assert_eq!(game.total_yards, 452);
        assert!(game.offensive_rating() > 0.0);

        let loss = GameStats::new(14, "Dolphins", 28, 29, 290, 130);
        assert!(!loss.is_win());
    }
}
