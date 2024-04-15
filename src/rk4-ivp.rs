// Exercise 1 - Solve the initial value problem (IVP) using the Fourth-Order Runge-Kutta (RK4) method
// dy/dt = 1 - t^2 + y, y(0) = 0.5, 0 <= t <= 2 with n = 10

fn main() {
    let a = 0.0;    // initial value of t
    let b = 2.0;    // final value of t
    let n = 10;     // number of steps
    let h = (b - a) / (n as f64);   // step size

    // Initial values
    let mut t = a;
    let mut y = 0.5; // y(0) = 0.5

    // Function for dy/dt = 1 - t^2 + y
    fn f(t: f64, y: f64) -> f64 {
        1.0 - t.powi(2) + y
    }

    // Runge-Kutta method
    for _i in 0..n {
        let k1 = h * f(t, y);
        let k2 = h * f(t + h / 2.0, y + k1 / 2.0);
        let k3 = h * f(t + h / 2.0, y + k2 / 2.0);
        let k4 = h * f(t + h, y + k3);

        y += (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        t += h;
    }

    println!("The value of y at t = {} is {}", t, y);
}
