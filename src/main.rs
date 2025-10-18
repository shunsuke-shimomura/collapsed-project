mod stack_calculator;

use stack_calculator::StackCalculator;

fn main() {
    let mut calc = StackCalculator::new();
    
    calc.push(10).unwrap();
    calc.push(5).unwrap();
    calc.add().unwrap();
    
    if let Ok(result) = calc.peek() {
        println!("Result: {}", result);
    }
}
