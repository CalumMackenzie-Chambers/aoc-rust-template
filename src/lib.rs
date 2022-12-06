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

    let solution = match day {
        1 => solutions::Day1,
        _ => panic!("Day {} not implemented", day),
    };

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
