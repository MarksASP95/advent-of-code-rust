use std::{str::FromStr, collections::HashMap};
mod input;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum ShapeName { ROCK, PAPER, SCISSORS, }
#[derive(Hash, PartialEq, Eq)]
enum ShapeNameShort { A, B, C, X, Y, Z, }

impl FromStr for ShapeNameShort {

    type Err = ();

    fn from_str(input: &str) -> Result<ShapeNameShort, Self::Err> {
        match input {
            "A" => Ok(ShapeNameShort::A),
            "B" => Ok(ShapeNameShort::B),
            "C" => Ok(ShapeNameShort::C),
            "X" => Ok(ShapeNameShort::X),
            "Y" => Ok(ShapeNameShort::Y),
            "Z" => Ok(ShapeNameShort::Z),
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
        (ShapeNameShort::X, ShapeName::ROCK),
        (ShapeNameShort::Y, ShapeName::PAPER),
        (ShapeNameShort::Z, ShapeName::SCISSORS),
    ])
        .get(ssn)
        .copied()
        .expect("Error getting shape name");
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

fn main() {    

    let value = input::get_input().trim();
    let value_split = value.split("\n");
    let mut total_score: u32 = 0;
    
    for item in value_split.into_iter() {
        let item_split = item.trim().split(" ").collect::<Vec<&str>>();

        let opponent_shape 
            = get_shape_name_by_short_name(&ShapeNameShort::from_str(item_split[0])
              .expect("Error parsing shape string"));
        let your_shape 
            = get_shape_name_by_short_name(&ShapeNameShort::from_str(item_split[1])
              .expect("Error parsing shape string"));
        let score: u32 = calculate_score(your_shape, get_winner(opponent_shape, your_shape));

        println!(
            "Opponent: {:?} - You: {:?} - Score: {}", 
            opponent_shape, 
            your_shape, 
            score
        );

        total_score += score;
    }

    println!("Total score: {}", total_score);
    
}
