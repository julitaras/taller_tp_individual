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

    pub fn peek(&self) -> Result<i16, String> {
        self.data
            .last()
            .copied()
            .ok_or_else(|| "stack-underflow".to_string())
    }

    pub fn peek_n(&self, n: usize) -> Result<i16, String> {
        if self.data.len() > n {
            Ok(self.data[self.data.len() - 1 - n])
        } else {
            Err("stack-underflow".to_string())
        }
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

    #[test]
    fn test_peek() {
        let mut stack = Stack::new(10);
        // Prueba peek en stack con un solo elemento.
        stack.push(42).unwrap();
        assert_eq!(stack.peek().unwrap(), 42);
    }

    #[test]
    fn test_peek_n() {
        let mut stack = Stack::new(10);
        // La pila: [1, 2, 3] (1 es el fondo y 3 el tope)
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        // peek_n(0) debería devolver el tope: 3.
        assert_eq!(stack.peek_n(0).unwrap(), 3);
        // peek_n(1) debe devolver 2.
        assert_eq!(stack.peek_n(1).unwrap(), 2);
        // peek_n(2) debe devolver 1.
        assert_eq!(stack.peek_n(2).unwrap(), 1);
    }

    #[test]
    fn test_peek_n_underflow() {
        let stack = Stack::new(10);
        // Si la pila está vacía, peek_n(0) debe devolver un error.
        assert!(stack.peek_n(0).is_err());
    }
}
