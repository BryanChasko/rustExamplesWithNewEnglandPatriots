// regex/play_parser.rs
//
// parse play-by-play text from the 2004 patriots season.
// patterns like "Brady pass complete to Branch for 23 yards"
// get decomposed into structured play data.

use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

/// compiled regex patterns -- initialized once, reused across calls.
/// OnceLock is the modern replacement for lazy_static.
fn pass_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        // target is a single-word player name (last name only in play-by-play)
        // "for" is a keyword delimiter, never part of a player name
        Regex::new(
            r"(?P<passer>\w+) pass (?P<result>complete|incomplete)(?: to (?P<target>\w+))?(?: for (?P<yards>-?\d+) yards?)?"
        ).unwrap()
    })
}

fn rush_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?P<rusher>\w+[\w ]*\w+) rush (?:for )?(?P<yards>-?\d+) yards?").unwrap()
    })
}

fn sack_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?P<passer>\w+) sacked (?:by (?P<defender>\w+[\w ]*\w+) )?for (?P<yards>-?\d+) yards?").unwrap()
    })
}

fn td_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)touchdown").unwrap())
}

/// the parsed result of a single play
#[derive(Debug, Clone, PartialEq)]
pub enum PlayResult {
    PassComplete {
        passer: String,
        target: String,
        yards: i32,
        touchdown: bool,
    },
    PassIncomplete {
        passer: String,
        target: Option<String>,
    },
    Rush {
        rusher: String,
        yards: i32,
        touchdown: bool,
    },
    Sack {
        passer: String,
        defender: Option<String>,
        yards: i32,
    },
    Unknown(String),
}

/// parse a single play description into a PlayResult
pub fn parse_play(text: &str) -> PlayResult {
    let is_td = td_pattern().is_match(text);

    if let Some(caps) = pass_pattern().captures(text) {
        let passer = caps["passer"].to_string();
        let result = &caps["result"];

        if result == "complete" {
            let target = caps
                .name("target")
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let yards = caps
                .name("yards")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            return PlayResult::PassComplete {
                passer,
                target,
                yards,
                touchdown: is_td,
            };
        }

        let target = caps.name("target").map(|m| m.as_str().to_string());
        return PlayResult::PassIncomplete { passer, target };
    }

    if let Some(caps) = sack_pattern().captures(text) {
        let passer = caps["passer"].to_string();
        let defender = caps.name("defender").map(|m| m.as_str().to_string());
        let yards = caps["yards"].parse().unwrap_or(0);
        return PlayResult::Sack {
            passer,
            defender,
            yards,
        };
    }

    if let Some(caps) = rush_pattern().captures(text) {
        let rusher = caps["rusher"].to_string();
        let yards = caps["yards"].parse().unwrap_or(0);
        return PlayResult::Rush {
            rusher,
            yards,
            touchdown: is_td,
        };
    }

    PlayResult::Unknown(text.to_string())
}

/// box score accumulated from parsed plays
#[derive(Debug, Default, Clone)]
pub struct BoxScore {
    pub pass_attempts: u32,
    pub completions: u32,
    pub passing_yards: i32,
    pub passing_tds: u32,
    pub rush_attempts: u32,
    pub rushing_yards: i32,
    pub rushing_tds: u32,
    pub sacks_taken: u32,
    pub targets: HashMap<String, u32>,
    pub receiving_yards: HashMap<String, i32>,
}

impl BoxScore {
    /// build a box score from a slice of play descriptions
    pub fn from_plays(plays: &[&str]) -> Self {
        let mut score = Self::default();
        for play in plays {
            score.add_play(&parse_play(play));
        }
        score
    }

    /// accumulate a single parsed play into the box score
    pub fn add_play(&mut self, play: &PlayResult) {
        match play {
            PlayResult::PassComplete {
                target,
                yards,
                touchdown,
                ..
            } => {
                self.pass_attempts += 1;
                self.completions += 1;
                self.passing_yards += yards;
                if *touchdown {
                    self.passing_tds += 1;
                }
                *self.targets.entry(target.clone()).or_insert(0) += 1;
                *self.receiving_yards.entry(target.clone()).or_insert(0) += yards;
            }
            PlayResult::PassIncomplete { .. } => {
                self.pass_attempts += 1;
            }
            PlayResult::Rush {
                yards, touchdown, ..
            } => {
                self.rush_attempts += 1;
                self.rushing_yards += yards;
                if *touchdown {
                    self.rushing_tds += 1;
                }
            }
            PlayResult::Sack { yards, .. } => {
                self.sacks_taken += 1;
                self.passing_yards += yards; // sack yards are negative
            }
            PlayResult::Unknown(_) => {}
        }
    }

