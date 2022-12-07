use std::fs;

pub mod solutions;

pub trait Solution {
    fn part_a(&self, input: &str) -> String;
    fn part_b(&self, input: &str) -> String;
}

pub fn load(day: u8, test: Option<bool>) -> String {
    let test = test.unwrap_or(false);

    let cwd = std::env::current_dir().unwrap();
    let path = cwd
        .join("data")
        .join(format!("{}", if test { "test_data" } else { "input" }))
        .join(format!("{:02}.txt", day));
    fs::read_to_string(path).unwrap()
}

pub fn parse_args() -> Result<u8, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    args.free_from_str()
}

pub fn run(day: u8, test: Option<bool>) {
    println!("[*] Running day {}...\n", day);

    let input = load(day, test);

    let solution: Option<Box<dyn Solution>> = match day {
        1 => Some(Box::new(solutions::Day1)),
        2 => Some(Box::new(solutions::Day2)),
        3 => Some(Box::new(solutions::Day3)),
        4 => Some(Box::new(solutions::Day4)),
        5 => Some(Box::new(solutions::Day5)),
        6 => Some(Box::new(solutions::Day6)),
        7 => Some(Box::new(solutions::Day7)),
        8 => Some(Box::new(solutions::Day8)),
        9 => Some(Box::new(solutions::Day9)),
        10 => Some(Box::new(solutions::Day10)),
        11 => Some(Box::new(solutions::Day11)),
        12 => Some(Box::new(solutions::Day12)),
        13 => Some(Box::new(solutions::Day13)),
        14 => Some(Box::new(solutions::Day14)),
        15 => Some(Box::new(solutions::Day15)),
        16 => Some(Box::new(solutions::Day16)),
        17 => Some(Box::new(solutions::Day17)),
        18 => Some(Box::new(solutions::Day18)),
        19 => Some(Box::new(solutions::Day19)),
        20 => Some(Box::new(solutions::Day20)),
        21 => Some(Box::new(solutions::Day21)),
        22 => Some(Box::new(solutions::Day22)),
        23 => Some(Box::new(solutions::Day23)),
        24 => Some(Box::new(solutions::Day24)),
        25 => Some(Box::new(solutions::Day25)),
        _ => None,
    };

    let solution = solution.expect("No solution found for this day");

    let part_a_time = std::time::Instant::now();
    let part_a = solution.part_a(&input);
    let part_a_time = part_a_time.elapsed();

    let part_b_time = std::time::Instant::now();
    let part_b = solution.part_b(&input);
    let part_b_time = part_b_time.elapsed();

    println!("--- Part A ---");
    println!("[+] OUT: {} ({} µs)\n", part_a, part_a_time.as_micros());
    println!("--- Part B ---");
    println!("[+] OUT: {} ({} µs)\n", part_b, part_b_time.as_micros());
}
