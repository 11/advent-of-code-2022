use std::fs::File;
use std::process;
use std::io::{ prelude::*, BufReader };
use std::collections::HashSet;


fn find_overlap<'a>(left: &'a Vec<char>, right: &'a Vec<char>) -> Vec<&'a char> {
    let left_set: HashSet<&char> = HashSet::from_iter(left);
    let right_set: HashSet<&char> = HashSet::from_iter(right);
    return left_set.intersection(&right_set).copied().collect();
}


fn get_priority_count(chars: &Vec<&char>) -> u32 {
    return chars
        .iter()
        .map(|c| {
            return if !c.is_ascii() {
                return 0;
            } else if c.is_lowercase() {
                (**c as u32) - 96
            } else {
                (**c as u32) - 64 + 26
            };
        })
        .sum()
}


fn main() {
    let file = File::open("day3-input.txt").expect("Could not read day3-input.txt");
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let mut items = String::from(line.unwrap());

        let half = items.len() / 2;
        let left: Vec<char> = items.split_off(half).chars().collect();
        let right: Vec<char> = items.chars().collect();

        let overlap = find_overlap(&left, &right);

        let count = get_priority_count(&overlap);
        total += count;
    }

    println!("{}", total);
}
