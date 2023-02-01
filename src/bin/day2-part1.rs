use std::fs::File;
use std::io::{ prelude::*, BufReader };
use std::collections::HashMap;

fn calculate_score_for_turn(oponent: &str, me: &str) -> i32 {
    let my_selection = match me {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    };

    let outcome = match (oponent, me) {
        ("rock",    "rock")    => 3,
        ("rock",    "paper")   => 6,
        ("rock",    "scissors") => 0,
        ("paper",   "rock")    => 0,
        ("paper",   "paper")   => 3,
        ("paper",   "scissors") => 6,
        ("scissors", "rock")    => 6,
        ("scissors", "paper")   => 0,
        ("scissors", "scissors") => 3,
        _ => 0,
    };

    return my_selection + outcome;
}

fn main() {
    let oponent_options = HashMap::from([
        ("a", "rock"),
        ("b", "paper"),
        ("c", "scissors"),
    ]);

    let my_options = HashMap::from([
        ("x", "rock"),
        ("y", "paper"),
        ("z", "scissors"),
    ]);

    let file = File::open("day2-input.txt").expect("Could not load day2-input.txt");
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let row = line.to_lowercase();
            let pair = row.split(' ').collect::<Vec<&str>>();

            let oponent = oponent_options[pair[0]];
            let me = my_options[pair[1]];
            let score = calculate_score_for_turn(&oponent, &me);

            total_score += score
        }
    }

    println!("{}", total_score);
}
