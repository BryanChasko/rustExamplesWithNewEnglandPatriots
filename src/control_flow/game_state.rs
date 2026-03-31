// control_flow -- game_state
//
// demonstrates: if/else, match on integers + enums + tuples, if let, while let, match guards
// python mirror: basic_match_name_input_NFL_legends.py
//
// 2004 patriots: game state decisions as type-safe match expressions

#[derive(Debug, PartialEq, Clone, Copy)]
enum Quarter { First, Second, Third, Fourth, Overtime }

#[derive(Debug)]
struct GameState {
    quarter: Quarter,
    score_diff: i8,   // patriots - opponent
    field_position: u8, // yards from own end zone (1-99)
    down: u8,
    yards_to_go: u8,
}

impl GameState {
    fn field_zone(&self) -> &'static str {
        match self.field_position {
            1..=25    => "own territory -- deep",
            26..=49   => "own territory",
            50        => "midfield",
            51..=74   => "opponent territory",
            75..=99   => "red zone",
            _         => "invalid",
        }
    }

    fn urgency(&self) -> &'static str {
        match (self.quarter, self.score_diff) {
            (Quarter::Fourth, diff) if diff < -8  => "two-score deficit -- need haste",
            (Quarter::Fourth, diff) if diff < 0   => "one-score game -- every drive counts",
            (Quarter::Fourth, diff) if diff > 10  => "comfortable lead -- run clock",
            (Quarter::Overtime, _)                => "sudden death",
            _                                     => "standard game management",
        }
    }

    fn recommend_call(&self) -> String {
        // if let -- only act when the option matches
        let hurry_up_eligible: Option<&str> = if self.quarter == Quarter::Fourth && self.score_diff < 0 {
            Some("hurry-up offense")
        } else {
            None
        };

        if let Some(mode) = hurry_up_eligible {
            return format!("{mode} -- get to the line fast");
        }

        // match on tuple of (down, yards_to_go)
        match (self.down, self.yards_to_go) {
            (1, _)           => "first and fresh -- establish run or play action".into(),
            (2, yds) if yds <= 3 => "short yardage -- draw or sneak".into(),
            (2, _)           => "second and medium -- high-percentage pass".into(),
            (3, yds) if yds <= 2 => "short third -- sneak or quick out".into(),
            (3, yds) if yds <= 7 => format!("third and {yds} -- crossing route or curl"),
            (3, _)           => "third and long -- deep route or checkdown".into(),
            (4, yds) if yds <= 1 => "fourth and inches -- go for it".into(),
            (4, _)           => "punt or field goal -- situation dependent".into(),
            _                => "unusual down/distance".into(),
        }
    }
}

fn main() {
    let states = vec![
        GameState { quarter: Quarter::First,  score_diff: 0,  field_position: 25, down: 1, yards_to_go: 10 },
        GameState { quarter: Quarter::Third,  score_diff: 3,  field_position: 68, down: 3, yards_to_go: 4  },
        GameState { quarter: Quarter::Fourth, score_diff: -7, field_position: 45, down: 2, yards_to_go: 8  },
        GameState { quarter: Quarter::Fourth, score_diff: 3,  field_position: 82, down: 4, yards_to_go: 1  },
    ];

    println!("2004 patriots -- game state analysis\n");

    for state in &states {
        println!("Q{:?} | {:+} | {} | down {}/{}", 
            state.quarter, state.score_diff, state.field_zone(), state.down, state.yards_to_go);
        println!("  urgency: {}", state.urgency());
        println!("  call:    {}\n", state.recommend_call());
    }
}
