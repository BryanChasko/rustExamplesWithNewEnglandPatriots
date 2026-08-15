// closures -- stat aggregation and higher-order functions
//
// demonstrates: closure syntax, Fn/FnMut/FnOnce traits, capturing by ref + move,
//               higher-order functions, iterator chains (map/filter/fold),
//               returning closures from functions
// python mirror: libraries_2_statistics_2023_patriots_qb_shuffle.py
//
// 2004 patriots: build stat aggregators as first-class closures

/// position on the 2004 patriots roster
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    QB,
    RB,
    WR,
    TE,
    LB,
    CB,
    S,
    K,
    DE,
    DT,
}

impl std::str::FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "QB" => Ok(Self::QB),
            "RB" => Ok(Self::RB),
            "WR" => Ok(Self::WR),
            "TE" => Ok(Self::TE),
            "LB" => Ok(Self::LB),
            "CB" => Ok(Self::CB),
            "S" => Ok(Self::S),
            "K" => Ok(Self::K),
            "DE" => Ok(Self::DE),
            "DT" => Ok(Self::DT),
            other => Err(format!("unknown position: {other}")),
        }
    }
}

/// a player on the 2004 roster
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub number: u8,
    pub position: Position,
    pub years_with_patriots: u8,
}

/// per-game stat line for a player
#[derive(Debug, Clone)]
pub struct GameStat {
    pub name: String,
    pub week: u8,
    pub yards: u32,
    pub tds: u32,
    pub points_contributed: u32,
}

/// single game result from the 2004 season log
#[derive(Debug, Clone)]
pub struct GameResult {
    pub week: u8,
    pub opponent: String,
    pub location: String,
    pub pts_patriots: u32,
    pub pts_opponent: u32,
    pub win: bool,
}

/// higher-order function -- takes a scoring predicate, returns matching stats
///
/// the predicate decides what qualifies as a "top performance"
pub fn top_performers(stats: &[GameStat], predicate: impl Fn(&GameStat) -> bool) -> Vec<&GameStat> {
    stats.iter().filter(|s| predicate(s)).collect()
}

/// returns a closure that filters by minimum yards threshold
pub fn min_yards_filter(min: u32) -> impl Fn(&GameStat) -> bool {
    move |s| s.yards >= min
}

/// returns a closure that filters by minimum tds
pub fn min_tds_filter(min: u32) -> impl Fn(&GameStat) -> bool {
    move |s| s.tds >= min
}

/// FnMut closure factory -- returns a closure that tracks a running average
/// of yards per game. each call updates the internal state
pub fn yards_per_game_tracker() -> impl FnMut(u32) -> f64 {
    let mut total_yards: u64 = 0;
    let mut games_played: u64 = 0;
    move |yards: u32| {
        total_yards += yards as u64;
        games_played += 1;
        total_yards as f64 / games_played as f64
    }
}

/// season point accumulator -- captures environment via move
/// returns (games_processed, total_points)
pub fn season_point_accumulator(games: &[GameResult]) -> (usize, u32) {
    let mut total_points = 0u32;
    let mut games_processed = 0usize;

    let mut accumulate = |game: &GameResult| {
        total_points += game.pts_patriots;
        games_processed += 1;
    };

    for game in games {
        accumulate(game);
    }

    (games_processed, total_points)
}

/// team stats computed via map/filter/fold chain on the game log
#[derive(Debug)]
pub struct TeamSeasonSummary {
    pub total_points_for: u32,
    pub total_points_against: u32,
    pub wins: usize,
    pub losses: usize,
    pub avg_points_per_game: f64,
    pub avg_margin_of_victory: f64,
    pub home_wins: usize,
    pub away_wins: usize,
}

/// compute team season summary using iterator chains (map/filter/fold)
pub fn compute_team_stats(games: &[GameResult]) -> TeamSeasonSummary {
    let total_points_for = games.iter().map(|g| g.pts_patriots).sum::<u32>();
    let total_points_against = games.iter().map(|g| g.pts_opponent).sum::<u32>();

    let wins = games.iter().filter(|g| g.win).count();
    let losses = games.iter().filter(|g| !g.win).count();

    let game_count = games.len() as f64;
    let avg_points_per_game = total_points_for as f64 / game_count;

    // fold to compute total margin, then average
    let total_margin: i64 = games.iter().filter(|g| g.win).fold(0i64, |acc, g| {
        acc + (g.pts_patriots as i64 - g.pts_opponent as i64)
    });
    let avg_margin_of_victory = if wins > 0 {
        total_margin as f64 / wins as f64
    } else {
        0.0
    };

    let home_wins = games
        .iter()
        .filter(|g| g.win && g.location == "home")
        .count();
    let away_wins = games
        .iter()
        .filter(|g| g.win && (g.location == "away" || g.location == "neutral"))
        .count();

    TeamSeasonSummary {
        total_points_for,
        total_points_against,
        wins,
        losses,
        avg_points_per_game,
        avg_margin_of_victory,
        home_wins,
        away_wins,
    }
}

