
use std::io;

fn basic_calculator() {
    
    let mut input1 = String::new();
    println!("Enter an integer1:\n");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: i128 = input1.trim().parse().expect("Invalid input");
    
    let mut input2 = String::new();
    println!("Enter an integer2:\n");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: i128 = input2.trim().parse().expect("Invalid input");
    
    println!("The numbers you entered are: {} {}\n", num1, num2);
    println!("Enter the operation you want to perform:\n");
    println!("A = Addition\nB = Subtraction\nC = Multiplication\nD = Division\n");
    
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let op: char = input3.trim().parse().expect("Invalid input");
    
    println!("The option you entered was: {}", op);
    
    if op == 'A' {
        println!("Addition: {}", num1 + num2);
    }
    else if op == 'B' {
        println!("Subtraction: {}", num1 - num2);
    }
    else if op == 'C' {
        println!("Multiplication: {}", num1 * num2);
    }
    else if op == 'D' {
        println!("Division {}", num1/num2);
    }
    else {
        println!("Invalid option!");
    }
}


fn main() {
    basic_calculator();
}