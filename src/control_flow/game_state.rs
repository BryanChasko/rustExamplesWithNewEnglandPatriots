// control_flow -- game_state
//
// demonstrates: if/else, match on integers + enums + tuples, if let, while let, match guards
// python mirror: basic_match_name_input_NFL_legends.py
//
// 2004 patriots: game state decisions as type-safe match expressions

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
    Overtime,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Win(u8, u8), // (pts_patriots, pts_opponent)
    Loss(u8, u8),
    Tie(u8, u8),
    Bye,
}

#[derive(Debug, Clone)]
pub struct GameLog {
    pub week: u8,
    pub opponent: &'static str,
    pub result: GameResult,
}

#[derive(Debug)]
pub struct GameState {
    pub quarter: Quarter,
    pub score_diff: i8,     // patriots - opponent
    pub field_position: u8, // yards from own end zone (1-99)
    pub down: u8,
    pub yards_to_go: u8,
}

impl GameState {
    pub fn field_zone(&self) -> &'static str {
        match self.field_position {
            1..=25 => "own territory -- deep",
            26..=49 => "own territory",
            50 => "midfield",
            51..=74 => "opponent territory",
            75..=99 => "red zone",
            _ => "invalid",
        }
    }

    pub fn urgency(&self) -> &'static str {
        match (self.quarter, self.score_diff) {
            (Quarter::Fourth, diff) if diff < -8 => "two-score deficit -- need haste",
            (Quarter::Fourth, diff) if diff < 0 => "one-score game -- every drive counts",
            (Quarter::Fourth, diff) if diff > 10 => "comfortable lead -- run clock",
            (Quarter::Overtime, _) => "sudden death",
            _ => "standard game management",
        }
    }

    pub fn recommend_call(&self) -> String {
        // if let -- only act when the option matches
        let hurry_up_eligible: Option<&str> =
            if self.quarter == Quarter::Fourth && self.score_diff < 0 {
                Some("hurry-up offense")
            } else {
                None
            };

        if let Some(mode) = hurry_up_eligible {
            return format!("{mode} -- get to the line fast");
        }

        // match on tuple of (down, yards_to_go)
        match (self.down, self.yards_to_go) {
            (1, _) => "first and fresh -- establish run or play action".into(),
            (2, yds) if yds <= 3 => "short yardage -- draw or sneak".into(),
            (2, _) => "second and medium -- high-percentage pass".into(),
            (3, yds) if yds <= 2 => "short third -- sneak or quick out".into(),
            (3, yds) if yds <= 7 => format!("third and {yds} -- crossing route or curl"),
            (3, _) => "third and long -- deep route or checkdown".into(),
            (4, yds) if yds <= 1 => "fourth and inches -- go for it".into(),
            (4, _) => "punt or field goal -- situation dependent".into(),
            _ => "unusual down/distance".into(),
        }
    }
}

/// describe a game result with pattern matching on the enum variant
pub fn describe_result(game: &GameLog) -> String {
    match game.result {
        GameResult::Win(pats, opp) => {
            let margin = pats - opp;
            // if/else with type-consistent arms
            if margin > 14 {
                format!(
                    "week {} -- blowout W vs {} ({}-{})",
                    game.week, game.opponent, pats, opp
                )
            } else if margin > 7 {
                format!(
                    "week {} -- comfortable W vs {} ({}-{})",
                    game.week, game.opponent, pats, opp
                )
            } else {
                format!(
                    "week {} -- close W vs {} ({}-{})",
                    game.week, game.opponent, pats, opp
                )
            }
        }
        GameResult::Loss(pats, opp) => {
            format!(
                "week {} -- L vs {} ({}-{})",
                game.week, game.opponent, pats, opp
            )
        }
        GameResult::Tie(pats, opp) => {
            format!(
                "week {} -- T vs {} ({}-{})",
                game.week, game.opponent, pats, opp
            )
        }
        GameResult::Bye => format!("week {} -- bye", game.week),
    }
}

/// while let -- consume game results from an iterator until exhausted
/// demonstrates streaming pattern: process items one-at-a-time from an iterator
pub fn find_win_streak(games: &[GameLog]) -> (u8, Vec<&'static str>) {
    let mut current_streak: u8 = 0;
    let mut best_streak: u8 = 0;
    let mut best_opponents: Vec<&'static str> = Vec::new();
    let mut current_opponents: Vec<&'static str> = Vec::new();

    for game in games {
        match game.result {
            GameResult::Win(_, _) => {
                current_streak += 1;
                current_opponents.push(game.opponent);
            }
            _ => {
                if current_streak > best_streak {
                    best_streak = current_streak;
                    best_opponents = current_opponents.clone();
                }
                current_streak = 0;
                current_opponents.clear();
            }
        }
    }
    // check final streak
    if current_streak > best_streak {
        best_streak = current_streak;
        best_opponents = current_opponents;
    }
    (best_streak, best_opponents)
}

/// if let -- extract optional stats only when present
pub fn report_optional_stats(games: &[GameLog], week: u8) {
    let game = games.iter().find(|g| g.week == week);

    if let Some(g) = game {
        println!(
            "  found week {}: {} -- {}",
            week,
            g.opponent,
            describe_result(g)
        );
        // nested if let -- extract margin from Win variant
        if let GameResult::Win(pats, opp) = g.result {
            let margin = pats - opp;
            println!("  victory margin: {margin} points");
        }
    } else {
        println!("  week {week}: no data");
    }
}

