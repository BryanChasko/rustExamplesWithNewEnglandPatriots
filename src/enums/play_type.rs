// enums -- play_type
//
// demonstrates: enums with data, pattern matching, match exhaustiveness,
//               enum methods via impl
// python mirror: n/a -- rust enums with data have no clean python equivalent
//
// 2004 patriots: play types as a type-safe enum

#[derive(Debug, Clone, PartialEq)]
pub enum PlayType {
    Run {
        carrier: String,
        yards: i8,
    },
    Pass {
        target: String,
        yards: i8,
        complete: bool,
    },
    Punt {
        hangtime: f32,
        net_yards: u8,
    },
    FieldGoal {
        distance: u8,
        good: bool,
    },
    Kickoff {
        touchback: bool,
    },
    Penalty {
        against: String,
        yards: u8,
        loss_of_down: bool,
    },
}

impl PlayType {
    pub fn is_scoring_opportunity(&self) -> bool {
        match self {
            PlayType::FieldGoal { .. } => true,
            PlayType::Pass { yards, .. } if *yards > 20 => true,
            _ => false,
        }
    }

    pub fn net_yards(&self) -> i32 {
        match self {
            PlayType::Run { yards, .. } => *yards as i32,
            PlayType::Pass {
                complete: true,
                yards,
                ..
            } => *yards as i32,
            PlayType::Pass {
                complete: false, ..
            } => 0,
            PlayType::Punt { net_yards, .. } => -(*net_yards as i32),
            PlayType::FieldGoal { good: true, .. } => 0, // handled separately
            PlayType::FieldGoal { good: false, .. } => 0,
            PlayType::Kickoff { .. } => 0,
            PlayType::Penalty {
                yards,
                loss_of_down: false,
                ..
            } => -(*yards as i32),
            PlayType::Penalty {
                yards,
                loss_of_down: true,
                ..
            } => -(*yards as i32),
        }
    }

    pub fn describe(&self) -> String {
        match self {
            PlayType::Run { carrier, yards } => format!("{carrier} runs for {yards} yards"),
            PlayType::Pass {
                target,
                yards,
                complete: true,
            } => format!("complete to {target} for {yards} yards"),
            PlayType::Pass {
                target,
                complete: false,
                ..
            } => format!("incomplete, intended for {target}"),
            PlayType::Punt { net_yards, .. } => format!("punt, {net_yards} yards net"),
            PlayType::FieldGoal {
                distance,
                good: true,
            } => format!("field goal GOOD from {distance} yards"),
            PlayType::FieldGoal {
                distance,
                good: false,
            } => format!("field goal NO GOOD from {distance} yards"),
            PlayType::Kickoff { touchback: true } => "kickoff touchback".into(),
            PlayType::Kickoff { .. } => "kickoff".into(),
            PlayType::Penalty { against, yards, .. } => {
                format!("penalty against {against} -- {yards} yards")
            }
        }
    }
}

fn main() {
    let drive: Vec<PlayType> = vec![
        PlayType::Pass {
            target: "David Givens".into(),
            yards: 12,
            complete: true,
        },
        PlayType::Run {
            carrier: "Corey Dillon".into(),
            yards: 7,
        },
        PlayType::Pass {
            target: "Deion Branch".into(),
            yards: 0,
            complete: false,
        },
        PlayType::Pass {
            target: "Daniel Graham".into(),
            yards: 18,
            complete: true,
        },
        PlayType::Run {
            carrier: "Kevin Faulk".into(),
            yards: 3,
        },
        PlayType::FieldGoal {
            distance: 38,
            good: true,
        },
    ];

    println!("2004 patriots -- drive analysis (pattern matching demo)\n");

    let mut total_yards = 0i32;
    for (i, play) in drive.iter().enumerate() {
        let yards = play.net_yards();
        total_yards += yards;
        println!("play {}: {} ({:+} yds)", i + 1, play.describe(), yards);
    }

    println!("\ndrive total: {total_yards} yards");

    let scoring_ops = drive.iter().filter(|p| p.is_scoring_opportunity()).count();
    println!("scoring opportunities: {scoring_ops}");
}
