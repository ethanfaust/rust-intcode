#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Add,
    Multiply,
    Read,
    Write,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt
}

impl Operation {
    pub fn from_i64(input: i64) -> Option<Operation> {
        match input {
            1 => Some(Operation::Add),
            2 => Some(Operation::Multiply),
            3 => Some(Operation::Read),
            4 => Some(Operation::Write),
            5 => Some(Operation::JumpIfTrue),
            6 => Some(Operation::JumpIfFalse),
            7 => Some(Operation::LessThan),
            8 => Some(Operation::Equals),
            99 => Some(Operation::Halt),
            _ => None
        }
    }

    pub fn argument_count(&self) -> usize {
        match *self {
            Operation::Add => 3,
            Operation::Multiply => 3,
            Operation::Read => 1,
            Operation::Write => 1,
            Operation::JumpIfTrue => 2,
            Operation::JumpIfFalse => 2,
            Operation::LessThan => 3,
            Operation::Equals => 3,
            Operation::Halt => 0
        }
    }
}
