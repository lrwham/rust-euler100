use rust_euler100::problems::*;

use std::io;

fn main() {
    loop {
        println!("Enter 'q' to quit.");
        println!("Select Euler problem to run (1 or 2):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => euler_problem_001::solve(),
            "2" => euler_problem_002::solve(),
            "q" | "Q" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid selection. Please enter 1 or 2."),
        }
    }
}
