use std::env;
use std::fs;
use std::iter::zip;

#[derive(PartialEq, Eq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Eq, Clone)]
enum Outcome {
    Win,
    Tie,
    Loss
}

fn rps_parse(in_char: &str) -> RPS {

    match in_char {
        "A" | "X" => RPS::Rock,
        "B" | "Y" => RPS::Paper,
        "C" | "Z" => RPS::Scissors,
        _ => {
            println!("Unexpected input to rps_parse(): {}", in_char);
            panic!();
        }
    }
}

fn outcome_parse(in_char: &str) -> Outcome {

    match in_char {
        "X" => Outcome::Loss,
        "Y" => Outcome::Tie,
        "Z" => Outcome::Win,
        _ => {
            println!("Unexpected input to outcome_parse(): {}", in_char);
            panic!();
        }
    }
}

fn rps_event(ours: &RPS, theirs: &RPS) -> Outcome {

    if ours == theirs {
        return Outcome::Tie;
    }

    let victory = match ours {
        RPS::Rock        => (*theirs == RPS::Scissors),
        RPS::Paper       => (*theirs == RPS::Rock),
        RPS::Scissors    => (*theirs == RPS::Paper)
    };

    if victory { Outcome::Win } else { Outcome::Loss }
}

fn force_outcome(theirs: &RPS, outcome: &Outcome) -> RPS {

    match outcome {
        Outcome::Tie    => { theirs.clone() }
        Outcome::Win    => {
            match theirs {
                RPS::Rock       => RPS::Paper,
                RPS::Paper      => RPS::Scissors,
                RPS::Scissors   => RPS::Rock,
            }
        }
        Outcome::Loss   => {
            match theirs {
                RPS::Rock       => RPS::Scissors,
                RPS::Paper      => RPS::Rock,
                RPS::Scissors   => RPS::Paper,
            }
        }
    }
}

fn rps_score(ours: &RPS, outcome: &Outcome) -> u32 {

    let selected_score = match ours {
        RPS::Rock       => 1,
        RPS::Paper       => 2,
        RPS::Scissors    => 3
    };

    let outcome_score = match outcome {
        Outcome::Win    => 6,
        Outcome::Tie    => 3,
        Outcome::Loss   => 0
    };

    selected_score + outcome_score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let string_in = fs::read_to_string(fname).unwrap();
    let string_split: Vec<&str> = string_in.split("\n").collect();

    let mut ours: Vec<RPS> = Vec::new();
    let mut theirs: Vec<RPS> = Vec::new();
    let mut outcomes: Vec<Outcome> = Vec::new();
    for line in string_split.iter() {
        if line.len() == 0 { break; }
        let values: Vec<&str> = line.split(" ").collect();
        theirs.push(rps_parse(values[0]));
        ours.push(rps_parse(values[1]));
        outcomes.push(outcome_parse(values[1])); // pt2
    }

    let total1 = zip(&ours, &theirs)
        .map(|(o, t)| (rps_event(&o, &t), o))
        .map(|(outcome, o)| rps_score(&o, &outcome))
        .sum::<u32>();
    println!("Pt1 total: {}", total1);

    let total2 = zip(&theirs, &outcomes)
        .map(|(t, outcome)| (force_outcome(&t, &outcome), t))
        .map(|(o, t)| (rps_event(&o, &t), o))
        .map(|(outcome, o)| rps_score(&o, &outcome))
        .sum::<u32>();
    println!("Pt2 total: {}", total2);
}
