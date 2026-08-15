// integration test — cross-module pipeline with real data files
//
// integration tests live in tests/ (outside src/) and see the crate as an
// external consumer would — only pub items are accessible. this validates
// the full pipeline: parse → compute → serialize → deserialize roundtrip.

use rust_examples_patriots::file_io::{
    filter_by_position, read_roster_csv, read_season_json, summarize_season, write_results_json,
    write_summary_json, GameResult, RosterEntry,
};
use std::path::Path;

// ---------------------------------------------------------------------------
// full pipeline: CSV parse → filter → verify
// ---------------------------------------------------------------------------

#[test]
fn integration_roster_csv_parses_and_filters_correctly() {
    let roster = read_roster_csv(Path::new("data/roster_2004.csv"))
        .expect("data/roster_2004.csv must exist and parse cleanly");

    // verify total count
    assert_eq!(roster.len(), 20, "2004 roster has 20 players");

    // verify position distribution from real data
    let qbs = filter_by_position(&roster, "QB");
    let rbs = filter_by_position(&roster, "RB");
    let wrs = filter_by_position(&roster, "WR");
    let lbs = filter_by_position(&roster, "LB");

    assert_eq!(qbs.len(), 1, "one QB (Brady)");
    assert_eq!(rbs.len(), 2, "two RBs (Dillon, Faulk)");
    assert_eq!(wrs.len(), 4, "four WRs (Givens, Branch, Brown, Johnson)");
    assert_eq!(lbs.len(), 3, "three LBs (Bruschi, McGinest, Vrabel)");

    // verify Brady is first in the CSV
    assert_eq!(qbs[0].player, "Tom Brady");
    assert_eq!(qbs[0].number, 12);
    assert_eq!(qbs[0].college, "Michigan");
}

// ---------------------------------------------------------------------------
// full pipeline: JSON parse → summarize → verify known 2004 season stats
// ---------------------------------------------------------------------------

#[test]
fn integration_season_json_parses_and_summarizes() {
    let games = read_season_json(Path::new("data/season_2004.json"))
        .expect("data/season_2004.json must exist and parse cleanly");

    assert_eq!(games.len(), 16, "JSON contains 16 regular season games");

    let summary = summarize_season(&games);

    // the 2004 patriots dominated — verify known facts
    assert!(
        summary.wins > summary.losses,
        "championship team wins more than it loses"
    );
    assert!(
        summary.point_differential > 100,
        "2004 patriots had a large point differential: {}",
        summary.point_differential
    );

    // verify specific game from the data
    let week1 = games.iter().find(|g| g.week == 1).unwrap();
    assert_eq!(week1.opponent, "Indianapolis Colts");
    assert_eq!(week1.patriots_score, 27);
    assert_eq!(week1.opponent_score, 24);
    assert!(week1.home);

    // the sole loss in the dataset
    let losses: Vec<&GameResult> = games
        .iter()
        .filter(|g| g.patriots_score < g.opponent_score)
        .collect();
    assert_eq!(losses.len(), 1, "only 1 loss in the 16-game JSON");
    assert_eq!(losses[0].week, 14);
    assert_eq!(losses[0].opponent, "Miami Dolphins");
}

// ---------------------------------------------------------------------------
// serialize → write → read → deserialize roundtrip
// ---------------------------------------------------------------------------

#[test]
fn integration_json_write_read_roundtrip() {
    let games = read_season_json(Path::new("data/season_2004.json")).unwrap();
    let summary = summarize_season(&games);

    // write to temp files
    let tmp_dir = std::env::temp_dir();
    let games_path = tmp_dir.join("patriots_test_games_roundtrip.json");
    let summary_path = tmp_dir.join("patriots_test_summary_roundtrip.json");

    write_results_json(&games_path, &games).expect("write games JSON");
    write_summary_json(&summary_path, &summary).expect("write summary JSON");

    // read back and verify
    let games_roundtrip = read_season_json(&games_path).expect("re-read games JSON");
    assert_eq!(games_roundtrip.len(), games.len());

    // verify each game survived the roundtrip
    for (original, roundtrip) in games.iter().zip(games_roundtrip.iter()) {
        assert_eq!(original.week, roundtrip.week);
        assert_eq!(original.opponent, roundtrip.opponent);
        assert_eq!(original.patriots_score, roundtrip.patriots_score);
        assert_eq!(original.opponent_score, roundtrip.opponent_score);
        assert_eq!(original.home, roundtrip.home);
    }

    // verify summary file is valid JSON
    let summary_json = std::fs::read_to_string(&summary_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&summary_json).unwrap();
    assert_eq!(parsed["wins"], summary.wins);
    assert_eq!(parsed["losses"], summary.losses);

    // cleanup
    let _ = std::fs::remove_file(&games_path);
    let _ = std::fs::remove_file(&summary_path);
}

// ---------------------------------------------------------------------------
// cross-module: roster data drives stat computations
// ---------------------------------------------------------------------------

#[test]
fn integration_roster_and_season_cross_module() {
    // load both datasets
    let roster = read_roster_csv(Path::new("data/roster_2004.csv")).unwrap();
    let games = read_season_json(Path::new("data/season_2004.json")).unwrap();

    // verify we can combine data from both sources
    let summary = summarize_season(&games);
    let roster_size = roster.len();

    // a roster that produced this dominant season
    assert!(
        roster_size >= 20,
        "championship roster had at least 20 key players"
    );
    assert!(
        summary.wins >= 14,
        "championship season had at least 14 wins from JSON data"
    );

    // every position group is represented
    let positions: Vec<&str> = roster.iter().map(|p| p.position.as_str()).collect();
    assert!(positions.contains(&"QB"));
    assert!(positions.contains(&"RB"));
    assert!(positions.contains(&"WR"));
    assert!(positions.contains(&"TE"));
    assert!(positions.contains(&"LB"));
    assert!(positions.contains(&"CB"));
    assert!(positions.contains(&"S"));
    assert!(positions.contains(&"K"));
}

// ---------------------------------------------------------------------------
// serde roundtrip on individual entries
// ---------------------------------------------------------------------------

#[test]
fn integration_roster_entry_serde_roundtrip() {
    let roster = read_roster_csv(Path::new("data/roster_2004.csv")).unwrap();

    // serialize every entry and deserialize back
    for entry in &roster {
        let json = serde_json::to_string(entry).unwrap();
        let roundtrip: RosterEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(roundtrip.player, entry.player);
        assert_eq!(roundtrip.position, entry.position);
        assert_eq!(roundtrip.number, entry.number);
        assert_eq!(roundtrip.college, entry.college);
    }
}

// ---------------------------------------------------------------------------
// error path integration: verify graceful failures
// ---------------------------------------------------------------------------

#[test]
fn integration_missing_csv_returns_descriptive_error() {
    let result = read_roster_csv(Path::new("data/does_not_exist.csv"));
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = format!("{err:#}");
    // anyhow chains context — verify our message is in there
    assert!(
        msg.contains("does_not_exist.csv"),
        "error should name the missing file: {msg}"
    );
}

#[test]
fn integration_malformed_json_returns_descriptive_error() {
    let tmp_dir = std::env::temp_dir();
    let bad_path = tmp_dir.join("patriots_bad_integration.json");
    std::fs::write(&bad_path, "[{\"week\": \"not_a_number\"}]").unwrap();

    let result = read_season_json(&bad_path);
    assert!(result.is_err());

    let _ = std::fs::remove_file(&bad_path);
}