/// build the 2004 roster inline (from roster_2004.csv data)
pub fn patriots_2004_roster() -> Vec<Player> {
    vec![
        Player {
            name: "Tom Brady".into(),
            number: 12,
            position: Position::QB,
            years_with_patriots: 5,
        },
        Player {
            name: "Corey Dillon".into(),
            number: 28,
            position: Position::RB,
            years_with_patriots: 1,
        },
        Player {
            name: "Kevin Faulk".into(),
            number: 33,
            position: Position::RB,
            years_with_patriots: 7,
        },
        Player {
            name: "Patrick Pass".into(),
            number: 38,
            position: Position::RB,
            years_with_patriots: 4,
        },
        Player {
            name: "David Givens".into(),
            number: 87,
            position: Position::WR,
            years_with_patriots: 3,
        },
        Player {
            name: "Deion Branch".into(),
            number: 83,
            position: Position::WR,
            years_with_patriots: 3,
        },
        Player {
            name: "David Patten".into(),
            number: 86,
            position: Position::WR,
            years_with_patriots: 4,
        },
        Player {
            name: "Troy Brown".into(),
            number: 80,
            position: Position::WR,
            years_with_patriots: 10,
        },
        Player {
            name: "Daniel Graham".into(),
            number: 82,
            position: Position::TE,
            years_with_patriots: 3,
        },
        Player {
            name: "Christian Fauria".into(),
            number: 88,
            position: Position::TE,
            years_with_patriots: 2,
        },
        Player {
            name: "Mike Vrabel".into(),
            number: 50,
            position: Position::LB,
            years_with_patriots: 6,
        },
        Player {
            name: "Tedy Bruschi".into(),
            number: 54,
            position: Position::LB,
            years_with_patriots: 9,
        },
        Player {
            name: "Willie McGinest".into(),
            number: 55,
            position: Position::LB,
            years_with_patriots: 11,
        },
        Player {
            name: "Rodney Harrison".into(),
            number: 37,
            position: Position::S,
            years_with_patriots: 2,
        },
        Player {
            name: "Ty Law".into(),
            number: 24,
            position: Position::CB,
            years_with_patriots: 10,
        },
        Player {
            name: "Adam Vinatieri".into(),
            number: 4,
            position: Position::K,
            years_with_patriots: 9,
        },
        Player {
            name: "Richard Seymour".into(),
            number: 93,
            position: Position::DE,
            years_with_patriots: 4,
        },
        Player {
            name: "Vince Wilfork".into(),
            number: 75,
            position: Position::DT,
            years_with_patriots: 1,
        },
        Player {
            name: "Asante Samuel".into(),
            number: 22,
            position: Position::CB,
            years_with_patriots: 3,
        },
        Player {
            name: "Eugene Wilson".into(),
            number: 26,
            position: Position::S,
            years_with_patriots: 2,
        },
    ]
}

/// build the 2004 game log inline (from game_log_2004.csv data)
pub fn patriots_2004_game_log() -> Vec<GameResult> {
    vec![
        GameResult {
            week: 1,
            opponent: "Indianapolis Colts".into(),
            location: "home".into(),
            pts_patriots: 27,
            pts_opponent: 24,
            win: true,
        },
        GameResult {
            week: 2,
            opponent: "Arizona Cardinals".into(),
            location: "away".into(),
            pts_patriots: 23,
            pts_opponent: 12,
            win: true,
        },
        GameResult {
            week: 3,
            opponent: "Buffalo Bills".into(),
            location: "home".into(),
            pts_patriots: 31,
            pts_opponent: 17,
            win: true,
        },
        GameResult {
            week: 4,
            opponent: "Miami Dolphins".into(),
            location: "away".into(),
            pts_patriots: 24,
            pts_opponent: 10,
            win: true,
        },
        GameResult {
            week: 5,
            opponent: "Pittsburgh Steelers".into(),
            location: "away".into(),
            pts_patriots: 20,
            pts_opponent: 34,
            win: false,
        },
        GameResult {
            week: 6,
            opponent: "Seattle Seahawks".into(),
            location: "home".into(),
            pts_patriots: 30,
            pts_opponent: 20,
            win: true,
        },
        GameResult {
            week: 7,
            opponent: "New York Jets".into(),
            location: "home".into(),
            pts_patriots: 13,
            pts_opponent: 7,
            win: true,
        },
        GameResult {
            week: 9,
            opponent: "St. Louis Rams".into(),
            location: "home".into(),
            pts_patriots: 40,
            pts_opponent: 22,
            win: true,
        },
        GameResult {
            week: 10,
            opponent: "Kansas City Chiefs".into(),
            location: "away".into(),
            pts_patriots: 27,
            pts_opponent: 19,
            win: true,
        },
        GameResult {
            week: 11,
            opponent: "Buffalo Bills".into(),
            location: "away".into(),
            pts_patriots: 29,
            pts_opponent: 6,
            win: true,
        },
        GameResult {
            week: 12,
            opponent: "Baltimore Ravens".into(),
            location: "home".into(),
            pts_patriots: 24,
            pts_opponent: 3,
            win: true,
        },
        GameResult {
            week: 13,
            opponent: "Cleveland Browns".into(),
            location: "away".into(),
            pts_patriots: 42,
            pts_opponent: 15,
            win: true,
        },
        GameResult {
            week: 14,
            opponent: "Cincinnati Bengals".into(),
            location: "home".into(),
            pts_patriots: 35,
            pts_opponent: 28,
            win: true,
        },
        GameResult {
            week: 15,
            opponent: "Miami Dolphins".into(),
            location: "home".into(),
            pts_patriots: 28,
            pts_opponent: 29,
            win: false,
        },
        GameResult {
            week: 16,
            opponent: "New York Jets".into(),
            location: "away".into(),
            pts_patriots: 23,
            pts_opponent: 7,
            win: true,
        },
        GameResult {
            week: 17,
            opponent: "San Francisco 49ers".into(),
            location: "home".into(),
            pts_patriots: 21,
            pts_opponent: 7,
            win: true,
        },
        GameResult {
            week: 18,
            opponent: "Indianapolis Colts (Playoffs Div)".into(),
            location: "home".into(),
            pts_patriots: 20,
            pts_opponent: 3,
            win: true,
        },
        GameResult {
            week: 19,
            opponent: "Pittsburgh Steelers (Playoffs AFC)".into(),
            location: "away".into(),
            pts_patriots: 41,
            pts_opponent: 27,
            win: true,
        },
        GameResult {
            week: 20,
            opponent: "Philadelphia Eagles (Super Bowl XXXIX)".into(),
            location: "neutral".into(),
            pts_patriots: 24,
            pts_opponent: 21,
            win: true,
        },
    ]
}

