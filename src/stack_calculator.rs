#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorError {
    EmptyStack,
    InsufficientOperands,
    DivisionByZero,
    StackOverflow,
}

#[derive(Debug, Clone)]
pub struct StackCalculator {
    stack: Vec<i32>,
    max_size: usize,
}

impl StackCalculator {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            max_size: 1000,
        }
    }

    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            stack: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, value: i32) -> Result<(), CalculatorError> {
        if self.stack.len() >= self.max_size {
            return Err(CalculatorError::StackOverflow);
        }
        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<i32, CalculatorError> {
        self.stack.pop().ok_or(CalculatorError::EmptyStack)
    }

    pub fn peek(&self) -> Result<i32, CalculatorError> {
        self.stack.last().copied().ok_or(CalculatorError::EmptyStack)
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn add(&mut self) -> Result<(), CalculatorError> {
        if self.stack.len() < 2 {
            return Err(CalculatorError::InsufficientOperands);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a + b);
        Ok(())
    }

    pub fn subtract(&mut self) -> Result<(), CalculatorError> {
        if self.stack.len() < 2 {
            return Err(CalculatorError::InsufficientOperands);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a - b);
        Ok(())
    }

    pub fn multiply(&mut self) -> Result<(), CalculatorError> {
        if self.stack.len() < 2 {
            return Err(CalculatorError::InsufficientOperands);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a * b);
        Ok(())
    }

    pub fn divide(&mut self) -> Result<(), CalculatorError> {
        if self.stack.len() < 2 {
            return Err(CalculatorError::InsufficientOperands);
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        
        if b == 0 {
            self.stack.push(a);
            self.stack.push(b);
            return Err(CalculatorError::DivisionByZero);
        }
        
        self.stack.push(a / b);
        Ok(())
    }
}

impl Default for StackCalculator {
    fn default() -> Self {
        Self::new()
    }
}

