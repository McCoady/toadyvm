use crate::memory::Memory;
use crate::stack::Stack;

pub struct Runner {
    code: String,
    stack: Stack,
    memory: Memory,
    pc: usize,
}

impl Runner {
    pub fn from_code(code: String) -> Self {
        Self {
            code,
            stack: Stack::new(),
            memory: Memory::new(),
            pc: 0
        }
    }
}

#[cfg(test)]
mod tests{
    use ruint::Uint;
    use super::*;

    #[test]
    fn test_setup() {
        let bytecode = String::from("6001600101");
        let mut runner = Runner::from_code(bytecode);
        assert_eq!(runner.stack.len(), 0);
    }

    #[test]
    fn test_runner_stack() {
        let bytecode = String::from("6001600101");
        let mut runner = Runner::from_code(bytecode);
        runner.stack.push(Uint::from(1));
        assert_eq!(runner.stack.len(), 1);
    }

    #[test]
    fn test_runner_pc() {
        let bytecode = String::from("6001600101");
        let mut runner = Runner::from_code(bytecode);
        assert_eq!(runner.pc, 0);
    }
}