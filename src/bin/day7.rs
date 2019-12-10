use permutator::Permutation;
use std::cmp::max;
use std::collections::HashMap;

static DEBUG: bool = false;

pub fn main() {
    let input = include_str!("../../data/seven.data");
    let max_thruster_signal = get_max_thruster_signal_with_feedback_amplification(input);

    println!("Max thruster signal {}", max_thruster_signal)
}

fn get_max_thruster_signal_with_one_round_of_amplification(input: &str) -> i32 {
    let codes: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
    let phase_settings = vec![0, 1, 2, 3, 4];
    let mut max_thruster_signal = 0;
    let mut permutation = phase_settings.clone().permutation().collect::<Vec<Vec<i32>>>();
    permutation.push(phase_settings);

    permutation.iter().for_each(|phase_setting_series| {
        let mut feedback_input = 0;
        phase_setting_series.iter().for_each(|phase_setting| {
            let (_, thruster_signal, _, _) = intcode(
                codes.clone(),
                &phase_setting.to_string(),
                true,
                &feedback_input.to_string(),
                true,
                0,
            );
            feedback_input = thruster_signal;
        });
        if DEBUG { println!("Series {:?} returns {}", phase_setting_series, feedback_input) }
        max_thruster_signal = max(max_thruster_signal, feedback_input)
    });
    max_thruster_signal
}

fn get_max_thruster_signal_with_feedback_amplification(input: &str) -> i32 {
    let mut codes: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let mut permutation = phase_settings.clone().permutation().collect::<Vec<Vec<i32>>>();
    permutation.push(phase_settings);
    let mut max_thruster_signal = 0;

    permutation.iter().enumerate().for_each(|(_, phase_setting_series)| {
        let immediate_output_mode = true;
        let mut starting_index = 0;
        let mut amplifier_state: HashMap<i32, (usize, Vec<String>)> = HashMap::new();
        let mut feedback_input = 0;
        codes = input.split(',').map(|s| s.to_string()).collect();
        let mut amplifier_reference = 0;
        loop {
            amplifier_reference = amplifier_reference % 5;

            let existing_state = amplifier_state.get(&amplifier_reference);
            let is_first_round = existing_state.is_none();

            if existing_state.is_some() {
                let (place_where_execution_stopped, last_execution_sequence) =
                    existing_state.unwrap();
                starting_index = *place_where_execution_stopped as i32;
                codes = last_execution_sequence.clone()
            }
            let (halt_code, thruster_signal, halt_location, is_complete) =
                intcode(
                    codes.clone(),
                    phase_setting_series[amplifier_reference as usize].to_string().as_str(),
                    is_first_round,
                    &feedback_input.to_string(),
                    immediate_output_mode,
                    starting_index as usize,
                );
            amplifier_state.insert(amplifier_reference, (halt_location, halt_code));

            if is_complete {
                break;
            } else {
                feedback_input = thruster_signal;
                amplifier_reference += 1;
            }
        }
        max_thruster_signal = max(max_thruster_signal, feedback_input);
    });
    max_thruster_signal
}

fn intcode(
    mut codes: Vec<String>,
    phase_setting: &str,
    use_phase_setting: bool,
    feedback_input: &str,
    immediate_output_mode: bool,
    starting_index: usize,
) -> (Vec<String>, i32, usize, bool) {
    let mut current_opcode_position = starting_index;
    let mut phase_setting_fed = !use_phase_setting;

    while (current_opcode_position < codes.len() - 1)
        && codes[current_opcode_position] != String::from("99") {
        let current_instruction = &codes[current_opcode_position].parse::<usize>().unwrap();
        let current_opcode = current_instruction % 10;
        let input1 = get_parameter_1(&codes, &current_opcode_position, current_instruction);
        if DEBUG { println!("codes {:?}", codes) }

        match current_opcode {
            1 => add(&mut codes, &mut current_opcode_position, current_instruction, input1),
            2 => multiply(&mut codes, &mut current_opcode_position, current_instruction, input1),
            3 => {
                let input = get_input(phase_setting, feedback_input, &mut phase_setting_fed);
                store_input(
                    &mut codes,
                    &mut current_opcode_position,
                    input,
                )
            }
            4 => {
                current_opcode_position += 2;
                let next_command = &codes[current_opcode_position];
                if *next_command == String::from("99") {
                    return (codes, input1, current_opcode_position, false);
                }
                return (codes, input1, current_opcode_position, false);
            }
            5 => jump_if_true(&mut codes, &mut current_opcode_position, current_instruction, input1),
            6 => jump_if_false(&mut codes, &mut current_opcode_position, current_instruction, input1),
            7 => on_first_parameter_lesser_than_second(&mut codes, &mut current_opcode_position, current_instruction, input1),
            8 => on_both_parameters_equal(&mut codes, &mut current_opcode_position, current_instruction, input1),
            _ => {
                panic!("Invalid opcode")
            }
        }
    }
    return (codes, 0, current_opcode_position, true);
}

