/// Constants will be read from memory
const PI: f64 = 3.142;
const VISC: f64 = 0.000018215; // Dynamic Viscosity of air at 20 degree Celsius in kg/(m*s)

/// position: tuple of x,y in meters
/// velocity: tuple of x,y in meters per second
/// radius: value in meters
/// delta_t: value in seconds for timesteps (smaller means more exact)
/// mass: value in kilogram
/// returns: Vector of tuple of pos_x, pos_y, vel_x, vel_y, time
pub fn get_list(position: (f64, f64), velocity: (f64, f64), radius: f64, delta_t: f64, mass: f64)
    -> Vec<(f64, f64, f64, f64, f64)>{
    // Set initial values
    let mut rx = position.0;
    let mut ry = position.1;
    let mut vx = velocity.0;
    let mut vy = velocity.1;
    let mut t = 0.0;
    let dt = delta_t;
    let gx = 0.0;
    let gy = -9.81;

    // Save initial values in new Vector
    let mut vec = vec![(rx, ry, vx, vy, t)];
    loop {
        // Calculate next velocity with gravity and stokes formula
        let next_vx = vx + (gx - 6.0 * PI * radius * VISC * vx / mass) * dt;
        let next_vy = vy + (gy - 6.0 * PI * radius * VISC * vy / mass) * dt;
        // Calculate next position with median of the speeds
        let newrx = rx + (vx + next_vx)/2.0 * dt;
        let newry = ry + (vy + next_vy)/2.0 * dt;
        // let newry = ry + (vy - v_y_start + next_vy - v_y_start)/2.0 * dt;
        // let newry = ry + (vy - v_y_start + next_vy - v_y_start)/2.0 * dt * v_y_start;
        // Assign new values
        t += dt;
        rx = newrx;
        ry = newry;
        vx = next_vx;
        vy = next_vy;
        // break condition is reaching the "ground" aka y==0
        if ry < 0.0{
            break;
        }
        vec.push((rx, ry, vx, vy, t));
    }
    return vec;
}