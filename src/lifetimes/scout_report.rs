// lifetimes/scout_report.rs
//
// demonstrates lifetime annotations through 2004 patriots scout reports.
// a ScoutReport borrows from the roster vec -- the compiler enforces that
// the report cannot outlive the roster it references.

use std::fmt;

/// a player on the 2004 patriots roster
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub position: String,
    pub number: u8,
    pub years_pro: u8,
}

impl Player {
    pub fn new(name: &str, position: &str, number: u8, years_pro: u8) -> Self {
        Self {
            name: name.to_string(),
            position: position.to_string(),
            number,
            years_pro,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} {} ({})", self.number, self.name, self.position)
    }
}

/// a scout report that borrows a reference to a player from the roster.
/// the lifetime `'a` ties the report's validity to the player it references --
/// the report cannot outlive the roster entry.
#[derive(Debug)]
pub struct ScoutReport<'a> {
    pub player: &'a Player,
    pub grade: f32,
    pub notes: &'a str,
}

impl<'a> ScoutReport<'a> {
    /// create a scout report borrowing from a player reference.
    /// both the player ref and the notes string must live at least as long as
    /// the returned ScoutReport.
    pub fn new(player: &'a Player, grade: f32, notes: &'a str) -> Self {
        Self {
            player,
            grade,
            notes,
        }
    }
}

impl fmt::Display for ScoutReport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "scout report: {} | grade: {:.1} | {}",
            self.player, self.grade, self.notes
        )
    }
}

/// returns a reference to whichever player has more years in the league.
/// explicit lifetime annotation: the returned reference lives as long as
/// both input references do.
pub fn longer_career<'a>(p1: &'a Player, p2: &'a Player) -> &'a Player {
    if p1.years_pro >= p2.years_pro {
        p1
    } else {
        p2
    }
}

/// returns the player with the highest jersey number from a borrowed slice.
/// demonstrates lifetime elision -- the compiler infers `'a` from the single
/// input reference. written with explicit annotation for clarity.
#[allow(clippy::needless_lifetimes)]
pub fn highest_number<'a>(roster: &'a [Player]) -> Option<&'a Player> {
    roster.iter().max_by_key(|p| p.number)
}

/// filter a roster slice to a single position group.
/// returns borrowed references tied to the input slice lifetime.
pub fn position_group<'a>(roster: &'a [Player], position: &str) -> Vec<&'a Player> {
    roster.iter().filter(|p| p.position == position).collect()
}

/// a roster view holding a borrowed slice of players.
/// demonstrates struct lifetime parameters on slices.
#[derive(Debug)]
pub struct RosterView<'a> {
    pub team: &'a str,
    pub players: &'a [Player],
}

impl<'a> RosterView<'a> {
    pub fn new(team: &'a str, players: &'a [Player]) -> Self {
        Self { team, players }
    }

    /// return the veteran (most years pro) from this view.
    /// lifetime of the returned reference is tied to the struct's lifetime.
    pub fn veteran(&self) -> Option<&'a Player> {
        self.players.iter().max_by_key(|p| p.years_pro)
    }

    /// count players at a given position
    pub fn count_position(&self, position: &str) -> usize {
        self.players
            .iter()
            .filter(|p| p.position == position)
            .count()
    }
}

/// demonstrates 'static lifetime -- string literals live for the entire program.
pub fn team_motto() -> &'static str {
    "do your job"
}

/// elision example: single input reference means the output lifetime is inferred.
/// this compiles without explicit annotation because of elision rule #1:
/// each input reference gets its own lifetime, and with one input the output
/// borrows from it.
pub fn first_player(roster: &[Player]) -> Option<&Player> {
    roster.first()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roster() -> Vec<Player> {
        vec![
            Player::new("Tom Brady", "QB", 12, 5),
            Player::new("Corey Dillon", "RB", 28, 8),
            Player::new("Deion Branch", "WR", 83, 3),
            Player::new("David Givens", "WR", 87, 3),
            Player::new("Tedy Bruschi", "LB", 54, 9),
            Player::new("Richard Seymour", "DL", 93, 4),
            Player::new("Ty Law", "CB", 24, 10),
            Player::new("Adam Vinatieri", "K", 4, 9),
        ]
    }

    #[test]
    fn test_scout_report_borrows_player() {
        let roster = test_roster();
        let notes = "franchise quarterback, 2x super bowl mvp candidate";
        let report = ScoutReport::new(&roster[0], 9.8, notes);
        assert_eq!(report.player.name, "Tom Brady");
        assert_eq!(report.grade, 9.8);
        assert!(report.notes.contains("franchise"));
    }

    #[test]
    fn test_longer_career() {
        let roster = test_roster();
        let veteran = longer_career(&roster[0], &roster[6]);
        assert_eq!(veteran.name, "Ty Law");
        assert_eq!(veteran.years_pro, 10);
    }

    #[test]
    fn test_highest_number() {
        let roster = test_roster();
        let highest = highest_number(&roster).unwrap();
        assert_eq!(highest.name, "Richard Seymour");
        assert_eq!(highest.number, 93);
    }

    #[test]
    fn test_position_group() {
        let roster = test_roster();
        let wrs = position_group(&roster, "WR");
        assert_eq!(wrs.len(), 2);
        assert!(wrs.iter().all(|p| p.position == "WR"));
    }

    #[test]
    fn test_roster_view_veteran() {
        let roster = test_roster();
        let view = RosterView::new("patriots", &roster);
        let vet = view.veteran().unwrap();
        assert_eq!(vet.name, "Ty Law");
    }

    #[test]
    fn test_roster_view_count_position() {
        let roster = test_roster();
        let view = RosterView::new("patriots", &roster);
        assert_eq!(view.count_position("WR"), 2);
        assert_eq!(view.count_position("QB"), 1);
        assert_eq!(view.count_position("TE"), 0);
    }

    #[test]
    fn test_static_lifetime() {
        let motto: &'static str = team_motto();
        assert_eq!(motto, "do your job");
    }

    #[test]
    fn test_first_player_elision() {
        let roster = test_roster();
        let first = first_player(&roster).unwrap();
        assert_eq!(first.name, "Tom Brady");
    }

    #[test]
    fn test_empty_roster() {
        let empty: Vec<Player> = vec![];
        assert!(highest_number(&empty).is_none());
        assert!(first_player(&empty).is_none());
        assert!(position_group(&empty, "QB").is_empty());
    }

    #[test]
    fn test_display_formats() {
        let roster = test_roster();
        let report = ScoutReport::new(&roster[0], 9.5, "elite");
        let display = format!("{}", report);
        assert!(display.contains("Tom Brady"));
        assert!(display.contains("9.5"));
        assert!(display.contains("elite"));
    }
}
