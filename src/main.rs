use std::fmt;
use std::{str::FromStr, collections::HashMap};
mod input;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum ShapeName { ROCK, PAPER, SCISSORS, }
#[derive(Hash, PartialEq, Eq)]
enum ShapeNameShort { A, B, C }
enum WantedOutcome { X, Y, Z, }

impl fmt::Display for WantedOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WantedOutcome::X => write!(f, "Loose"),
            WantedOutcome::Y => write!(f, "Tie"),
            WantedOutcome::Z => write!(f, "Win"),
        }
    }
}

impl FromStr for WantedOutcome {

    type Err = ();

    fn from_str(input: &str) -> Result<WantedOutcome, Self::Err> {
        match input {
            "X" => Ok(WantedOutcome::X),
            "Y" => Ok(WantedOutcome::Y),
            "Z" => Ok(WantedOutcome::Z),
            _   => Err(()),
        }
    }
}

impl FromStr for ShapeNameShort {

    type Err = ();

    fn from_str(input: &str) -> Result<ShapeNameShort, Self::Err> {
        match input {
            "A" => Ok(ShapeNameShort::A),
            "B" => Ok(ShapeNameShort::B),
            "C" => Ok(ShapeNameShort::C),
            _   => Err(()),
        }
    }
}

fn get_shape_score(s: &ShapeName) -> u32 {
    return HashMap::from([
        (ShapeName::ROCK, 1),
        (ShapeName::PAPER, 2),
        (ShapeName::SCISSORS, 3),
    ])
        .get(&s)
        .copied()
        .expect("Error getting shape score");
}

fn get_shape_name_by_short_name(ssn: &ShapeNameShort) -> ShapeName {
    return HashMap::from([
        (ShapeNameShort::A, ShapeName::ROCK),
        (ShapeNameShort::B, ShapeName::PAPER),
        (ShapeNameShort::C, ShapeName::SCISSORS),
    ])
        .get(ssn)
        .copied()
        .expect("Error getting shape name");
}

fn beats(s: &ShapeName) -> ShapeName {
    return HashMap::from([
        (ShapeName::ROCK, ShapeName::PAPER),
        (ShapeName::PAPER, ShapeName::SCISSORS),
        (ShapeName::SCISSORS, ShapeName::ROCK),
    ])
        .get(&s)
        .copied()
        .expect("Error getting beat shape");
}

fn loses_to(s: &ShapeName) -> ShapeName {
    return HashMap::from([
        (ShapeName::ROCK, ShapeName::SCISSORS),
        (ShapeName::PAPER, ShapeName::ROCK),
        (ShapeName::SCISSORS, ShapeName::PAPER),
    ])
        .get(&s)
        .copied()
        .expect("Error getting beat shape");
}

fn get_winner(s1: ShapeName, s2: ShapeName) -> Option<ShapeName> {

    if loses_to(&s1) == s2 { return Some(s1); }
    if loses_to(&s2) == s1 { return Some(s2); }

    return None;
}

fn calculate_score(your_shape: ShapeName, winning_shape: Option<ShapeName>) -> u32 {
    let shape_score: u32 = get_shape_score(&your_shape);

    let outcome_score: u32 = match winning_shape {
        Some(ws) if ws == your_shape => 6,
        Some(_) => 0,
        None => 3,
    };

    return shape_score + outcome_score;

}

fn get_wanted_shape(opponent_shape: &ShapeName, wanted_outcome: &WantedOutcome) -> ShapeName {
    match wanted_outcome {
        WantedOutcome::X => loses_to(&opponent_shape),
        WantedOutcome::Y => *opponent_shape,
        WantedOutcome::Z => beats(&opponent_shape),
    }
}

fn main() {    

    let value = input::get_input().trim();
    let value_split = value.split("\n");
    let mut total_score: u32 = 0;
    
    for item in value_split.into_iter() {
        let item_split = item.trim().split(" ").collect::<Vec<&str>>();

        let opponent_shape: ShapeName 
            = get_shape_name_by_short_name(&ShapeNameShort::from_str(item_split[0])
              .expect("Error parsing shape string"));
        let wanted_outcome: WantedOutcome = WantedOutcome::from_str(item_split[1])
            .expect("Error getting wanted outcome");
        let wanted_shape: ShapeName = get_wanted_shape(&opponent_shape, &wanted_outcome);
        let score: u32 = calculate_score(wanted_shape, get_winner(opponent_shape, wanted_shape));

        println!(
            "Opponent: {:?} - You must: {} - Score: {}", 
            opponent_shape, 
            wanted_outcome, 
            score
        );

        total_score += score;
    }

    println!("Total score: {}", total_score);
    
}
