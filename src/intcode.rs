
use num::pow;

static DEBUG_SHOW_INPUT_OUTPUT : bool = true;
static DEBUG_EACH_OPERATION_OUTPUT : bool = true;

struct IntCode {
    program: Vec<i64>,
    current_opcode_position: usize,
    current_instruction: i64,
    input: i64,
    output: Option<i64>,
}

impl IntCode {
    fn initialize(program: &str, input: i64) -> IntCode {
        IntCode {
            program: program
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            current_opcode_position: 0,
            current_instruction: 0,
            input,
            output: None,
        }
    }

    fn get_instruction_parameter(&self, position: usize) -> i64 {
        let input_mode = (self.current_instruction / (10 * pow(10, position))) % 10;
        let input_parameter = self.program[self.current_opcode_position + position];

        match input_mode {
            0 => self.program[input_parameter as usize],
            1 => input_parameter as i64,
            _ => panic!("Invalid position mode {}", input_mode),
        }
    }


    fn execute(&mut self) {

        while (self.current_opcode_position < self.program.len() - 1)
            && self.program[self.current_opcode_position] != 99 {
            self.current_instruction = self.program[self.current_opcode_position];

            match self.current_instruction % 10 {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.store_input(),
                4 => if self.store_output() { return },
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.on_first_parameter_lesser_than_second(),
                8 => self.on_both_parameters_equal(),
                _ => {
                    panic!("Invalid opcode")
                }
            }
            if DEBUG_EACH_OPERATION_OUTPUT {self.print_memory()}
        }
    }

    fn add(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.program[self.current_opcode_position + 3] as usize;

        self.program[output_position] = input1 + input2;
        self.current_opcode_position += 4;
    }

    fn multiply(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.program[self.current_opcode_position + 3] as usize;

        self.program[output_position] = input1 * input2;
        self.current_opcode_position += 4;
    }

    fn store_input(&mut self) {
        let output_position = self.program[self.current_opcode_position + 1] as usize as usize;

        self.program[output_position] = self.input;
        self.current_opcode_position += 2;
    }

    fn store_output(&mut self) -> bool {
        let input = self.get_instruction_parameter(1);

        self.current_opcode_position += 2;
        let next_command = &self.program[self.current_opcode_position];

        println!("{}", input);
        if input != 0 && *next_command != 99 {
            panic!("It can't be non-zero")
        }
        self.output = Some(input);
        *next_command == 99
    }

    fn jump_if_true(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        self.current_opcode_position =
            if input1 != 0 { input2 as usize } else { self.current_opcode_position + 3 }
    }

    fn jump_if_false(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        self.current_opcode_position =
            if input1 == 0 { input2 as usize } else { self.current_opcode_position + 3 } as usize
    }

    fn on_first_parameter_lesser_than_second(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.program[self.current_opcode_position + 3] as usize;

        self.program[output_position] = if input1 < input2 { 1 } else { 0 };
        self.current_opcode_position += 4;
    }

    fn on_both_parameters_equal(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.program[self.current_opcode_position + 3] as usize;

        self.program[output_position] = if input1 == input2 { 1 } else { 0 };
        self.current_opcode_position += 4;
    }

    fn print_memory(&self) {
        println!("{}", self.program.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }

    fn memory_string(&self) -> String{
        format!("{}", self.program.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::IntCode;

    fn intcode_output(program: &str, input: i64) -> i64 {
        let mut intcode = IntCode::initialize(String::from(program).as_str(), input);
        intcode.execute();
        let output = intcode.output.unwrap();
        output
    }

    #[test]
    fn uses_input_mode() {
        let mut intcode = IntCode::initialize(String::from("1002,4,3,4,33").as_str(), 1);
        intcode.execute();

        assert_eq!(intcode.memory_string(), "1002,4,3,4,99");
    }

    #[test]
    fn returns_output_for_test_with_opcode_8_using_position_mode() {
        assert_eq!(intcode_output("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);
        assert_eq!(intcode_output("3,9,8,9,10,9,4,9,99,-1,8", 1), 0);
    }

    #[test]
    fn returns_output_for_test_with_opcode_8_using_immediate_mode() {
        assert_eq!(intcode_output("3,3,1108,-1,8,3,4,3,99", 8), 1);
        assert_eq!(intcode_output("3,3,1108,-1,8,3,4,3,99", 1), 0);
    }

    #[test]
    fn returns_output_for_test_with_opcode_7_using_position_mode() {
        assert_eq!(intcode_output("3,9,7,9,10,9,4,9,99,-1,8", 7),1);
        assert_eq!(intcode_output("3,9,7,9,10,9,4,9,99,-1,8", 8), 0);
    }

    #[test]
    fn returns_output_for_test_with_opcode_7_using_immediate_mode() {
        assert_eq!(intcode_output("3,3,1107,-1,8,3,4,3,99", 7), 1);
        assert_eq!(intcode_output("3,3,1107,-1,8,3,4,3,99", 8), 0);
    }

    #[test]
    fn returns_output_for_test_jumps_using_position_mode() {
        assert_eq!(intcode_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 7), 1);
        assert_eq!(intcode_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0), 0);
    }

    #[test]
    fn returns_output_for_test_jumps_using_immediate_mode() {
        assert_eq!(intcode_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 7), 1);
        assert_eq!(intcode_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0), 0);
    }
}