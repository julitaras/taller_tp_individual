//! Módulo de la pila (Stack) para el intérprete Forth.
//!
//! Este módulo proporciona una estructura de datos para almacenar números enteros (`i16`) con una capacidad máxima.
//! La pila es utilizada por el intérprete Forth para realizar operaciones como empujar, sacar y consultar valores.
//!
//! # Funcionalidades principales
//! - Crear una pila con capacidad máxima (`Stack::new`).
//! - Empujar valores a la pila (`Stack::push`).
//! - Sacar valores de la pila (`Stack::pop`).
//! - Consultar el valor superior de la pila (`Stack::peek`).
//! - Consultar valores en posiciones específicas desde el tope (`Stack::peek_n`).
//! - Obtener todos los valores de la pila como un vector (`Stack::to_vec`).

/// Estructura que representa una pila con capacidad máxima.
#[derive(Debug, PartialEq)]
pub struct Stack {
    data: Vec<i16>,
    max_size: usize,
}

impl Stack {
    /// Crea una nueva pila con la capacidad máxima indicada.
    ///
    /// # Parámetros
    /// - `max_size`: Tamaño máximo de la pila.
    ///
    /// # Retorna
    /// Una nueva instancia de `Stack` con la capacidad especificada.
    pub fn new(max_size: usize) -> Self {
        Stack {
            data: Vec::new(),
            max_size,
        }
    }

    /// Empuja un valor en la pila.
    ///
    /// Si la pila ya alcanzó su capacidad máxima, se retorna un error de "stack-overflow".
    ///
    /// # Parámetros
    /// - `val`: Valor a empujar en la pila.
    ///
    /// # Retorna
    /// - `Ok(())`: Si el valor se empujó correctamente.
    /// - `Err(String)`: Si la pila está llena.
    pub fn push(&mut self, value: i16) -> Result<(), String> {
        if self.data.len() >= self.max_size {
            return Err("stack-overflow".to_string());
        }
        self.data.push(value);
        Ok(())
        // let max_elements = self.max_size / std::mem::size_of::<i16>();
        // if self.data.len() >= max_elements {
        //     return Err("stack-overflow".to_string());
        // }
        // self.data.push(value);
        // Ok(())
    }

    /// Saca el valor superior de la pila.
    ///
    /// Si la pila está vacía, se retorna un error de "stack-underflow".
    ///
    /// # Retorna
    /// - `Ok(i16)`: El valor superior de la pila.
    /// - `Err(String)`: Si la pila está vacía.
    pub fn pop(&mut self) -> Result<i16, String> {
        self.data.pop().ok_or_else(|| "stack-underflow".to_string())
    }

    /// Devuelve el valor superior de la pila sin removerlo.
    ///
    /// Si la pila está vacía, se retorna un error de "stack-underflow".
    ///
    /// # Retorna
    /// - `Ok(i16)`: El valor superior de la pila.
    /// - `Err(String)`: Si la pila está vacía.
    pub fn peek(&self) -> Result<i16, String> {
        self.data
            .last()
            .copied()
            .ok_or_else(|| "stack-underflow".to_string())
    }

    /// Devuelve una referencia al vector interno de datos.
    ///
    /// Los elementos están en el mismo orden en que fueron insertados (FIFO).
    ///
    /// # Retorna
    /// Una referencia al vector interno de la pila.
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
