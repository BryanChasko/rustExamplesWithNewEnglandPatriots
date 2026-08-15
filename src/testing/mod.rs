// testing — #[test], assert!, integration tests with real data
//
// demonstrates:
// - unit tests with #[test], assert!, assert_eq!, assert_ne!
// - #[should_panic] for expected failures
// - #[ignore] for slow tests
// - Result-returning tests for error propagation
// - test helpers/fixtures with common setup functions
// - testing with real 2004 patriots data
//
// run all tests:       cargo test
// include ignored:     cargo test -- --include-ignored
// run just this module: cargo test testing

#[cfg(test)]
mod tests {
    use crate::file_io::{
        filter_by_position, read_roster_csv, read_season_json, summarize_season, GameResult,
        RosterEntry,
    };
    use std::path::Path;

    // -----------------------------------------------------------------------
    // test helpers / fixtures
    // -----------------------------------------------------------------------

    /// builds a sample roster for unit tests without touching the filesystem
    fn fixture_roster() -> Vec<RosterEntry> {
        vec![
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
            RosterEntry {
                player: "David Givens".to_string(),
                position: "WR".to_string(),
                number: 87,
                college: "Notre Dame".to_string(),
            },
            RosterEntry {
                player: "Deion Branch".to_string(),
                position: "WR".to_string(),
                number: 83,
                college: "Louisville".to_string(),
            },
            RosterEntry {
                player: "Daniel Graham".to_string(),
                position: "TE".to_string(),
                number: 82,
                college: "Colorado".to_string(),
            },
            RosterEntry {
                player: "Tedy Bruschi".to_string(),
                position: "LB".to_string(),
                number: 54,
                college: "Arizona".to_string(),
            },
            RosterEntry {
                player: "Adam Vinatieri".to_string(),
                position: "K".to_string(),
                number: 4,
                college: "South Dakota State".to_string(),
            },
        ]
    }

    /// builds a sample season schedule (subset) for computation tests
    fn fixture_games() -> Vec<GameResult> {
        vec![
            GameResult {
                week: 1,
                opponent: "Indianapolis Colts".to_string(),
                patriots_score: 27,
                opponent_score: 24,
                home: true,
            },
            GameResult {
                week: 2,
                opponent: "Arizona Cardinals".to_string(),
                patriots_score: 23,
                opponent_score: 12,
                home: false,
            },
            GameResult {
                week: 14,
                opponent: "Miami Dolphins".to_string(),
                patriots_score: 28,
                opponent_score: 29,
                home: true,
            },
        ]
    }

    // -----------------------------------------------------------------------
    // unit tests — assert!, assert_eq!, assert_ne!
    // -----------------------------------------------------------------------

    #[test]
    fn test_roster_fixture_has_players() {
        let roster = fixture_roster();
        // assert! checks a boolean condition
        assert!(!roster.is_empty(), "fixture roster must not be empty");
        assert!(roster.len() >= 7);
    }

    #[test]
    fn test_brady_is_number_12() {
        let roster = fixture_roster();
        let brady = roster.iter().find(|p| p.player == "Tom Brady").unwrap();
        // assert_eq! checks equality with helpful error messages
        assert_eq!(brady.number, 12);
        assert_eq!(brady.position, "QB");
        assert_eq!(brady.college, "Michigan");
    }

    #[test]
    fn test_dillon_is_not_a_quarterback() {
        let roster = fixture_roster();
        let dillon = roster.iter().find(|p| p.player == "Corey Dillon").unwrap();
        // assert_ne! confirms values differ
        assert_ne!(dillon.position, "QB");
        assert_ne!(dillon.number, 12);
    }

    // -- roster count verification --

    #[test]
    fn test_fixture_position_counts() {
        let roster = fixture_roster();
        let wr_count = filter_by_position(&roster, "WR").len();
        let qb_count = filter_by_position(&roster, "QB").len();
        let lb_count = filter_by_position(&roster, "LB").len();

        assert_eq!(wr_count, 2, "fixture has 2 wide receivers");
        assert_eq!(qb_count, 1, "fixture has 1 quarterback");
        assert_eq!(lb_count, 1, "fixture has 1 linebacker");
    }

