# Runge-Kutta 4th Order Method

This Rust project implements the Runge-Kutta 4th Order method for solving initial value problem (IVP) and the Burgers equation for fluid flow. <br>

## Overview
The Runge-Kutta 4th order (RK4) method is a widely used numerical technique for solving ordinary differential equations (ODEs). It is a higher-order method that provides relatively accurate solutions compared to simpler methods like Euler's method.<br>

## Source Files 
1. [main.rs](https://github.com/abhinavtk/flaxandteal-exercise/blob/main/src/main.rs) - Prompts the user to choose which program to execute. 
2. [rk4_ivp.rs](https://github.com/abhinavtk/flaxandteal-exercise/blob/main/src/rk4_ivp.rs) - Contains the implementation of the Runge-Kutta 4th Order method for solving an initial value problem. 
3. [rk4_burgers.rs](https://github.com/abhinavtk/flaxandteal-exercise/blob/main/src/rk4_burgers.rs) - Contains the implementation of the Runge-Kutta 4th Order method for solving the Burgers equation for fluid flow. <br>

## Usage
The following steps will allow you to run this Rust project and see the outputs.
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
&emsp; - Enter 1 to solve an initial value problem (IVP) using RK4. <br>
&emsp; - Enter 2 to solve Burger's equation using RK4. <br>
4. Follow the on-screen instructions to input your choice and view the results. <br>

## Dependencies
This project uses the Rust programming language and Cargo, the Rust package manager. No additional dependencies are required. <br>
