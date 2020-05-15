use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
pub struct Pipe {
    data: Vec<i64>,
    read_pos: usize
}

pub type PipeRef = Rc<RefCell<Pipe>>;

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            data: Vec::new(),
            read_pos: 0
        }
    }

    pub fn new_ref() -> PipeRef {
        Rc::new(RefCell::new(Pipe::new()))
    }

    pub fn write(&mut self, val: i64) {
        self.data.push(val);
    }

    pub fn can_read(&self) -> bool {
		if self.data.len() == 0 {
			return false;
		}
        return self.read_pos <= self.data.len() - 1;
    }

    pub fn read(&mut self) -> i64 {
        if !self.can_read() {
            panic!("called read but no data available");
        }
        let val = self.data[self.read_pos];
        self.read_pos += 1;
        return val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
		let mut p: Pipe = Pipe::new();
		assert_eq!(false, p.can_read());
		p.write(1);
		assert_eq!(true, p.can_read());
		let val = p.read();
		assert_eq!(1, val);
	}

	#[test]
	fn test_buildup() {
		let mut p: Pipe = Pipe::new();
		p.write(5);
		p.write(4);
		p.write(3);
		assert_eq!(true, p.can_read());
		assert_eq!(5, p.read());
		assert_eq!(4, p.read());
		assert_eq!(3, p.read());
		assert_eq!(false, p.can_read());
	}
}
