pub use self::execution::IntCodeExecution;
pub use self::pipe::{Pipe, PipeRef};

mod execution;
mod instruction;
mod operation;
mod pipe;
