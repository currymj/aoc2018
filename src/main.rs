use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod util;
mod day1;
fn main() -> std::io::Result<()> {
    // Day 1
    let file = File::open("./puzzle_inputs/day11.txt")?;
    let mut input_string = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut input_string)?;
    let split = util::split_and_parse_string(&input_string);

    let result_freq = day1::compute_final_frequency(&split);

    println!("{}", result_freq);



    Ok(())
}
