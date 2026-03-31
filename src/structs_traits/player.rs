// structs_traits -- player
//
// demonstrates: struct definitions, impl blocks, trait implementation, Display
// python mirror: object_oriented_programming/oop_player_getter.py
//
// 2004 patriots: roster data as typed structs

use std::fmt;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub number: u8,
    pub position: Position,
    pub years_with_patriots: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    Quarterback,
    WideReceiver,
    RunningBack,
    TightEnd,
    OffensiveLine,
    DefensiveLine,
    Linebacker,
    Cornerback,
    Safety,
    Kicker,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Position::Quarterback => "QB",
            Position::WideReceiver => "WR",
            Position::RunningBack => "RB",
            Position::TightEnd => "TE",
            Position::OffensiveLine => "OL",
            Position::DefensiveLine => "DL",
            Position::Linebacker => "LB",
            Position::Cornerback => "CB",
            Position::Safety => "S",
            Position::Kicker => "K",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{} {} ({}) -- {} year{} with patriots",
            self.number,
            self.name,
            self.position,
            self.years_with_patriots,
            if self.years_with_patriots == 1 { "" } else { "s" }
        )
    }
}

pub trait Stats {
    fn position_group(&self) -> &'static str;
    fn is_skill_position(&self) -> bool;
}

impl Stats for Player {
    fn position_group(&self) -> &'static str {
        match self.position {
            Position::Quarterback | Position::WideReceiver
            | Position::RunningBack | Position::TightEnd => "offense",
            Position::OffensiveLine => "o-line",
            Position::DefensiveLine | Position::Linebacker
            | Position::Cornerback | Position::Safety => "defense",
            Position::Kicker => "special teams",
        }
    }

    fn is_skill_position(&self) -> bool {
        matches!(
            self.position,
            Position::Quarterback | Position::WideReceiver | Position::RunningBack | Position::TightEnd
        )
    }
}

fn main() {
    let roster: Vec<Player> = vec![
        Player { name: "Tom Brady".into(), number: 12, position: Position::Quarterback, years_with_patriots: 5 },
        Player { name: "Corey Dillon".into(), number: 28, position: Position::RunningBack, years_with_patriots: 1 },
        Player { name: "David Givens".into(), number: 87, position: Position::WideReceiver, years_with_patriots: 3 },
        Player { name: "Tedy Bruschi".into(), number: 54, position: Position::Linebacker, years_with_patriots: 9 },
        Player { name: "Ty Law".into(), number: 24, position: Position::Cornerback, years_with_patriots: 10 },
        Player { name: "Adam Vinatieri".into(), number: 4, position: Position::Kicker, years_with_patriots: 9 },
    ];

    println!("2004 new england patriots -- selected roster\n");
    for player in &roster {
        println!("{player}");
        println!("  group: {} | skill position: {}", player.position_group(), player.is_skill_position());
    }

    let skill: Vec<_> = roster.iter().filter(|p| p.is_skill_position()).collect();
    println!("\nskill position players: {}", skill.len());
}
