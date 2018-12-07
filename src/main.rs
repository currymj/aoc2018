use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod util;
mod day1;
mod day2;
mod day3;
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


    // Day 2
    let file2 = File::open("./puzzle_inputs/day2.txt")?;
    let mut input2 = String::new();
    let mut buf2 = BufReader::new(file2);
    buf2.read_to_string(&mut input2)?;
    let input_vec: Vec<&str> = input2.split("\n").filter(|x| {x.len() >= 1}).collect();

    let results = day2::find_matching_letters(&input_vec);
    match results {
        Some(s) => println!("Day 2 Part 2: {}", s),
        None => println!("Uh-oh...")
    }

    //Day 3
    day3()?;


    Ok(())
}

fn day3() -> std::io::Result<()> {
    let file = File::open("./puzzle_inputs/day3.txt")?;

    let mut input = String::new();
    let mut buf = BufReader::new(file);
    buf.read_to_string(&mut input)?;


    let input_vec: Vec<&str> = input.split("\n").filter(|x| {x.len() >= 1}).collect();

    let result = day3::find_overlaps(&input_vec);
    println!("Day 3 Part 1: {}", result);

    Ok(())
}