fn get_input<'a>(
    phase_setting: &'a str,
    feedback_input: &'a str,
    mut phase_setting_fed: &mut bool,
) -> &'a str {
    if *phase_setting_fed {
        feedback_input
    } else {
        *phase_setting_fed = true;
        phase_setting
    }
}

fn on_both_parameters_equal(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_parameter_2(&mut codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    if DEBUG { println!("Opcode: {}, input one: {}, output position: {}", "8", input1, output_position) }
    codes[output_position] = if input1 == input2 { 1 } else { 0 }.to_string();
    *current_opcode_position += 4;
}

fn on_first_parameter_lesser_than_second(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_parameter_2(&mut codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    if DEBUG { println!("Opcode: {}, input one: {}, output position: {}", "7", input1, output_position) }
    codes[output_position] = if input1 < input2 { 1 } else { 0 }.to_string();
    *current_opcode_position += 4;
}

fn jump_if_false(mut codes: &mut Vec<String>, mut current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_parameter_2(&mut codes, &mut current_opcode_position, current_instruction);

    if DEBUG { println!("Opcode: {}, input one: {}, input two: {}", "6", input1, input2) }
    if input1 == 0 {
        *current_opcode_position = input2 as usize
    } else {
        *current_opcode_position += 3;
    }
}

fn jump_if_true(mut codes: &mut Vec<String>, mut current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_parameter_2(&mut codes, &mut current_opcode_position, current_instruction);
    if DEBUG { println!("Opcode: {}, input one: {}, input 2: {}", "5", input1, input2) }
    if input1 != 0 {
        *current_opcode_position = input2 as usize
    } else {
        *current_opcode_position += 3;
    }
}

fn store_input(
    mut codes: &mut Vec<String>,
    current_opcode_position: &mut usize,
    input: &str,
) -> () {
    let output_position = get_number(&mut codes, &current_opcode_position, 1);
    if DEBUG { println!("Opcode: {}, input one: {}, output position: {}", "3", input, output_position) }
    codes[output_position] = String::from(input);
    *current_opcode_position += 2;
}

fn multiply(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_parameter_2(&mut codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    if DEBUG { println!("Opcode: {}, input one: {}, input two: {}, output position: {}", "2", input1, input2, output_position) }
    codes[output_position] = (input1 * input2).to_string();
    *current_opcode_position += 4;
}

fn add(mut codes: &mut Vec<String>, current_opcode_position: &mut usize, current_instruction: &usize, input1: i32) -> () {
    let input2 = get_parameter_2(&codes, &current_opcode_position, current_instruction);
    let output_position = get_number(&mut codes, &current_opcode_position, 3);

    if DEBUG { println!("Opcode: {}, input one: {}, input two: {}, output position: {}", "1", input1, input2, output_position) }
    codes[output_position] = (input1 + input2).to_string();
    *current_opcode_position += 4;
}

fn get_number(codes: &mut Vec<String>, current_opcode_position: &usize, relative_index_from_opcode: usize) -> usize {
    codes[current_opcode_position + relative_index_from_opcode].parse::<usize>().unwrap()
}

fn get_parameter_1(codes: &Vec<String>, current_opcode_position: &usize, current_instruction: &usize) -> i32 {
    let input_mode = (current_instruction / 100) % 10;
    let input_parameter = codes[current_opcode_position + 1].parse::<i32>().unwrap();
    get_input_by_mode(codes, input_mode, input_parameter as usize)
}

fn get_parameter_2(codes: &Vec<String>, current_opcode_position: &usize, current_instruction: &usize) -> i32 {
    let input_mode = (current_instruction / 1000) % 10;
    let input_parameter = codes[current_opcode_position + 2].parse::<i32>().unwrap();
    get_input_by_mode(codes, input_mode, input_parameter as usize)
}

fn get_input_by_mode(codes: &Vec<String>, input_mode: usize, input_parameter: usize) -> i32 {
    match input_mode {
        0 => {
            codes[input_parameter as usize].parse::<i32>().unwrap()
        }
        1 => {
            input_parameter as i32
        }
        _ => {
            panic!("Invalid position mode {}", input_mode)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{intcode, get_max_thruster_signal_with_one_round_of_amplification, get_max_thruster_signal_with_feedback_amplification};

    #[test]
    fn it_works() {
        let max_thruster_signal = get_max_thruster_signal_with_one_round_of_amplification(
            String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").as_str());
        assert_eq!(max_thruster_signal, 43210);

        let max_thruster_signal = get_max_thruster_signal_with_one_round_of_amplification(
            String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0").as_str());
        assert_eq!(max_thruster_signal, 54321);

        let max_thruster_signal = get_max_thruster_signal_with_one_round_of_amplification(
            String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0").as_str());
        assert_eq!(max_thruster_signal, 65210);
    }

    #[test]
    fn it_works_for_feedback_loop() {
        let max_thruster_signal = get_max_thruster_signal_with_feedback_amplification(
            String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5").as_str());
        assert_eq!(max_thruster_signal, 139629729);

        let max_thruster_signal = get_max_thruster_signal_with_feedback_amplification(
            String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10").as_str());
        assert_eq!(max_thruster_signal, 18216);
    }
}