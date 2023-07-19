// Exercise 4
// Implement logic :
// Run tests
// Enum representing arithmetic operations

fn main() {
    
}
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Perform arithmetic operations
fn perform_operation(operation: Operation, num1: f64, num2: f64) -> f64 {
    match operation {
       Operation::Add => num1 + num2 ,
       Operation::Subtract => num1 - num2 ,
       Operation::Multiply => num1 * num2 ,
       Operation::Divide => num1 / num2 ,
    }
}