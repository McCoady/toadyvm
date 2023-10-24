use ruint::Uint;
use ruint::aliases::U256;

pub struct Memory {
    data: Vec<U256>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    // how to stretch vector to necessary size
    pub fn store(&mut self, offset: usize, item: U256) {
        if offset + 1 > self.data.len() {
            let slots_needed = offset - self.data.len() + 1;
            self.data.extend(vec![Uint::from(0); slots_needed]);
        }
        self.data[offset] = item;
    }

    pub fn load(&self, offset: usize) -> U256 {
        if offset > self.data.len() {
            panic!("Offset OOB");
        }
        self.data[offset]
    }
}

#[cfg(test)]
mod tests {
    use ruint::Uint;

    use super::*;

    #[test]
    fn test_store_at_zero() {
        let mut memory = Memory::new();
        let val = Uint::from(42);
        memory.store(0, val);
        let val_out = memory.load(0);
        assert_eq!(val, val_out);
    }

    #[test]
    fn test_store_at_one_hundred() {
        let mut memory = Memory::new();
        let val = Uint::from(42);
        memory.store(100, val);
        let val_out = memory.load(100);
        assert_eq!(val, val_out);
    }

    #[test]
    fn test_empty_value() {
        let mut memory = Memory::new();
        let val = Uint::from(42);
        memory.store(10, val);
        let empty_val_out = memory.load(9);
        assert_eq!(empty_val_out, Uint::from(0));
    }
    
    #[test]
    #[should_panic]
    fn test_oob_offset() {
        let mut memory = Memory::new();
        let val = Uint::from(69);
        memory.store(10, val);
        memory.load(100);
    }
}