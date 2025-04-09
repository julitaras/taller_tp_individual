//! Módulo de la pila (Stack) para el intérprete Forth.

/// Estructura que representa una pila con capacidad máxima.
#[derive(Debug, PartialEq)]
pub struct Stack {
    data: Vec<i16>,
    max_size: usize,
}

impl Stack {
    /// Crea una nueva pila con la capacidad máxima indicada.
    pub fn new(max_size: usize) -> Self {
        Stack {
            data: Vec::new(),
            max_size,
        }
    }

    /// Empuja un valor en la pila.
    ///
    /// Si la pila ya alcanzó su capacidad máxima, se retorna un error de "stack-overflow".
    pub fn push(&mut self, value: i16) -> Result<(), String> {
        if self.data.len() >= self.max_size {
            return Err("stack-overflow".to_string());
        }
        self.data.push(value);
        Ok(())
    }

    /// Saca el valor superior de la pila.
    ///
    /// Si la pila está vacía, se retorna un error de "stack-underflow".
    pub fn pop(&mut self) -> Result<i16, String> {
        self.data.pop().ok_or_else(|| "stack-underflow".to_string())
    }

    /// Devuelve el valor superior de la pila sin removerlo.
    ///
    /// Si la pila está vacía, se retorna un error de "stack-underflow".
    pub fn peek(&self) -> Result<i16, String> {
        self.data
            .last()
            .copied()
            .ok_or_else(|| "stack-underflow".to_string())
    }

    /// Devuelve una referencia al vector interno de datos.
    pub fn to_vec(&self) -> &[i16] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new(5);
        assert!(stack.push(10).is_ok());
        assert!(stack.push(20).is_ok());
        assert_eq!(stack.pop(), Ok(20));
        assert_eq!(stack.pop(), Ok(10));
        assert_eq!(stack.pop(), Err("stack-underflow".to_string()));
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new(2);
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert_eq!(stack.push(3), Err("stack-overflow".to_string()));
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new(10);
        stack.push(42).unwrap();
        assert_eq!(stack.peek().unwrap(), 42);
    }
}
