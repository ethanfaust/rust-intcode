#[derive(Debug, PartialEq, Clone)]
pub struct IntCodeProgram {
    pub program: Vec<i64>
}

impl IntCodeProgram {
    pub fn from_string(input: String) -> IntCodeProgram {
        let memory: Vec<i64> = input.split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        return IntCodeProgram {
            program: memory
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let program = IntCodeProgram::from_string(String::from("1,0,0,0,99"));
        assert_eq!(vec![1,0,0,0,99], program.program);
    }
}
