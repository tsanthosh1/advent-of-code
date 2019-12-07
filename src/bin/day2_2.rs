use std::cmp::min;

pub fn main() {
    let input = include_str!("../../data/two.data");
    let mut codes: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
    let mut success = false;
    let noun_and_verb_limit = (min(99, codes.len()) + 1) as i32;
    'outer: for noun in 0..noun_and_verb_limit {
        'inner: for verb in 0..noun_and_verb_limit {
            let mut input_codes = codes.clone();
            let intcode_output = intcode(&mut input_codes, &noun, &verb);
            if intcode_output[0] == String::from("19690720") {
                println!("noun and verb: {}", (noun * 100) + verb);
                success = true;
                break 'outer
            }
        }
    }
    if !success {
        println!("No solution")
    }
}

fn intcode(codes: &mut Vec<String>, noun: &i32, verb: &i32) -> Vec<String> {
    codes[1] = noun.to_string();
    codes[2] = verb.to_string();
    let mut current_opcode_position = 0;

    while (current_opcode_position < codes.len() - 1)
        && codes[current_opcode_position] != String::from("99") {

        let current_opcode = &codes[current_opcode_position];
        let input_position_1 = codes[current_opcode_position + 1].parse::<usize>().unwrap();
        let input_position_2 = codes[current_opcode_position + 2].parse::<usize>().unwrap();
        let output_position = codes[current_opcode_position + 3].parse::<usize>().unwrap();

        let input1 = codes[input_position_1].parse::<i32>().unwrap();
        let input2 = codes[input_position_2].parse::<i32>().unwrap();
        match current_opcode.as_str() {
            "1" => {
                let x = (input1 + input2).to_string();
                codes[output_position] = x
            }
            "2" => {
                let x1 = (input1 * input2).to_string();
                codes[output_position] = x1
            }
            _ => {}
        }

        current_opcode_position += 4;
    }
    codes.to_vec()
}
