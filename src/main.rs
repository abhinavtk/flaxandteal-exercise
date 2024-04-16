
// Import the `rk4_ivp` module
mod rk4_ivp;
use rk4_ivp::rk4;

// Import the `rk4_burgers` module
mod rk4_burgers;
use rk4_burgers::solve_burgers;


fn main() {
    // Call the function from rk4
    println!("\nSolving IVP Using RK4...");
    rk4();
    

    println!("\nSolving Burger's equation using RK4...");
    solve_burgers();

}