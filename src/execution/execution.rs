use crate::program::IntCodeProgram;
use super::operation::Operation;
use super::instruction::{Argument, Instruction, ParameterMode};
use super::pipe::{Pipe, PipeRef};

#[derive(Debug, PartialEq, Clone)]
pub struct IntCodeExecution {
    pub memory: Vec<i64>,
    instruction_pointer: usize,
    input_pipe_ref: Option<PipeRef>,
    output_pipe_ref: Option<PipeRef>
}

impl IntCodeExecution {
    pub fn new(program: IntCodeProgram) -> IntCodeExecution {
        IntCodeExecution {
            memory: program.program.clone(),
            instruction_pointer: 0,
            input_pipe_ref: None,
            output_pipe_ref: None
        }
    }

    pub fn set_input_pipe(&mut self, input_pipe: &PipeRef) {
        self.input_pipe_ref = Some(input_pipe.clone());
    }

    pub fn set_output_pipe(&mut self, output_pipe: &PipeRef) {
        self.output_pipe_ref = Some(output_pipe.clone());
    }

    fn memread(&self, address: usize) -> i64 {
        return self.memory[address];
    }

    fn memwrite(&mut self, address: usize, value: i64) {
        self.memory[address] = value;
        println!("MEM[{}] = {}", address, value);
    }

    fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    fn set_instruction_pointer(&mut self, value: usize) {
        self.instruction_pointer = value;
    }

    fn debug_dump_memory(&self) {
        println!("  mem: {:?}", self.memory);
    }

    fn read_input(&mut self) -> i64 {
        let pipe_option = self.input_pipe_ref.clone()
            .expect("no input pipe set");
        let mut pipe = pipe_option.borrow_mut();
        if !pipe.can_read() {
            panic!("no input to read from input pipe");
        }
        let input = pipe.read();
        println!("READ {}", input);
        return input;
    }

    fn write_output(&mut self, val: i64) {
        let pipe_option = self.output_pipe_ref.clone()
            .expect("no output pipe set");
        let mut pipe = pipe_option.borrow_mut();
        pipe.write(val);
        println!("WROTE {}", val);
    }

    fn instruction_fetch_decode(&self, address: usize) -> Instruction {
        let mut cur = self.memread(address);
        let opcode = cur % 100;
        //println!("opcode is {}", opcode);
        let operation = Operation::from_i64(opcode);
        assert!(operation.is_some(), "invalid opcode {}", opcode);
        let operation = operation.unwrap();
        cur /= 100;
        let arg_count = operation.argument_count();
        let mut arguments: Vec<Argument> = Vec::new();
        for i in 0..arg_count {
            let mode = ParameterMode::from_i64(cur % 10)
                .expect("invalid parameter mode for parameter");
            cur /= 10;
            arguments.push(Argument {
                value: self.memory[address + 1 + i],
                parameter_mode: mode
            });
        }
        return Instruction {
            operation: operation,
            arguments: arguments
        };
    }

    fn get_mode_argument(&self, instruction: &Instruction, argument_index: usize) -> i64 {
        let argument = &instruction.arguments[argument_index];
        return match argument.parameter_mode {
            ParameterMode::Position => self.memread(argument.value as usize),
            ParameterMode::Immediate => argument.value
        };
    }

    fn get_address_argument(&self, instruction: &Instruction, argument_index: usize) -> usize {
        let argument = &instruction.arguments[argument_index];
        return argument.value as usize;
    }

    fn advance_instruction_pointer(&mut self, instruction: &Instruction) {
        let arg_count = instruction.operation.argument_count();
        self.set_instruction_pointer(self.get_instruction_pointer() + arg_count + 1);
    }

