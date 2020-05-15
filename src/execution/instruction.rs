use super::operation::Operation;
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub arguments: Vec<Argument>
}

impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "INST {:?} {:?}", self.operation, self.arguments)
    }
}

#[derive(PartialEq, Clone)]
pub struct Argument {
    pub value: i64,
    pub parameter_mode: ParameterMode
}

impl fmt::Debug for Argument {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.parameter_mode {
            ParameterMode::Position => write!(fmt, "mem[{}]", self.value),
            ParameterMode::Immediate => write!(fmt, "i{}", self.value)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterMode {
    Position,
    Immediate
}

impl ParameterMode {
    pub fn from_i64(input: i64) -> Option<ParameterMode> {
        match input {
            0 => Some(ParameterMode::Position),
            1 => Some(ParameterMode::Immediate),
            _ => None
        }
    }
}
