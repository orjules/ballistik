pub fn get_vec() -> Vec<(f64, f64, f64, f64, f64)>{
    // Set initial values
    let mut rx = 0.0;
    let mut ry = 0.0;
    let mut vx = 100.0;
    let mut vy = 100.0;
    let mut dt = 0.8;
    let mut t = 0.0;
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

