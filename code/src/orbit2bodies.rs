// Braucht keine Reibung aber Gravitation ist Ortsabhängig
const G: f64 =  0.000000000066743; //  m^3 / kg * s^2

/// Returns pos_1_x, pos_1_y, pos_2_x, pos_2_y, time
pub fn get_orbit_2_bodies_2d(mass_1: f64, pos_1: (f64, f64), vel_1: (f64, f64),
                             mass_2: f64, pos_2: (f64, f64), vel_2: (f64, f64),
                             d_t: f64, step_count: u32)
                             -> Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>{
    let mut time = 0.0;
    let mut r_1_x = pos_1.0;
    let mut r_1_y = pos_1.1;
    let mut v_1_x = vel_1.0;
    let mut v_1_y = vel_1.1;
    let mut r_2_x = pos_2.0;
    let mut r_2_y = pos_2.1;
    let mut v_2_x = vel_2.0;
    let mut v_2_y = vel_2.1;

    let mut list = vec![(r_1_x, r_1_y, v_1_x, v_1_y, r_2_x, r_2_y, v_2_x, v_2_y, time)];

    for _number in 0..step_count {
        // Ort neu berechnen
        let r_1_x_new = r_1_x + v_1_x * d_t;
        let r_1_y_new = r_1_y + v_1_y * d_t;
        let r_2_x_new = r_2_x + v_2_x * d_t;
        let r_2_y_new = r_2_y + v_2_y * d_t;
        // Geschwindigkeit neu berechnen
        let r_1_2 = (r_2_x-r_1_x)*(r_2_x-r_1_x)+(r_2_y-r_1_y)*(r_2_y-r_1_y);
        let v_1_x_new = v_1_x + (G * mass_2 / r_1_2 * (r_2_x - r_1_x) / r_1_2.sqrt()) * d_t;
        let v_1_y_new = v_1_y + (G * mass_2 / r_1_2 * (r_2_y - r_1_y) / r_1_2.sqrt()) * d_t;
        let r_2_1 = (r_1_x-r_2_x)*(r_1_x-r_2_x)+(r_1_y-r_2_y)*(r_1_y-r_2_y);
        let v_2_x_new = v_2_x + (G * mass_1 / r_2_1 * (r_1_x - r_2_x) / r_2_1.sqrt()) * d_t;
        let v_2_y_new = v_2_y + (G * mass_1 / r_2_1 * (r_1_y - r_2_y) / r_2_1.sqrt()) * d_t;
        // Neu zuweisen
        r_1_x = r_1_x_new;
        r_1_y = r_1_y_new;
        v_1_x = v_1_x_new;
        v_1_y = v_1_y_new;
        r_2_x = r_2_x_new;
        r_2_y = r_2_y_new;
        v_2_x = v_2_x_new;
        v_2_y = v_2_y_new;
        time += d_t;
        list.push((r_1_x, r_1_y, v_1_x, v_1_y, r_2_x, r_2_y, v_2_x, v_2_y, time))
    }

    return list;
}