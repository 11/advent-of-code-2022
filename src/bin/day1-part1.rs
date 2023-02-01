use std::fs::File;
use std::io::{ prelude::*, BufReader };

fn main() {
    let reader = BufReader::new(File::open("day1-input.txt").expect("Cannot open day1-input.txt"));

    let mut max_calories = 0;
    let mut calorie_count = 0;
    for line in reader.lines() {
        match line {
            Ok(line) if line.trim() == "" => {
                if calorie_count > max_calories {
                    max_calories = calorie_count;
                }

                calorie_count = 0;
            },
            Ok(line) => {
                let calories = line.trim().parse::<i32>().unwrap();
                calorie_count += calories;
            },
            Err(_) => continue,
        };
    }

    println!("{}", max_calories);
}
