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

    // Erde_x,          Erde_y,         Erde_v_x,       Erde_v_y,   Mond_x,             Mond_y,         Mond_v_x,       Mond_v_y
    // 0.000,           0.000,          0.000,          0.000,      384400.000,         0.000,          0.000,          1.022
    // 0.000,           0.000,          33194.583,      0.000,      384400.000,         1022.000,       -2697573.638,   1.022
    // 33194582.676     0.000,          66388.813,      88.253,     -2697189238.011,    2044.000,       -5395118.674,   -7170.911
    // 99583396.071     88253.132,      66388.813,      88.253,     -8092307912.076,    -7168866.952,   -5395118.621,   -7170.911
    // 165972208.808    176506.263,     66388.813,      88.253,     -13487426532.674,   -14339777.905,  -5395118.615,   -7170.911
    // 232361021.472,   264759.395,     66388.813,      88.253,     -18882545147.331,   -21510688.852,  -5395118.613,   -7170.911
    // 298749834.109,   353012.526,     66388.813,      88.253,     -24277663759.850,   -28681599.796,  -5395118.611,   -7170.911

    for number in 0..step_count {
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

/// Returns pos_1_x, pos_1_y, pos_1_z, pos_2_x, pos_2_y, pos_2_z, time
pub fn get_orbit_2_bodies_3d(mass_1: f64, pos_1: (f64, f64, f64), vel_1: (f64, f64, f64),
                          mass_2: f64, pos_2: (f64, f64, f64), vel_2: (f64, f64, f64), d_t: f64)
-> Vec<(f64, f64, f64, f64, f64, f64, f64)>{
    let mut time = 0.0;
    let mut r_1_x = pos_1.0;
    let mut r_1_y = pos_1.1;
    let mut r_1_z = pos_1.2;
    let mut v_1_x = vel_1.0;
    let mut v_1_y = vel_1.1;
    let mut v_1_z = vel_1.2;
    let mut r_2_x = pos_2.0;
    let mut r_2_y = pos_2.1;
    let mut r_2_z = pos_2.2;
    let mut v_2_x = vel_2.0;
    let mut v_2_y = vel_2.1;
    let mut v_2_z = vel_2.2;

    let mut list = vec![(r_1_x, r_1_y, r_1_z, r_2_x, r_2_y, r_2_z, time)];

    loop {
        // Ort neu berechnen
        let r_1_x_new = r_1_x + v_1_x * d_t;
        let r_1_y_new = r_1_y + v_1_y * d_t;
        let r_1_z_new = r_1_z + v_1_z * d_t;
        let r_2_x_new = r_2_x + v_2_x * d_t;
        let r_2_y_new = r_2_y + v_2_y * d_t;
        let r_2_z_new = r_2_z + v_2_z * d_t;
        // Geschwindigkeit neu berechnen
        let r_1_2 = (r_2_x-r_1_x)*(r_2_x-r_1_x)+(r_2_y-r_1_y)*(r_2_y-r_1_y)+(r_2_z-r_1_z)*(r_2_z-r_1_z);
        let v_1_x_new = v_1_x + (G * mass_2 / r_1_2 * (r_2_x - r_1_x) / r_1_2.sqrt()) * d_t;
        let v_1_y_new = v_1_y + (G * mass_2 / r_1_2 * (r_2_y - r_1_y) / r_1_2.sqrt()) * d_t;
        let v_1_z_new = v_1_z + (G * mass_2 / r_1_2 * (r_2_z - r_1_z) / r_1_2.sqrt()) * d_t;
        let r_2_1 = (r_1_x-r_2_x)*(r_1_x-r_2_x)+(r_1_y-r_2_y)*(r_1_y-r_2_y)+(r_1_z-r_2_z)*(r_1_z-r_2_z);
        let v_2_x_new = v_2_x + (G * mass_1 / r_2_1 * (r_1_x - r_2_x) / r_2_1.sqrt()) * d_t;
        let v_2_y_new = v_2_y + (G * mass_1 / r_2_1 * (r_1_y - r_2_y) / r_2_1.sqrt()) * d_t;
        let v_2_z_new = v_2_z + (G * mass_1 / r_2_1 * (r_1_z - r_2_z) / r_2_1.sqrt()) * d_t;
        // Neu zuweisen
        r_1_x = r_1_x_new;
        r_1_y = r_1_y_new;
        r_1_z = r_1_z_new;
        v_1_x = v_1_x_new;
        v_1_y = v_1_y_new;
        v_1_z = v_1_z_new;
        r_2_x = r_2_x_new;
        r_2_y = r_2_y_new;
        r_2_z = r_2_z_new;
        v_2_x = v_2_x_new;
        v_2_y = v_2_y_new;
        v_2_z = v_2_z_new;
    }

}