use std::fs::File;
use std::io::{ prelude::*, BufReader };
use std::collections::HashMap;


fn calculate_score_for_turn(oponent: &str, me: &str, game_options: &HashMap<&str, i32>, my_selection: &HashMap<&str, i32>) -> i32 {

    let outcome = match (oponent, me) {
        ("rock",     "win")  => "paper",
        ("rock",     "lose") => "scissors",
        ("rock",     "draw") => "rock",
        ("paper",    "win")  => "scissors",
        ("paper",    "lose") => "rock",
        ("paper",    "draw") => "paper",
        ("scissors", "win")  => "rock",
        ("scissors", "lose") => "paper",
        ("scissors", "draw") => "scissors",
        _ => "",
    };

    return my_selection[me] + game_options[outcome];
}

fn main() {
    let oponent_options = HashMap::from([
        ("a", "rock"),
        ("b", "paper"),
        ("c", "scissors"),
    ]);

    let my_options = HashMap::from([
        ("x", "lose"),
        ("y", "draw"),
        ("z", "win"),
    ]);

    let my_selection: HashMap<&str, i32> = HashMap::from([
        ("win",  6),
        ("draw", 3),
        ("lose", 0),
    ]);

    let game_options: HashMap<&str, i32> = HashMap::from([
        ("rock",     1),
        ("paper",    2),
        ("scissors", 3),
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
            let score = calculate_score_for_turn(&oponent, &me, &game_options, &my_selection);

            total_score += score
        }
    }

    println!("{}", total_score);
}