    /// passer completion percentage
    pub fn completion_pct(&self) -> f32 {
        if self.pass_attempts == 0 {
            return 0.0;
        }
        (self.completions as f32 / self.pass_attempts as f32) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_complete() {
        let play = parse_play("Brady pass complete to Branch for 23 yards");
        assert_eq!(
            play,
            PlayResult::PassComplete {
                passer: "Brady".to_string(),
                target: "Branch".to_string(),
                yards: 23,
                touchdown: false,
            }
        );
    }

    #[test]
    fn test_pass_complete_touchdown() {
        let play = parse_play("Brady pass complete to Givens for 5 yards TOUCHDOWN");
        assert_eq!(
            play,
            PlayResult::PassComplete {
                passer: "Brady".to_string(),
                target: "Givens".to_string(),
                yards: 5,
                touchdown: true,
            }
        );
    }

    #[test]
    fn test_pass_incomplete() {
        let play = parse_play("Brady pass incomplete to Branch");
        assert_eq!(
            play,
            PlayResult::PassIncomplete {
                passer: "Brady".to_string(),
                target: Some("Branch".to_string()),
            }
        );
    }

    #[test]
    fn test_rush() {
        let play = parse_play("Dillon rush for 12 yards");
        assert_eq!(
            play,
            PlayResult::Rush {
                rusher: "Dillon".to_string(),
                yards: 12,
                touchdown: false,
            }
        );
    }

    #[test]
    fn test_rush_touchdown() {
        let play = parse_play("Dillon rush for 2 yards TOUCHDOWN");
        assert_eq!(
            play,
            PlayResult::Rush {
                rusher: "Dillon".to_string(),
                yards: 2,
                touchdown: true,
            }
        );
    }

    #[test]
    fn test_sack() {
        let play = parse_play("Brady sacked by Freeney for -8 yards");
        assert_eq!(
            play,
            PlayResult::Sack {
                passer: "Brady".to_string(),
                defender: Some("Freeney".to_string()),
                yards: -8,
            }
        );
    }

    #[test]
    fn test_unknown_play() {
        let play = parse_play("timeout called by new england");
        assert!(matches!(play, PlayResult::Unknown(_)));
    }

    #[test]
    fn test_box_score_from_plays() {
        let plays = vec![
            "Brady pass complete to Branch for 23 yards",
            "Brady pass complete to Givens for 5 yards TOUCHDOWN",
            "Brady pass incomplete to Patten",
            "Dillon rush for 12 yards",
            "Dillon rush for 2 yards TOUCHDOWN",
            "Brady sacked by Freeney for -8 yards",
        ];
        let score = BoxScore::from_plays(&plays);
        assert_eq!(score.pass_attempts, 3);
        assert_eq!(score.completions, 2);
        assert_eq!(score.passing_yards, 23 + 5 - 8);
        assert_eq!(score.passing_tds, 1);
        assert_eq!(score.rush_attempts, 2);
        assert_eq!(score.rushing_yards, 14);
        assert_eq!(score.rushing_tds, 1);
        assert_eq!(score.sacks_taken, 1);
    }

    #[test]
    fn test_box_score_targets() {
        let plays = vec![
            "Brady pass complete to Branch for 10 yards",
            "Brady pass complete to Branch for 15 yards",
            "Brady pass complete to Givens for 8 yards",
        ];
        let score = BoxScore::from_plays(&plays);
        assert_eq!(score.targets["Branch"], 2);
        assert_eq!(score.targets["Givens"], 1);
        assert_eq!(score.receiving_yards["Branch"], 25);
    }

    #[test]
    fn test_completion_pct() {
        let plays = vec![
            "Brady pass complete to Branch for 10 yards",
            "Brady pass incomplete to Givens",
        ];
        let score = BoxScore::from_plays(&plays);
        assert!((score.completion_pct() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_box_score() {
        let score = BoxScore::default();
        assert_eq!(score.completion_pct(), 0.0);
    }

    #[test]
    fn test_negative_rush_yards() {
        let play = parse_play("Dillon rush for -3 yards");
        assert_eq!(
            play,
            PlayResult::Rush {
                rusher: "Dillon".to_string(),
                yards: -3,
                touchdown: false,
            }
        );
    }
}