    pub fn execute(&mut self) {
        self.debug_dump_memory();
        loop {
            let mut is_jump = false;
            let instruction_pointer = self.get_instruction_pointer();
            println!("pc: {}", instruction_pointer);

            let instruction = self.instruction_fetch_decode(instruction_pointer);
            println!("{}", instruction);

            match instruction.operation {
                Operation::Add => {
                    let arg1 = self.get_mode_argument(&instruction, 0);
                    let arg2 = self.get_mode_argument(&instruction, 1);
                    let arg3 = self.get_address_argument(&instruction, 2);
                    self.memwrite(arg3, arg1 + arg2);
                },
                Operation::Multiply => {
                    let arg1 = self.get_mode_argument(&instruction, 0);
                    let arg2 = self.get_mode_argument(&instruction, 1);
                    let arg3 = self.get_address_argument(&instruction, 2);
                    self.memwrite(arg3, arg1 * arg2);
                },
                Operation::Read => {
                    let arg1 = self.get_address_argument(&instruction, 0);
                    let input = self.read_input();
                    self.memwrite(arg1, input);
                },
                Operation::Write => {
                    let arg1 = self.get_mode_argument(&instruction, 0);
                    self.write_output(arg1);
                },
                Operation::JumpIfTrue => {
                    let arg1 = self.get_mode_argument(&instruction, 0);
                    let arg2 = self.get_mode_argument(&instruction, 1);
                    if arg1 != 0 {
                        self.set_instruction_pointer(arg2 as usize);
                        is_jump = true;
                    }
                },
                Operation::JumpIfFalse => {
                    let arg1 = self.get_mode_argument(&instruction, 0);
                    let arg2 = self.get_mode_argument(&instruction, 1);
                    if arg1 == 0 {
                        self.set_instruction_pointer(arg2 as usize);
                        is_jump = true;
                    }
                },
                Operation::LessThan => {
                    let arg1 = self.get_mode_argument(&instruction, 0);
                    let arg2 = self.get_mode_argument(&instruction, 1);
                    let arg3 = self.get_address_argument(&instruction, 2);
                    if arg1 < arg2 {
                        self.memwrite(arg3, 1);
                    } else {
                        self.memwrite(arg3, 0);
                    }
                },
                Operation::Equals => {
                    let arg1 = self.get_mode_argument(&instruction, 0);
                    let arg2 = self.get_mode_argument(&instruction, 1);
                    let arg3 = self.get_address_argument(&instruction, 2);
                    if arg1 == arg2 {
                        self.memwrite(arg3, 1);
                    } else {
                        self.memwrite(arg3, 0);
                    }
                },
                Operation::Halt => {
                    println!("halted successfully");
                    break;
                }
            }
            self.debug_dump_memory();
            if !is_jump {
                self.advance_instruction_pointer(&instruction);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_add() {
        let program = IntCodeProgram::from_string(String::from("1,0,0,0,99"));
        let mut execution = IntCodeExecution::new(program);
        execution.execute();
        assert_eq!(vec![2,0,0,0,99], execution.memory);
    }

    #[test]
    fn test_basic_multiply() {
        let program = IntCodeProgram::from_string(String::from("2,3,0,3,99"));
        let mut execution = IntCodeExecution::new(program);
        execution.execute();
        assert_eq!(vec![2,3,0,6,99], execution.memory);
    }

    #[test]
    fn test_multiply2() {
        let program = IntCodeProgram::from_string(String::from("2,4,4,5,99,0"));
        let mut execution = IntCodeExecution::new(program);
        execution.execute();
        assert_eq!(vec![2,4,4,5,99,9801], execution.memory);
    }

    #[test]
    fn test_intermediate() {
        let program = IntCodeProgram::from_string(String::from("1,1,1,4,99,5,6,0,99"));
        let mut execution = IntCodeExecution::new(program);
        execution.execute();
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], execution.memory);
    }

    #[test]
    fn test_intermediate2() {
        let program = IntCodeProgram::from_string(String::from("1002,4,3,4,33"));
        let mut execution = IntCodeExecution::new(program);
        execution.execute();
        assert_eq!(vec![1002,4,3,4,99], execution.memory);
    }

    #[test]
    fn test_negative() {
        let program = IntCodeProgram::from_string(String::from("1101,100,-1,4,0"));
        let mut execution = IntCodeExecution::new(program);
        execution.execute();
        assert_eq!(vec![1101,100,-1,4,99], execution.memory);
    }

    #[test]
    fn test_output() {
        let program = IntCodeProgram::from_string(String::from("1,0,0,0,4,0,99"));
        let mut execution = IntCodeExecution::new(program);
        let output_pipe_ref = Pipe::new_ref();
        execution.set_output_pipe(&output_pipe_ref);
        execution.execute();
        assert_eq!(vec![2,0,0,0,4,0,99], execution.memory);
        {
            let mut output_pipe = output_pipe_ref.borrow_mut();
            assert_eq!(2, output_pipe.read());
            assert!(!output_pipe.can_read());
        }
    }

    #[test]
    fn test_input() {
        let program = IntCodeProgram::from_string(String::from("3,1,1001,1,1,1,99"));
        let mut execution = IntCodeExecution::new(program);
        let input_pipe_ref = Pipe::new_ref();
        execution.set_input_pipe(&input_pipe_ref);
        {
            input_pipe_ref.borrow_mut().write(4);
        }
        execution.execute();
        assert_eq!(vec![3,5,1001,1,1,1,99], execution.memory);
    }

    #[test]
    fn test_eq() {
        let program = IntCodeProgram::from_string(String::from("3,9,8,9,10,9,4,9,99,-1,8"));
        let input_pipe_ref = Pipe::new_ref();
        let output_pipe_ref = Pipe::new_ref();
        let mut execution = IntCodeExecution::new(program);
        execution.set_input_pipe(&input_pipe_ref);
        execution.set_output_pipe(&output_pipe_ref);
        {
            let mut input_pipe = input_pipe_ref.borrow_mut();
            input_pipe.write(8);
        }
        execution.execute();
        {
            let mut output_pipe = output_pipe_ref.borrow_mut();
            assert_eq!(1, output_pipe.read());
        }
    }

    fn input_output_test_wrapper(program_str: String, input: Vec<i64>, expected_output: Vec<i64>) {
        let program = IntCodeProgram::from_string(program_str);
        let mut execution = IntCodeExecution::new(program);
        let input_pipe_ref = Pipe::new_ref();
        let output_pipe_ref = Pipe::new_ref();
        execution.set_input_pipe(&input_pipe_ref);
        execution.set_output_pipe(&output_pipe_ref);
        {
            let mut input_pipe = input_pipe_ref.borrow_mut();
            for input_int in input {
                input_pipe.write(input_int);
            }
        }
        execution.execute();
        {
            let mut output_pipe = output_pipe_ref.borrow_mut();
            let mut actual_output: Vec<i64> = Vec::new();
            while output_pipe.can_read() {
                actual_output.push(output_pipe.read());
            }
            assert_eq!(expected_output, actual_output);
        }
    }

    #[test]
    fn test_eq1() {
        let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
        let input = vec![42];
        let expected_output = vec![0];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_lt_positional() {
        let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
        let input = vec![7];
        let expected_output = vec![1];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test] 
    fn test_lt_positional1() {
        let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
        let input = vec![9];
        let expected_output = vec![0];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_eq_immediate() {
        let program = String::from("3,3,1108,-1,8,3,4,3,99");
        let input = vec![8];
        let expected_output = vec![1];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_eq_immediate1() {
        let program = String::from("3,3,1108,-1,8,3,4,3,99");
        let input = vec![7];
        let expected_output = vec![0];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_lt_immediate() {
        let program = String::from("3,3,1107,-1,8,3,4,3,99");
        let input = vec![7];
        let expected_output = vec![1];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_lt_immediate1() {
        let program = String::from("3,3,1107,-1,8,3,4,3,99");
        let input = vec![8];
        let expected_output = vec![0];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_jmp() {
        let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let input = vec![0];
        let expected_output = vec![0];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_jmp_1() {
        let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let input = vec![10];
        let expected_output = vec![1];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_larger_lt() {
        let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let input = vec![4];
        let expected_output = vec![999];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_larger_eq() {
        let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let input = vec![8];
        let expected_output = vec![1000];
        input_output_test_wrapper(program, input, expected_output);
    }

    #[test]
    fn test_larger_gt() {
        let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let input = vec![10];
        let expected_output = vec![1001];
        input_output_test_wrapper(program, input, expected_output);
    }
}
