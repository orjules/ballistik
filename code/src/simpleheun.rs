pub fn get_list(position: (f64, f64), velocity: (f64, f64), delta_t: f64) -> Vec<(f64, f64, f64, f64, f64)>{
    // Set initial values
    let mut rx = position.0;
    let mut ry = position.1;
    let mut vx = velocity.0;
    let mut vy = velocity.1;
    let mut t = 0.0;
    let dt = delta_t;
    let gx = 0.0;
    let gy = -9.81;

    // Initiales speichern
    let mut vec = vec![(rx, ry, vx, vy, t)];

    // Eigentlicher Code

    return vec;
}