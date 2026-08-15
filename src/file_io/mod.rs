// file_io — std::fs reading CSV, serde for JSON
//
// demonstrates:
// - csv crate for parsing roster data from CSV
// - serde + serde_json for serializing/deserializing game results
// - writing structured data to files
// - error handling with `?` and anyhow

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// a player on the 2004 patriots roster, parsed from CSV
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RosterEntry {
    pub player: String,
    pub position: String,
    pub number: u8,
    pub college: String,
}

/// a single game result from the 2004 season
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameResult {
    pub week: u8,
    pub opponent: String,
    pub patriots_score: u16,
    pub opponent_score: u16,
    pub home: bool,
}

/// season summary computed from game results
#[derive(Debug, Serialize)]
pub struct SeasonSummary {
    pub wins: u8,
    pub losses: u8,
    pub total_points_scored: u16,
    pub total_points_allowed: u16,
    pub point_differential: i32,
}

/// reads a CSV file and returns a vector of roster entries
///
/// uses the csv crate's typed deserialization via serde derive
pub fn read_roster_csv(path: &Path) -> Result<Vec<RosterEntry>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("failed to open CSV file: {}", path.display()))?;

    let mut roster = Vec::new();
    for result in reader.deserialize() {
        let entry: RosterEntry = result.with_context(|| "failed to deserialize CSV record")?;
        roster.push(entry);
    }

    Ok(roster)
}

/// reads game results from a JSON file using serde_json
pub fn read_season_json(path: &Path) -> Result<Vec<GameResult>> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file: {}", path.display()))?;

    let results: Vec<GameResult> =
        serde_json::from_str(&contents).with_context(|| "failed to parse game results JSON")?;

    Ok(results)
}

/// computes a season summary from game results
pub fn summarize_season(games: &[GameResult]) -> SeasonSummary {
    let mut wins = 0u8;
    let mut losses = 0u8;
    let mut total_scored = 0u16;
    let mut total_allowed = 0u16;

    for game in games {
        if game.patriots_score > game.opponent_score {
            wins += 1;
        } else {
            losses += 1;
        }
        total_scored += game.patriots_score;
        total_allowed += game.opponent_score;
    }

    SeasonSummary {
        wins,
        losses,
        total_points_scored: total_scored,
        total_points_allowed: total_allowed,
        point_differential: i32::from(total_scored) - i32::from(total_allowed),
    }
}

/// serializes game results to JSON and writes to a file
pub fn write_results_json(path: &Path, games: &[GameResult]) -> Result<()> {
    let json = serde_json::to_string_pretty(games)
        .with_context(|| "failed to serialize game results to JSON")?;

    fs::write(path, json)
        .with_context(|| format!("failed to write JSON to: {}", path.display()))?;

    Ok(())
}

/// writes a season summary to a JSON file
pub fn write_summary_json(path: &Path, summary: &SeasonSummary) -> Result<()> {
    let json = serde_json::to_string_pretty(summary)
        .with_context(|| "failed to serialize season summary")?;

    fs::write(path, json)
        .with_context(|| format!("failed to write summary to: {}", path.display()))?;

    Ok(())
}

/// filters roster by position
pub fn filter_by_position<'a>(roster: &'a [RosterEntry], position: &str) -> Vec<&'a RosterEntry> {
    roster
        .iter()
        .filter(|entry| entry.position == position)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_roster_csv() {
        // uses the actual data file committed to the repo
        let roster = read_roster_csv(Path::new("data/roster_2004.csv")).unwrap();
        assert!(roster.len() >= 20);
        assert_eq!(roster[0].player, "Tom Brady");
        assert_eq!(roster[0].number, 12);
        assert_eq!(roster[0].position, "QB");
    }

    #[test]
    fn test_game_result_roundtrip() {
        let games = vec![GameResult {
            week: 1,
            opponent: "Indianapolis Colts".to_string(),
            patriots_score: 27,
            opponent_score: 24,
            home: true,
        }];

        let json = serde_json::to_string(&games).unwrap();
        let parsed: Vec<GameResult> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed[0].week, 1);
        assert_eq!(parsed[0].patriots_score, 27);
    }

    #[test]
    fn test_summarize_season() {
        let games = vec![
            GameResult {
                week: 1,
                opponent: "Team A".to_string(),
                patriots_score: 30,
                opponent_score: 20,
                home: true,
            },
            GameResult {
                week: 2,
                opponent: "Team B".to_string(),
                patriots_score: 10,
                opponent_score: 17,
                home: false,
            },
        ];

        let summary = summarize_season(&games);
        assert_eq!(summary.wins, 1);
        assert_eq!(summary.losses, 1);
        assert_eq!(summary.total_points_scored, 40);
        assert_eq!(summary.total_points_allowed, 37);
        assert_eq!(summary.point_differential, 3);
    }

    #[test]
    fn test_filter_by_position() {
        let roster = vec![
            RosterEntry {
                player: "Tom Brady".to_string(),
                position: "QB".to_string(),
                number: 12,
                college: "Michigan".to_string(),
            },
            RosterEntry {
                player: "Corey Dillon".to_string(),
                position: "RB".to_string(),
                number: 28,
                college: "Cincinnati".to_string(),
            },
        ];

        let qbs = filter_by_position(&roster, "QB");
        assert_eq!(qbs.len(), 1);
        assert_eq!(qbs[0].player, "Tom Brady");
    }
}
