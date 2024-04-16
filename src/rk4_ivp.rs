// Exercise 1 - Solve the initial value problem (IVP) using the Fourth-Order Runge-Kutta (RK4) method
// dy/dt = 1 - t^2 + y, 
// y(0) = 0.5, 0 <= t <= 2 with n = 10

pub fn rk4() {
    let a = 0.0;    // initial value of t
    let b = 2.0;    // final value of t
    let n = 10;     // number of steps
    let h = (b - a) / (n as f64);   // step size

    // Initial values
    let mut t = a; 
    let mut y = 0.5;   // y(0) = 0.5

    // Vector to store y values
    let mut y_list: Vec<f64> = Vec::new();
    y_list.push(y);     // Store initial value of y

    // Function dy/dt = 1 - t^2 + y
    fn dydt(t: f64, y: f64) -> f64 {
        1.0 - t.powi(2) + y
    }

    // Runge-Kutta Fourth Order method
    for _i in 0..n {
        let k1 = h * dydt(t, y);    // Calculation for k1
        let k2 = h * dydt(t + h / 2.0, y + k1 / 2.0);   // Calculation for k2
        let k3 = h * dydt(t + h / 2.0, y + k2 / 2.0);   // Calculation for k3
        let k4 = h * dydt(t + h, y + k3);   // Calculation for k4

        y += (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;  // Update y
        t += h;   // Update t
        y_list.push(y); // Store the value of y for each step
    }

    // Print the values of y at each step
    for (index, value) in y_list.iter().enumerate() {
        println!("Step {}: y({:.2}) = {:.4}", index, a + index as f64 * h, value);
    }
}