    #[test]
    fn test_full_roster_from_csv_has_20_players() {
        let roster = read_roster_csv(Path::new("data/roster_2004.csv")).unwrap();
        assert_eq!(roster.len(), 20, "2004 roster CSV has 20 entries");
    }

    // -- game results and stats calculations --

    #[test]
    fn test_season_summary_wins_and_losses() {
        let games = fixture_games();
        let summary = summarize_season(&games);

        // 2 wins (weeks 1, 2), 1 loss (week 14)
        assert_eq!(summary.wins, 2);
        assert_eq!(summary.losses, 1);
    }

    #[test]
    fn test_season_summary_point_totals() {
        let games = fixture_games();
        let summary = summarize_season(&games);

        // 27 + 23 + 28 = 78 scored
        assert_eq!(summary.total_points_scored, 78);
        // 24 + 12 + 29 = 65 allowed
        assert_eq!(summary.total_points_allowed, 65);
        // differential = 78 - 65 = 13
        assert_eq!(summary.point_differential, 13);
    }

    #[test]
    fn test_full_2004_season_record() {
        // the 2004 patriots went 14-2 in the regular season (data has 16 games)
        let games = read_season_json(Path::new("data/season_2004.json")).unwrap();
        let summary = summarize_season(&games);

        assert_eq!(summary.wins, 15, "patriots won 15 of the 16 games in JSON");
        assert_eq!(summary.losses, 1, "patriots lost 1 game in JSON (week 14)");
    }

    #[test]
    fn test_point_differential_positive_for_dominant_season() {
        let games = read_season_json(Path::new("data/season_2004.json")).unwrap();
        let summary = summarize_season(&games);

        assert!(
            summary.point_differential > 0,
            "championship team should have a positive point differential"
        );
        // they scored significantly more than they allowed
        assert!(
            summary.total_points_scored > summary.total_points_allowed,
            "scored {} vs allowed {}",
            summary.total_points_scored,
            summary.total_points_allowed
        );
    }

    // -----------------------------------------------------------------------
    // #[should_panic] for invalid data handling
    // -----------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panics_on_invalid_roster_index() {
        let roster = fixture_roster();
        // accessing beyond the roster length panics
        let _player = &roster[100];
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_panics_on_missing_player_unwrap() {
        let roster = fixture_roster();
        // unwrapping a find that matches nobody panics
        let _ghost = roster.iter().find(|p| p.player == "Randy Moss").unwrap();
    }

    // -----------------------------------------------------------------------
    // #[ignore] for slow tests
    // -----------------------------------------------------------------------

