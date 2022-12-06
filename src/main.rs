use std::process;

use advent_of_code;

fn main() {
    let day = match advent_of_code::parse_args() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("Need to specify a day (as integer). example: `cargo run 2`");
            process::exit(1);
        }
    };

    advent_of_code::run(day, None);
}
