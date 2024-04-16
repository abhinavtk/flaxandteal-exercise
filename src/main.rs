mod rk4_ivp;
use rk4_ivp::rk4;

mod rk4_burgers;
use rk4_burgers::solve_burgers;

use std::io;

fn main() {
    println!("Choose an option:");
    println!("1. Solve IVP using RK4");
    println!("2. Solve Burger's equation using RK4");

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse input to u32
    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,         // Return num if input is valid
        Err(_) => {
            println!("Invalid input. Please enter 1 or 2.");    // Print error message if input is invalid
            return;
        }
    };

    // Match user input to choose function
    match choice {
        1 => {
            println!("\nSolving IVP Using RK4 method...");
            rk4();      // Call rk4 function
        }
        2 => {
            println!("\nSolving Burger's equation using RK4 method...");
            solve_burgers();        // Call solve_burgers function
        }
        _ => println!("Invalid option. Please choose 1 or 2."),   // Print error message if input is invalid
    }
}
