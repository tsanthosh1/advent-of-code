pub fn main() {
    let input = include_str!("../../data/five.data");
    let device_id = 5;
    let (codes, _) = intcode(input, &device_id.to_string());

    println!("Output of Intcode: {}", codes.join(","))
}

fn intcode(input: &str, device_id: &str) -> (Vec<String>, i32) {
    let mut codes: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
    let mut current_opcode_position = 0;

    while (current_opcode_position < codes.len() - 1)
        && codes[current_opcode_position] != String::from("99") {

        let current_instruction = &codes[current_opcode_position].parse::<usize>().unwrap();
        let current_opcode = current_instruction % 10;
        let input1 = get_input1(&codes, &current_opcode_position, current_instruction);

        match current_opcode {
            1 => add(&mut codes, &mut current_opcode_position, current_instruction, input1),
            2 => multiply(&mut codes, &mut current_opcode_position, current_instruction, input1),
            3 => get_input(device_id, &mut codes, &mut current_opcode_position),
            4 => {
                current_opcode_position += 2;
                let next_command = &codes[current_opcode_position];
                if input1 != 0 && *next_command != String::from("99") {
                    panic!("It can't be non-zero")
                }
                println!("Output {}", input1);
                if *next_command == String::from("99") {
                    return (codes, input1);
                }
            }
            5 => jump_if_true(&mut codes, &mut current_opcode_position, current_instruction, input1),
            6 => jump_if_false(&mut codes, &mut current_opcode_position, current_instruction, input1),
            7 => on_first_parameter_lesser_than_second(&mut codes, &mut current_opcode_position, current_instruction, input1),
            8 => on_both_parameters_equal(&mut codes, &mut current_opcode_position, current_instruction, input1),
            _ => {
                panic!("Invalid opcode")
            }
        }

        println!("{}", codes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }
    (codes, 0)
}

fn on_both_parameters_equal(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_input2(&mut codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    codes[output_position] = if input1 == input2 { 1 } else { 0 }.to_string();
    *current_opcode_position += 4;
}

fn on_first_parameter_lesser_than_second(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_input2(&mut codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    codes[output_position] = if input1 < input2 { 1 } else { 0 }.to_string();
    *current_opcode_position += 4;
}

fn jump_if_false(mut codes: &mut Vec<String>, mut current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_input2(&mut codes, &mut current_opcode_position, current_instruction);

    if input1 == 0 {
        *current_opcode_position = input2 as usize
    } else {
        *current_opcode_position += 3;
    }
}

fn jump_if_true(mut codes: &mut Vec<String>, mut current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_input2(&mut codes, &mut current_opcode_position, current_instruction);
    if input1 != 0 {
        *current_opcode_position = input2 as usize
    } else {
        *current_opcode_position += 3;
    }
}

fn get_input(device_id: &str, mut codes: &mut Vec<String>, current_opcode_position: &mut usize) -> () {
    let output_position = get_number(&mut codes, &current_opcode_position, 1);
    codes[output_position] = String::from(device_id);
    *current_opcode_position += 2;
}

fn multiply(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_input2(&mut codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    codes[output_position] = (input1 * input2).to_string();
    *current_opcode_position += 4;
}

fn add(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_input2(&codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    codes[output_position] = (input1 + input2).to_string();
    *current_opcode_position += 4;
}

fn get_number(codes: &mut Vec<String>, current_opcode_position: &usize, relative_index_from_opcode: usize) -> usize {
    codes[current_opcode_position + relative_index_from_opcode].parse::<usize>().unwrap()
}

fn get_input1(codes: &Vec<String>, current_opcode_position: &usize, current_instruction: &usize) -> i32 {
    let input_mode = (current_instruction / 100) % 10;
    let input_parameter = codes[current_opcode_position + 1].parse::<i32>().unwrap();
    get_input_by_mode(codes, input_mode, input_parameter as usize)
}

fn get_input2(codes: & Vec<String>, current_opcode_position: &usize, current_instruction: &usize) -> i32 {
    let input_mode = (current_instruction / 1000) % 10;
    let input_parameter = codes[current_opcode_position + 2].parse::<i32>().unwrap();
    get_input_by_mode(codes, input_mode, input_parameter as usize)
}

fn get_input_by_mode(codes: &Vec<String>, input_mode: usize, input_parameter: usize) -> i32 {
    match input_mode {
        0 => {
            codes[input_parameter as usize].parse::<i32>().unwrap()
        },
        1 => {
            input_parameter as i32
        },
        _ => {
            panic!("Invalid position mode {}", input_mode)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode;

    #[test]
    fn uses_input_mode() {
        let (codes, _) = intcode(String::from("1002,4,3,4,33").as_str(), "1");
        assert_eq!(codes.join(","), "1002,4,3,4,99");
    }

    #[test]
    fn returns_output_for_test_with_opcode_8_using_position_mode() {
        let (codes, output) = intcode(
            String::from("3,9,8,9,10,9,4,9,99,-1,8").as_str(), "8");
        assert_eq!(output, 1);

        let (codes, output) = intcode(
            String::from("3,9,8,9,10,9,4,9,99,-1,8").as_str(), "1");
        assert_eq!(output, 0);
    }

    #[test]
    fn returns_output_for_test_with_opcode_8_using_immediate_mode() {
        let (codes, output) = intcode(
            String::from("3,3,1108,-1,8,3,4,3,99").as_str(), "8");
        assert_eq!(output, 1);

        let (codes, output) = intcode(
            String::from("3,3,1108,-1,8,3,4,3,99").as_str(), "1");
        assert_eq!(output, 0);
    }

    #[test]
    fn returns_output_for_test_with_opcode_7_using_position_mode() {
        let (codes, output) = intcode(
            String::from("3,9,7,9,10,9,4,9,99,-1,8").as_str(), "7");
        assert_eq!(output, 1);

        let (codes, output) = intcode(
            String::from("3,9,7,9,10,9,4,9,99,-1,8").as_str(), "8");
        assert_eq!(output, 0);
    }

    #[test]
    fn returns_output_for_test_with_opcode_7_using_immediate_mode() {
        let (codes, output) = intcode(
            String::from("3,3,1107,-1,8,3,4,3,99").as_str(), "7");
        assert_eq!(output, 1);

        let (codes, output) = intcode(
            String::from("3,3,1107,-1,8,3,4,3,99").as_str(), "8");
        assert_eq!(output, 0);
    }

    #[test]
    fn returns_output_for_test_jumps_using_position_mode() {
        let (codes, output) = intcode(
            String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").as_str(), "7");
        assert_eq!(output, 1);

        let (codes, output) = intcode(
            String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9").as_str(), "0");
        assert_eq!(output, 0);
    }

    #[test]
    fn returns_output_for_test_jumps_using_immediate_mode() {
        let (codes, output) = intcode(
            String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").as_str(), "7");
        assert_eq!(output, 1);

        let (codes, output) = intcode(
            String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").as_str(), "0");
        assert_eq!(output, 0);
    }
}