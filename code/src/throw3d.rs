/// The return values are pos_x, pos_y, pos_z and time
/// Here x and y are a plane with z being up and down
pub fn get_3d_pos_list(position: (f64, f64, f64), velocity: (f64, f64, f64), delta_t: f64) -> Vec<(f64, f64, f64, f64)>{
    // Set initial values
    let mut rx = position.0;
    let mut ry = position.1;
    let mut rz = position.2;
    let mut vx = velocity.0;
    let mut vy = velocity.1;
    let mut vz = velocity.2;
    let mut t = 0.0;
    let dt = delta_t;
    let gx = 0.0;
    let gy = 0.0;
    let gz = -9.81;

    // Initiales speichern
    let mut vec = vec![(rx, ry, rz, t)];

    // Eigentlicher loop
    loop {
        // Erst neue Position, dann Geschwindigkeit, falls g später ortsabhängig sein soll!
        let newrx = rx + vx * dt;
        let newry = ry + vy * dt;
        let newrz = rz + vz * dt;
        let newvx = vx + gx * dt;
        let newvy = vy + gy * dt;
        let newvz = vz + gz * dt;

        t += dt;
        rx = newrx;
        ry = newry;
        rz = newrz;
        vx = newvx;
        vy = newvy;
        vz = newvz;

        if rz < 0.0{
            break;
        }

        vec.push((rx, ry, rz, t));
    }
    return vec;
}