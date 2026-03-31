// error_handling -- game_result
//
// demonstrates: Result<T,E>, Option<T>, the ? operator, custom error types
// python mirror: basic_function_exception_logging_hello_world_input_wr_name.py
//
// rust replaces try/except with Result -- errors are values, not exceptions

use std::fmt;

#[derive(Debug)]
pub enum PatriotsError {
    PlayerNotFound(String),
    InvalidStatline(String),
    DatasetMissing(String),
}

impl fmt::Display for PatriotsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatriotsError::PlayerNotFound(name) => write!(f, "player not found: {name}"),
            PatriotsError::InvalidStatline(msg) => write!(f, "invalid statline: {msg}"),
            PatriotsError::DatasetMissing(path) => write!(f, "dataset missing: {path}"),
        }
    }
}

impl std::error::Error for PatriotsError {}

struct GameLog {
    week: u8,
    opponent: &'static str,
    score: Option<(u8, u8)>, // (patriots, opponent)
}

fn get_result(game: &GameLog) -> Result<String, PatriotsError> {
    match game.score {
        None => Err(PatriotsError::InvalidStatline(
            format!("week {} vs {} has no score", game.week, game.opponent)
        )),
        Some((us, them)) if us > them => Ok(format!("W {us}-{them}")),
        Some((us, them)) if them > us => Ok(format!("L {us}-{them}")),
        Some((us, them)) => Ok(format!("T {us}-{them}")),
    }
}

fn find_player(roster: &[&str], name: &str) -> Result<String, PatriotsError> {
    roster
        .iter()
        .find(|&&p| p.to_lowercase() == name.to_lowercase())
        .map(|&p| p.to_string())
        .ok_or_else(|| PatriotsError::PlayerNotFound(name.to_string()))
}

fn main() {
    let games = vec![
        GameLog { week: 1, opponent: "Colts", score: Some((27, 24)) },
        GameLog { week: 2, opponent: "Cardinals", score: Some((23, 12)) },
        GameLog { week: 3, opponent: "Bills", score: None }, // corrupted data
        GameLog { week: 4, opponent: "Dolphins", score: Some((24, 10)) },
    ];

    println!("2004 patriots game results -- error handling demo\n");

    for game in &games {
        match get_result(game) {
            Ok(result) => println!("week {}: vs {} -- {}", game.week, game.opponent, result),
            Err(e) => println!("week {}: vs {} -- error: {e}", game.week, game.opponent),
        }
    }

    println!("\nroster lookup demo:");
    let roster = ["Tom Brady", "Corey Dillon", "Tedy Bruschi", "Adam Vinatieri"];

    for name in ["Tom Brady", "Randy Moss", "Tedy Bruschi"] {
        match find_player(&roster, name) {
            Ok(player) => println!("  found: {player}"),
            Err(e) => println!("  {e}"),
        }
    }
}
