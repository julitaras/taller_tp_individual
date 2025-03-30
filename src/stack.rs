#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    data: Vec<i16>,
    max_size: usize,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Stack {
            data: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, val: i16) -> Result<(), String> {
        if self.data.len() >= self.max_size {
            return Err("stack-overflow".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<i16, String> {
        self.data.pop().ok_or_else(|| "stack-underflow".to_string())
    }

    pub fn to_vec(&self) -> Vec<i16> {
        self.data.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new(5);
        // Empujar valores
        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());
        // Pop y verificar el valor
        assert_eq!(stack.pop(), Ok(20));
        assert_eq!(stack.pop(), Ok(10));
        // Prueba de underflow
        assert_eq!(stack.pop(), Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new(2);
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        // Este push debe fallar por overflow
        assert_eq!(stack.push(3), Err("stack-overflow".to_string()));
    }
}