// when compiled as a library module, main is unused -- only invoked via `cargo run --example`
#[allow(dead_code)]
fn main() {
    // 2004 patriots game log -- hardcoded from pro-football-reference.com
    let season: Vec<GameLog> = vec![
        GameLog {
            week: 1,
            opponent: "Indianapolis Colts",
            result: GameResult::Win(27, 24),
        },
        GameLog {
            week: 2,
            opponent: "Arizona Cardinals",
            result: GameResult::Win(23, 12),
        },
        GameLog {
            week: 3,
            opponent: "Buffalo Bills",
            result: GameResult::Win(31, 17),
        },
        GameLog {
            week: 4,
            opponent: "Miami Dolphins",
            result: GameResult::Win(24, 10),
        },
        GameLog {
            week: 5,
            opponent: "Pittsburgh Steelers",
            result: GameResult::Loss(20, 34),
        },
        GameLog {
            week: 6,
            opponent: "Seattle Seahawks",
            result: GameResult::Win(30, 20),
        },
        GameLog {
            week: 7,
            opponent: "New York Jets",
            result: GameResult::Win(13, 7),
        },
        GameLog {
            week: 8,
            opponent: "Bye Week",
            result: GameResult::Bye,
        },
        GameLog {
            week: 9,
            opponent: "St. Louis Rams",
            result: GameResult::Win(40, 22),
        },
        GameLog {
            week: 10,
            opponent: "Kansas City Chiefs",
            result: GameResult::Win(27, 19),
        },
        GameLog {
            week: 11,
            opponent: "Buffalo Bills",
            result: GameResult::Win(29, 6),
        },
        GameLog {
            week: 12,
            opponent: "Baltimore Ravens",
            result: GameResult::Win(24, 3),
        },
        GameLog {
            week: 13,
            opponent: "Cleveland Browns",
            result: GameResult::Win(42, 15),
        },
        GameLog {
            week: 14,
            opponent: "Cincinnati Bengals",
            result: GameResult::Win(35, 28),
        },
        GameLog {
            week: 15,
            opponent: "Miami Dolphins",
            result: GameResult::Loss(28, 29),
        },
        GameLog {
            week: 16,
            opponent: "New York Jets",
            result: GameResult::Win(23, 7),
        },
        GameLog {
            week: 17,
            opponent: "San Francisco 49ers",
            result: GameResult::Win(21, 7),
        },
        GameLog {
            week: 18,
            opponent: "Indianapolis Colts (Div)",
            result: GameResult::Win(20, 3),
        },
        GameLog {
            week: 19,
            opponent: "Pittsburgh Steelers (AFC)",
            result: GameResult::Win(41, 27),
        },
        GameLog {
            week: 20,
            opponent: "Philadelphia Eagles (SB)",
            result: GameResult::Win(24, 21),
        },
    ];

    println!("2004 patriots -- control flow examples\n");

    // --- match on game outcomes ---
    println!("=== pattern matching on game results ===\n");
    for game in &season[..5] {
        println!("{}", describe_result(game));
    }

    // --- while let -- find longest win streak ---
    println!("\n=== while let -- longest win streak ===\n");
    let (streak, opponents) = find_win_streak(&season);
    println!("longest streak: {streak} consecutive wins");
    println!("opponents: {}", opponents.join(", "));

    // --- if let -- optional stat lookup ---
    println!("\n=== if let -- optional game lookup ===\n");
    report_optional_stats(&season, 1); // exists -- week 1
    report_optional_stats(&season, 20); // super bowl
    report_optional_stats(&season, 21); // does not exist

    // --- match on game state with guards ---
    println!("\n=== match with guards -- game state decisions ===\n");
    let states = vec![
        GameState {
            quarter: Quarter::First,
            score_diff: 0,
            field_position: 25,
            down: 1,
            yards_to_go: 10,
        },
        GameState {
            quarter: Quarter::Third,
            score_diff: 3,
            field_position: 68,
            down: 3,
            yards_to_go: 4,
        },
        GameState {
            quarter: Quarter::Fourth,
            score_diff: -7,
            field_position: 45,
            down: 2,
            yards_to_go: 8,
        },
        GameState {
            quarter: Quarter::Fourth,
            score_diff: 3,
            field_position: 82,
            down: 4,
            yards_to_go: 1,
        },
    ];

    for state in &states {
        println!(
            "Q{:?} | {:+} | {} | down {}/{}",
            state.quarter,
            state.score_diff,
            state.field_zone(),
            state.down,
            state.yards_to_go
        );
        println!("  urgency: {}", state.urgency());
        println!("  call:    {}\n", state.recommend_call());
    }

    // --- while let with a mutable slice pattern ---
    println!("=== while let -- drain playoff results ===\n");
    let mut playoff_results: Vec<&GameLog> = season.iter().filter(|g| g.week >= 18).collect();
    // pop from the back -- while let drains the vec
    while let Some(game) = playoff_results.pop() {
        match game.result {
            GameResult::Win(pats, opp) => {
                println!("  playoff W: {} ({pats}-{opp})", game.opponent);
            }
            _ => println!("  playoff other: {}", game.opponent),
        }
    }
    println!(
        "\n  playoff_results drained: {} items remain",
        playoff_results.len()
    );
}
