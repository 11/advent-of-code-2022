use std::fs::File;
use std::io::{ prelude::*, BufReader };

fn main() {
    let file = File::open("day1-input.txt").expect("Cannot open day1-input.txt");
    let reader = BufReader::new(file);

    let mut elf_calories = reader
        .lines()
        .map(|line| {
            match line {
                Ok(line) if line.trim() == "" => 0,
                Ok(line) => line.trim().parse::<i32>().unwrap(),
                _ => 0
            }
        })
        .collect::<Vec<i32>>()
        .rsplit(|num| *num == 0)
        .map(|range| range.iter().sum())
        .collect::<Vec<i32>>();

    elf_calories.sort_by(|a,b| b.cmp(a));

    let (result, _) = elf_calories.split_at(3);


    println!("{:?}", result.iter().sum::<i32>());

}
