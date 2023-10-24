use ruint::aliases::U256;

pub const STACK_LIMIT: usize = 1024;

pub struct Stack {
    data: Vec<U256> 
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(STACK_LIMIT),
        }
    }

    // should push return the stack?
    pub fn push(&mut self, item: U256) {
        if self.data.len() == STACK_LIMIT {
            panic!("Stack overflow");
        }
        self.data.push(item);
    }

    pub fn pop(&mut self) -> U256 {
        if self.data.len() == 0 {
            // handle this better
            panic!("Stack underflow");
        }

        self.data.pop().unwrap()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use ruint::Uint;
    use super::*;

    #[test]
    fn test_len() {
        let mut stack = Stack::new();
        stack.push(Uint::from(1));
        stack.push(Uint::from(1));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(Uint::from(69));
        let res = stack.pop();
        assert_eq!(res, Uint::from(69));
    }

    #[test]
    #[should_panic]
    fn test_underflow() {
        let mut stack = Stack::new();
        stack.pop();
    }

    #[test]
    #[should_panic]
    fn test_overflow() {
        let mut stack = Stack::new();
        for _i in 0..1025 {
            stack.push(Uint::from(1));
        }
        println!("{}", stack.len());
        assert!(stack.len() <= STACK_LIMIT);
    }
}