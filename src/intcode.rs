use num::pow;

static DEBUG_SHOW_INPUT_OUTPUT: bool = false;
static DEBUG_EACH_OPERATION_OUTPUT: bool = false;
static TRACE: bool = false;

pub struct IntCode {
    program: Vec<i64>,
    pub current_opcode_position: usize,
    pub current_instruction: i64,
    input: Option<i64>,
    output: Vec<i64>,
    relative_base: i64,
    should_stop_at_memory_not_available: bool,
    immediate_output_mode: bool
}

impl IntCode {
    pub fn initialize(program: &str, input: Option<i64>, immediate_output_mode: bool) -> IntCode {
        IntCode {
            program: program
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
            current_opcode_position: 0,
            current_instruction: 0,
            input,
            output: Vec::new(),
            relative_base: 0,
            should_stop_at_memory_not_available: false,
            immediate_output_mode
        }
    }

    fn get_instruction_parameter(&mut self, position: usize) -> i64 {
        let input_mode = (self.current_instruction / (10 * pow(10, position))) % 10;
        let input_parameter = self.get_memory(self.current_opcode_position + position);

        match input_mode {
            0 => self.get_memory(input_parameter as usize),
            1 => input_parameter as i64,
            2 => self.get_memory((self.relative_base + input_parameter) as usize),
            _ => panic!("Invalid position mode {}", input_mode),
        }
    }

    fn get_output_position(&mut self, instruction_position: usize) -> usize {
        let output_mode = (self.current_instruction / (10 * pow(10, instruction_position as usize))) % 10;
        let output_parameter = self.get_memory(self.current_opcode_position + instruction_position);

        match output_mode {
            0 => return output_parameter as usize,
            2 => return
                (self.relative_base + output_parameter) as usize,
            _ => panic!("Invalid position mode {}", output_mode),
        }
    }

    fn get_memory(&mut self, index: usize) -> i64 {
        self.make_sure_index_exists(index);
        self.program[index]
    }

    fn store_memory(&mut self, index: usize, value: i64) {
        self.make_sure_index_exists(index);
        self.program[index] = value
    }

    fn make_sure_index_exists(&mut self, index: usize) {
        let index_out_of_bounds = index >= self.program.len();

        if index_out_of_bounds && !self.should_stop_at_memory_not_available {
            self.pad_memory(index);
        }
    }

    fn pad_memory(&mut self, index: usize) {
        let pad_length = index - (self.program.len() - 1);
        let mut pad_vector: Vec<i64> = vec![0; pad_length];
        self.program.append(&mut pad_vector);
    }

    pub fn execute(&mut self) {
        let index_out_of_bounds = self.current_opcode_position >= self.program.len();
        let continue_iteration: bool = !index_out_of_bounds || !self.should_stop_at_memory_not_available;

        while continue_iteration
            && self.get_memory(self.current_opcode_position) != 99 {
            self.current_instruction = self.get_memory(self.current_opcode_position);

            if DEBUG_EACH_OPERATION_OUTPUT { println!("OPCODE {}", self.current_instruction) }
            if DEBUG_EACH_OPERATION_OUTPUT { println!("POSITION {}", self.current_opcode_position) }

            match self.current_instruction % 10 {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.store_input(),
                4 => if self.store_output() { return; },
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.on_first_parameter_lesser_than_second(),
                8 => self.on_both_parameters_equal(),
                9 => self.adjust_relative_base(),
                _ => {
                    unreachable!("Invalid opcode")
                }
            }
//            if DEBUG_EACH_OPERATION_OUTPUT { self.print_memory() }

        }
    }

    fn add(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.get_output_position(3);

        self.store_memory(output_position, input1 + input2);
        self.current_opcode_position += 4;
    }

    fn multiply(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.get_output_position(3);

        self.store_memory(output_position, input1 * input2);
        self.current_opcode_position += 4;
    }

    fn store_input(&mut self) {
        let output_position = self.get_output_position(1);

        if self.input.is_none() { panic!(dbg!("No input")) }
        if TRACE { println!("Storing {} in {}", self.input.unwrap(), output_position)}
        self.store_memory(output_position, self.input.unwrap());
        self.input = None;
        self.current_opcode_position += 2;
    }

    fn store_output(&mut self) -> bool {
        let output = self.get_instruction_parameter(1);

        self.current_opcode_position += 2;
        let next_command = &self.get_memory(self.current_opcode_position);
        self.output.push(output);

        if self.immediate_output_mode {
            return true
        }

        // TODO Add immediate output mode
        if output != 0 && *next_command != 99 && false {
            panic!("It can't be non-zero")
        }
        *next_command == 99
    }

    fn jump_if_true(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);

        if TRACE { println!("Jumping from {} to {}", self.current_opcode_position, input2)}
        self.current_opcode_position =
            if input1 != 0 { input2 as usize } else { self.current_opcode_position + 3 }

    }

    fn jump_if_false(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);

        if TRACE { println!("Jumping from {} to {}", self.current_opcode_position, input2)}

        self.current_opcode_position =
            if input1 == 0 { input2 as usize } else { self.current_opcode_position + 3 } as usize
    }

    fn on_first_parameter_lesser_than_second(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.get_output_position(3);

        self.store_memory(output_position, if input1 < input2 { 1 } else { 0 });
        self.current_opcode_position += 4;
    }

    fn on_both_parameters_equal(&mut self) {
        let input1 = self.get_instruction_parameter(1);
        let input2 = self.get_instruction_parameter(2);
        let output_position = self.get_output_position(3);

        self.store_memory(output_position, if input1 == input2 { 1 } else { 0 });
        self.current_opcode_position += 4;
    }

    fn adjust_relative_base(&mut self) {
        let input1 = self.get_instruction_parameter(1);

        self.relative_base += input1;
        self.current_opcode_position += 2
    }

    pub fn set_input(&mut self, input: i64) {
        self.input = Some(input);
    }

    pub fn has_output(&self) -> bool {
        self.output.len() != 0
    }

    pub fn take_output(&mut self) -> Vec<i64>{
        let temp = self.output.clone();
        self.output = vec![];
        temp
    }

    fn print_memory(&self) {
        println!("{}", self.program.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }

    fn memory_string(&self) -> String {
        format!("{}", self.program.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }

    pub fn output_string(&self) -> String {
        format!("{}", self.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::IntCode;

    #[test]
    fn uses_input_mode() {
        let mut intcode = IntCode::initialize(String::from("1002,4,3,4,33").as_str(), Some(1), false);
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
        assert_eq!(intcode_output("3,9,7,9,10,9,4,9,99,-1,8", 7), 1);
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

    #[test]
    fn relative_mode_works() {
        let intcode = intcode_execute("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", None);
        assert_eq!(intcode.output_string(),
                   "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")
    }

    #[test]
    fn  works_for_big_numbers() {
        let intcode = intcode_execute("1102,34915192,34915192,7,4,7,99,0", None);
        assert_eq!(intcode.output_string(), "1219070632396864");

        let intcode = intcode_execute("104,1125899906842624,99", None);
        assert_eq!(intcode.output_string(), "1125899906842624")
    }

    pub fn intcode_output(program: &str, input: i64) -> i64 {
        let intcode = intcode_execute(program, Some(input));
        intcode.output[0]
    }

    pub fn intcode_execute(program: &str, input: Option<i64>) -> IntCode {
        let mut intcode = IntCode::initialize(String::from(program).as_str(), input, false);
        intcode.execute();
        intcode
    }

    pub fn intcode_memory(program: &str, input: Option<i64>) -> String {
        let intcode = intcode_execute(program, input);
        intcode.memory_string()
    }
}