use std::io;
fn main() {
    //Rust code for a simple calculator
    //Taking input from user for how many times they want to run the code
    let mut num = String::new();
    println!("Enter how many times you want to run the code: ");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: u32 = num.trim().parse().expect("Please type a number!");
    for _ in 0..num {
        let mut num1 = String::new();
        let mut num2 = String::new();
        println!("Please enter the first number: ");
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        println!("Please enter the second number: ");
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");
        let num1_trimmed: i32 = num1.trim().parse().expect("Please type a number!");
        let num2_trimmed: i32 = num2.trim().parse().expect("Please type a number!");
        loop {
            let mut op = String::new();
            println!("Please enter the operation you want to perform (+ ,- , * , /, %): ");
            io::stdin()
                .read_line(&mut op)
                .expect("Expected a operator (+ , -, *, /, %)");
            let op_trimmed = op.trim();
            match op_trimmed {
                "+" => {
                    println!(
                        "The sum of {} and {} is {}",
                        num1_trimmed,
                        num2_trimmed,
                        num1_trimmed + num2_trimmed
                    );
                    break;
                }
                "-" => {
                    println!(
                        "The difference of {} and {} is {}",
                        num1_trimmed,
                        num2_trimmed,
                        num1_trimmed - num2_trimmed
                    );
                    break;
                }
                "*" => {
                    println!(
                        "The product of {} and {} is {}",
                        num1_trimmed,
                        num2_trimmed,
                        num1_trimmed * num2_trimmed
                    );
                    break;
                }
                "/" => {
                    if num2_trimmed == 0 {
                        println!("Cannot divide by zero");
                    } else {
                        println!(
                            "The quotient of {} and {} is {}",
                            num1_trimmed,
                            num2_trimmed,
                            num1_trimmed / num2_trimmed
                        );
                    }
                    break;
                }
                "%" => {
                    if num2_trimmed == 0 {
                        println!("Cannot divide by zero");
                    } else {
                        println!(
                            "The remainder of {} and {} is {}",
                            num1_trimmed,
                            num2_trimmed,
                            num1_trimmed % num2_trimmed
                        );
                    }
                    break;
                }
                _ => println!("Invalid operator"),
            }
        }
    }
}
