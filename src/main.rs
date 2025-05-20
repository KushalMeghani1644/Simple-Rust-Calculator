use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parse_number(input: &str, message: &str) -> Option<i32> {
    match input.trim().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("{}", message);
            None
        }
    }
}

fn main() {
    println!("-----SIMPLE-CALCULATOR-----");
    let repeat = loop {
        let input = read_input("Enter how many times do you want to run the calculator: ");
        match input.trim().parse::<u32>() {
            Ok(n) => break n,
            Err(_) => println!("Please enter a valid positive number"),
        }
    };
    for _ in 0..repeat {
        let num1 = loop {
            let input = read_input("Please enter the first number: ");
            if let Some(n) = parse_number(&input, "That's not a valid number!") {
                break n;
            }
        };
        let num2 = loop {
            let input = read_input("Please enter the second number: ");
            if let Some(n) = parse_number(&input, "That's not a valid number!") {
                break n;
            }
        };
        let operator = loop {
            let op = read_input("Please enter an operator (+,-,*,/,%)");
            match op.as_str() {
                "+" | "-" | "*" | "/" | "%" => break op,
                _ => println!("Invalid operator! Please try again"),
            }
        };
        match operator.as_str() {
            "+" => println!("The sum of {} and {} is {}", num1, num2, num1 + num2),
            "-" => println!("The difference of {} and {} is {}", num1, num2, num1 - num2),
            "*" => println!("The product of {} and {} is {}", num1, num2, num1 * num2),
            "/" => {
                if num2 == 0 {
                    println!("Cannot divide by zero!");
                } else {
                    println!("The quotient of {} and {} is {}", num1, num2, num1 / num2);
                }
            }
            "%" => {
                if num2 == 0 {
                    println!("Cannot divide by zero!");
                } else {
                    println!("The remainder of {} and {} is {}", num1, num2, num1 % num2);
                }
            }
            _ => unreachable!(),
        }
        println!("---------------------------------------");
    }
    println!("Thanks for using the calculator!");
}