    #[test]
    #[ignore = "run with --include-ignored; validates all CSV files in datasets/"]
    fn test_all_dataset_csvs_parse_cleanly() {
        // walks every CSV in datasets/ and confirms they parse without error
        // slow because it does real I/O across all available files
        let dataset_dir = Path::new("datasets");
        if !dataset_dir.exists() {
            return;
        }

        for entry in std::fs::read_dir(dataset_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "csv") {
                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .from_path(&path)
                    .unwrap_or_else(|_| panic!("failed to open {}", path.display()));

                let record_count = reader.records().count();
                assert!(
                    record_count > 0,
                    "{} should have at least one record",
                    path.display()
                );
            }
        }
    }

    #[test]
    #[ignore = "run with --include-ignored; full serialization roundtrip stress test"]
    fn test_large_roundtrip_serialization() {
        // stress test: serialize and deserialize the full season many times
        let games = read_season_json(Path::new("data/season_2004.json")).unwrap();

        for _ in 0..1000 {
            let json = serde_json::to_string(&games).unwrap();
            let parsed: Vec<GameResult> = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed.len(), games.len());
        }
    }

    // -----------------------------------------------------------------------
    // Result-returning tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_read_roster_csv_returns_ok() -> Result<(), Box<dyn std::error::Error>> {
        let roster = read_roster_csv(Path::new("data/roster_2004.csv"))?;
        assert!(!roster.is_empty());
        Ok(())
    }

    #[test]
    fn test_read_season_json_returns_ok() -> Result<(), Box<dyn std::error::Error>> {
        let games = read_season_json(Path::new("data/season_2004.json"))?;
        assert!(!games.is_empty());
        Ok(())
    }

    #[test]
    fn test_missing_file_returns_err() {
        let result = read_roster_csv(Path::new("data/nonexistent.csv"));
        assert!(result.is_err(), "reading a missing file should return Err");

        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("nonexistent.csv"),
            "error should mention the filename, got: {err_msg}"
        );
    }

    #[test]
    fn test_invalid_json_returns_err() {
        // create a temp file with bad JSON to verify error path
        let tmp_dir = std::env::temp_dir();
        let bad_json_path = tmp_dir.join("bad_patriots_data.json");
        std::fs::write(&bad_json_path, "{ this is not valid json !!!").unwrap();

        let result = read_season_json(&bad_json_path);
        assert!(result.is_err());

        // cleanup
        let _ = std::fs::remove_file(&bad_json_path);
    }

    // -----------------------------------------------------------------------
    // serde roundtrip correctness
    // -----------------------------------------------------------------------

    #[test]
    fn test_game_result_serde_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let original = GameResult {
            week: 7,
            opponent: "Pittsburgh Steelers".to_string(),
            patriots_score: 34,
            opponent_score: 20,
            home: false,
        };

        let json = serde_json::to_string(&original)?;
        let deserialized: GameResult = serde_json::from_str(&json)?;

        assert_eq!(deserialized.week, original.week);
        assert_eq!(deserialized.opponent, original.opponent);
        assert_eq!(deserialized.patriots_score, original.patriots_score);
        assert_eq!(deserialized.opponent_score, original.opponent_score);
        assert_eq!(deserialized.home, original.home);
        Ok(())
    }

    #[test]
    fn test_roster_entry_serde_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let original = RosterEntry {
            player: "Rodney Harrison".to_string(),
            position: "S".to_string(),
            number: 37,
            college: "Western Illinois".to_string(),
        };

        let json = serde_json::to_string(&original)?;
        let deserialized: RosterEntry = serde_json::from_str(&json)?;

        assert_eq!(deserialized.player, original.player);
        assert_eq!(deserialized.position, original.position);
        assert_eq!(deserialized.number, original.number);
        assert_eq!(deserialized.college, original.college);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // filter logic
    // -----------------------------------------------------------------------

    #[test]
    fn test_filter_returns_empty_for_unknown_position() {
        let roster = fixture_roster();
        let punters = filter_by_position(&roster, "P");
        assert!(punters.is_empty(), "no punter in fixture data");
    }

    #[test]
    fn test_filter_returns_correct_subset() {
        let roster = fixture_roster();
        let wrs = filter_by_position(&roster, "WR");

        assert_eq!(wrs.len(), 2);
        assert!(wrs.iter().all(|p| p.position == "WR"));
        // confirm specific players
        let names: Vec<&str> = wrs.iter().map(|p| p.player.as_str()).collect();
        assert!(names.contains(&"David Givens"));
        assert!(names.contains(&"Deion Branch"));
    }

    // -----------------------------------------------------------------------
    // edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_summarize_empty_season() {
        let empty: Vec<GameResult> = vec![];
        let summary = summarize_season(&empty);

        assert_eq!(summary.wins, 0);
        assert_eq!(summary.losses, 0);
        assert_eq!(summary.total_points_scored, 0);
        assert_eq!(summary.total_points_allowed, 0);
        assert_eq!(summary.point_differential, 0);
    }
}
