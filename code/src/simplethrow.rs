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

    // Eigentlicher loop
    loop {
        // Erst neue Position, dann Geschwindigkeit, falls g später ortsabhängig sein soll!
        let newrx = rx + vx * dt;
        let newry = ry + vy * dt;
        let newvx = vx + gx * dt;
        let newvy = vy + gy * dt;

        t += dt;
        rx = newrx;
        ry = newry;
        vx = newvx;
        vy = newvy;

        if ry < 0.0{
            break;
        }

        vec.push((rx, ry, vx, vy, t));
    }
    return vec;
}