/// sample game stats for demonstrating higher-order functions
pub fn sample_game_stats() -> Vec<GameStat> {
    vec![
        GameStat {
            name: "Tom Brady".into(),
            week: 1,
            yards: 335,
            tds: 3,
            points_contributed: 18,
        },
        GameStat {
            name: "Tom Brady".into(),
            week: 2,
            yards: 230,
            tds: 1,
            points_contributed: 6,
        },
        GameStat {
            name: "Tom Brady".into(),
            week: 3,
            yards: 298,
            tds: 2,
            points_contributed: 12,
        },
        GameStat {
            name: "Tom Brady".into(),
            week: 4,
            yards: 260,
            tds: 2,
            points_contributed: 12,
        },
        GameStat {
            name: "Corey Dillon".into(),
            week: 1,
            yards: 86,
            tds: 1,
            points_contributed: 6,
        },
        GameStat {
            name: "Corey Dillon".into(),
            week: 3,
            yards: 111,
            tds: 2,
            points_contributed: 12,
        },
        GameStat {
            name: "Corey Dillon".into(),
            week: 9,
            yards: 112,
            tds: 1,
            points_contributed: 6,
        },
        GameStat {
            name: "Corey Dillon".into(),
            week: 13,
            yards: 122,
            tds: 2,
            points_contributed: 12,
        },
        GameStat {
            name: "David Givens".into(),
            week: 1,
            yards: 105,
            tds: 1,
            points_contributed: 6,
        },
        GameStat {
            name: "David Givens".into(),
            week: 6,
            yards: 89,
            tds: 0,
            points_contributed: 0,
        },
        GameStat {
            name: "Deion Branch".into(),
            week: 9,
            yards: 116,
            tds: 1,
            points_contributed: 6,
        },
        GameStat {
            name: "Deion Branch".into(),
            week: 20,
            yards: 133,
            tds: 0,
            points_contributed: 0,
        },
        GameStat {
            name: "Adam Vinatieri".into(),
            week: 1,
            yards: 0,
            tds: 0,
            points_contributed: 9,
        },
        GameStat {
            name: "Adam Vinatieri".into(),
            week: 7,
            yards: 0,
            tds: 0,
            points_contributed: 13,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_roster_by_position() {
        let roster = patriots_2004_roster();
        let qbs: Vec<&Player> = roster
            .iter()
            .filter(|p| p.position == Position::QB)
            .collect();
        assert_eq!(qbs.len(), 1);
        assert_eq!(qbs[0].name, "Tom Brady");
    }

    #[test]
    fn top_performers_with_predicate() {
        let stats = sample_game_stats();
        let big_games = top_performers(&stats, |s| s.yards >= 100);
        assert!(big_games.len() >= 5);
        assert!(big_games.iter().all(|s| s.yards >= 100));
    }

    #[test]
    fn yards_tracker_running_average() {
        let mut tracker = yards_per_game_tracker();
        assert!((tracker(100) - 100.0).abs() < f64::EPSILON);
        assert!((tracker(200) - 150.0).abs() < f64::EPSILON);
        assert!((tracker(300) - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn season_accumulator_totals() {
        let games = patriots_2004_game_log();
        let (count, total) = season_point_accumulator(&games);
        assert_eq!(count, 19);
        assert!(total > 400); // 2004 pats scored 437 regular season + playoffs
    }

    #[test]
    fn team_stats_chain() {
        let games = patriots_2004_game_log();
        let summary = compute_team_stats(&games);
        assert_eq!(summary.wins, 17);
        assert_eq!(summary.losses, 2);
        assert!(summary.avg_points_per_game > 25.0);
    }
}
