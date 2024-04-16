// Exercise 2 - Solve Burger's equation for fluid flow using RK4, assuming some initial conditions.
// dudt = NU * d2u_dx2 - udu_dx
// Initial conditions: u(x, 0) = 0.5 * (1 - exp(-x^2)), 0 <= x <= 10

const NX: usize = 10;  // Number of spatial points
const NT: usize = 10;  // Number of time steps
const DT: f64 = 1.0;   // Time step
const DX: f64 = 1.0;   // Spatial step
const NU: f64 = 0.01;  // Viscosity

fn dudt(u: &[f64; NX], i: usize) -> f64 {
    let du_dx = (u[i + 1] - u[i - 1]) / (2.0 * DX);  // central difference approx. of first derivative
    let udu_dx = u[i] * du_dx;
    let d2u_dx2 = (u[i + 1] - 2.0 * u[i] + u[i - 1]) / (DX * DX);  // central difference approx. of second derivative
    NU * d2u_dx2 - udu_dx
}

pub fn solve_burgers() {
    let mut u = [0.0; NX];
    let mut u_new = [0.0; NX];
    let mut u_prev = [0.0; NX];

    // Set initial conditions; u(0,x)
    for i in 0..NX {
        let x = i as f64 * DX;
        u[i] = 0.5 * (1.0 - (-x.powi(2)).exp());
    }

    // Solve Burger's equation using RK4
    let mut t = 0.0;
    for _ in 0..NT {
        // Save previous time step
        u_prev.copy_from_slice(&u);

        // RK4 steps
        for i in 1..NX - 1 {
        
            // Calculation for k1
            let k1 = DT * dudt(&u_prev, i);
            
            // Calculation for k2
            let mut u_prev_k2 = u_prev.clone(); // Clone u_prev to modify it
            for elem in u_prev_k2.iter_mut() {
                *elem += 0.5 * k1;
            }
            let k2 = DT * dudt(&u_prev_k2, i);
            
            // Calculation for k3
            let mut u_prev_k3 = u_prev.clone(); // Clone u_prev to modify it
            for elem in u_prev_k3.iter_mut() {
                *elem += 0.5 * k2;
            }
            let k3 = DT * dudt(&u_prev_k3, i);
            
            // Calculation for k4
            let mut u_prev_k4 = u_prev.clone(); // Clone u_prev to modify it
            for elem in u_prev_k4.iter_mut() {
                *elem += k3;
            }
            let k4 = DT * dudt(&u_prev_k4, i);

            u_new[i] = u_prev[i] + (1.0 / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
        }

        // Update u
        u.copy_from_slice(&u_new);
        t += DT;

        // Print current state (optional)
        println!("Time step: {}", t);
        for (i, &value) in u.iter().enumerate() {
            let x = i as f64 * DX;
            println!("u({}, {}) = {:.6}", t, x, value);
        }
    }
}
