const PI: f64 = 3.142;
const VISC: f64 = 0.000018215;

const STOKES: f64 = 0.000343345;

pub fn get_list(position: (f64, f64), velocity: (f64, f64), radius: f64, delta_t: f64, mass: f64)
                -> Vec<(f64, f64, f64, f64, f64)>{

    let mut rx = position.0;
    let mut ry = position.1;
    let mut vx = velocity.0;
    let mut vy = velocity.1;
    let mut t = 0.0;

    // Helpful "constants"
    let stokes_const_1 = STOKES * radius;
    let stokes_const_2 = stokes_const_1/mass;
    let stokes_const = stokes_const_2 * delta_t;
    let earth_speed_y = -9.81 * delta_t;
    let half_step = delta_t/2.0;

    // Save initial values in new Vector
    let mut vec = vec![(rx, ry, vx, vy, t)];
    loop {
        // Calculate next velocity with gravity and stokes formula
        let next_vx_1 = stokes_const * vx;
        let next_vx = vx - next_vx_1;
        let next_vy_1 = vy + earth_speed_y; // der eine Schritt mehr, den y über x braucht
        let next_vy_2 = stokes_const * vy;
        let next_vy = next_vy_1 - next_vy_2;
        // Calculate next position with median of the speeds
        let newrx_1 = vx + next_vx;
        let newrx_2 = newrx_1 * half_step;
        let newrx = rx + newrx_2;
        let newry_1 = vy + next_vy;
        let newry_2 = newry_1 * half_step;
        let newry = ry + newry_2;
        // Assign new values
        t += delta_t;
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