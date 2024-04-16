# Runge-Kutta 4th Order Method

This Rust project implements the Runge-Kutta 4th Order method for solving initial value problem (IVP) and the Burgers equation for fluid flow. <br>

## Overview
The Runge-Kutta 4th Order method is a numerical technique used to approximate the solution of ordinary differential equations (ODEs). It is particularly useful for solving initial value problems where the value of the function and its derivative(s) at a single point are known.<br>

The Burgers equation is a fundamental partial differential equation in fluid mechanics that describes the behavior of viscous fluids. The solver provided in this project utilizes the Runge-Kutta 4th Order method to approximate the solution of the Burgers equation. <br>

## Source Files <br>
[rk4_ivp.rs](https://github.com/abhinavtk/flaxandteal-exercise/blob/main/src/rk4_ivp.rs) - Contains the implementation of the Runge-Kutta 4th Order method for solving initial value problems.  <br>
[rk4_burgers.rs](https://github.com/abhinavtk/flaxandteal-exercise/blob/main/src/rk4_burgers.rs) - Contains the implementation of the Runge-Kutta 4th Order method for solving the Burgers equation for fluid flow. <br>

**Usage** <br>
The following steps will allow you to run this Rust project and see outputs.
1. Clone the repository to your local machine:  
```
git clone https://github.com/abhinavtk/flaxandteal-exercise.git
```
2. Navigate to the project directory: 
```
cd flaxandteal-exercise
```
3. Compile and run the project:
```
cargo run
```
You will be prompted to choose an option:  <br>
&emsp;Enter 1 to solve an initial value problem (IVP) using RK4. <br>
&emsp;Enter 2 to solve Burger's equation using RK4. <br>
4. Follow the on-screen instructions to input your choice and view the results. <br>
