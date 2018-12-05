use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod util;
mod day1;
mod day2;
fn main() -> std::io::Result<()> {
    // Day 1
    let file = File::open("./puzzle_inputs/day11.txt")?;
    let mut input_string = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut input_string)?;
    let split = util::split_and_parse_string(&input_string);

    let result_freq = day1::compute_final_frequency(&split);

    let second_result_freq = day1::compute_repeat_frequency(&split);
    println!("Day 1 Part 1: {}", result_freq);
    println!("Day 1 Part 2: {}", second_result_freq);



    Ok(())
}
