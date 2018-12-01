use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod util;

fn main() -> std::io::Result<()> {
    let file = File::open("./puzzle_inputs/day11.txt")?;
    let mut input_string = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut input_string)?;
    let split = util::split_and_parse_string(&input_string);

    println!("{}", input_string);
    Ok(())
}
