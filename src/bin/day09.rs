use std::str::Lines;
use intcode::IntCode;

pub fn main() {

    let contents = include_str!("../../data/nine.data");
    let mut intcode = IntCode::initialize(contents, Some(2));
    intcode.execute();

    println!("Total fuel required: {}", intcode.output_string())
}

