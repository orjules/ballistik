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

    // Wie Euler von einem Punkt die Steigung berechnen
    // und über die Schrittweite den nächsten Punkt berechnen
    // Aber diesmal an diesem neuen Punkt auch die Steigung berechnen und den Mittelwert beider bilden
    // Dann mit diesem Mittelwert und der Schrittweite einen besseren nächsten Punkt finden

    loop {
        let next_vx = vx + gx * dt;
        let next_vy = vy + gy * dt;

        let newrx = rx + (vx + next_vx)/2.0 * dt;
        let newry = ry + (vy + next_vy)/2.0 * dt;

        t += dt;
        rx = newrx;
        ry = newry;
        vx = next_vx;
        vy = next_vy;

        if ry < 0.0{
            break;
        }

        vec.push((rx, ry, vx, vy, t));
    }
    return vec;
}