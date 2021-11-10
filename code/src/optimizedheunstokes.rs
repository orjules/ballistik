// Rust hat keine Fixed Point Zahlen aber weil ich das sowieso selber verstehen muss, mache ich es
// mit unsinged ints der länge 32bit
// Den Punkt wähle ich in der Mitte also 16 Bit, 16 Bit

/// Constants will be read from memory
// Multiplication of 6*PI*VISC also 0,0003415351037 (mit etwas Fehler durch abschneiden)
const STOKES_FACTOR: u32 = 0b0000_0000_0000_0000_0000_0000_0001_0110;
const GRAVITY: u32 = 0b0000_0000_0000_1001_1100_1111_0101_1100;

/// position: tuple of x,y in meters
/// velocity: tuple of x,y in meters per second
/// radius: value in meters
/// delta_t: value in seconds for timesteps (smaller means more exact)
/// mass: value in kilogram
/// returns: Vector of tuple of pos_x, pos_y, vel_x, vel_y, time
pub fn no_negatives(pos_x: u32, pos_y: u32, vel_x: u32, vel_y: u32, radius: u32, mass: u32, delta_t: u32)
    -> Vec<(u32, u32, u32, u32, u32)> {
    // Erstmal Initialwerte in register schreiben
    let mut rx = pos_x;
    let mut ry = pos_y;
    let mut vx = vel_x;
    let mut vy = vel_y;
    // Konstanten für diesen Durchlauf berechnen
    let vel_const_1 = mult(STOKES_FACTOR, radius);
    println!("{}", vel_const_1);
    let vel_const_2 = mult(vel_const_1, delta_t);
    println!("{}", vel_const_2);
    let vel_const_3 = vel_const_2 / mass; // TODO Diff implementieren
    println!("{}", vel_const_3);
    let vel_const_4 = my_shift(vel_const_3, 1); // halbiert
    println!("{}", vel_const_4);
    let vel_const_5 = 0b0000_0000_0000_0001_0000_0000_0000_0000 - vel_const_4;
    let vel_const = delta_t * vel_const_5;
    let start_vel_y = vel_const * vel_y;
    let earth_const_1 = GRAVITY * delta_t;
    let earth_const_2 = earth_const_1 * delta_t;
    let earth_const = my_shift(earth_const_2, 1); // halbiert
    // Zu testzwecken
    let mut t = 0;
    let mut values = vec![(rx, ry, vx, vy, t)];

    loop {
        // X Komponente
        let useful_vel_x = vel_const * vx;
        let new_vx = vel_const_5 * vx;
        let new_rx = add_sub(rx, useful_vel_x, 0).0; // wird nie kleiner 0
        // Y Komponente
        let useful_vel_y = vel_const * vy;
        let new_vy_1 = vel_const_5 * vy;
        let new_vy = add_sub(new_vy_1, earth_const_1, 1).0; // Wird nie kleiner 0, weil hier die Verdopplung noch drin steckt
        let sum_1 = add_sub(useful_vel_y, earth_const, 0).0; // Kann nie kleiner 0 werden
        let return_type = add_sub(sum_1, start_vel_y, 1); // Könnte kleiner 0 werden
        let mut final_addition;
        if return_type.4 == 1{
            // Fall, wenn kleiner Null
            let sum2 = add_sub(start_vel_y, sum_1, 1).0; // Kann nicht mehr kleiner 0 sein
            final_addition = add_sub(ry, sum2, 1); // die Neue Zahl muss abgezogen werden
        }else {
            final_addition = add_sub(ry, return_type.0, 1); // die erste Zahl war positiv und kann einfach abgezogen werden
        }

        if final_addition.4 == 1{
            // Finale Addition wurde negativ, heißt wir hätten eine negative y position
            break;
        }
        vx = new_vx;
        rx = new_rx;
        vy = new_vy;
        ry = final_addition.0;

        // Zu Testzwecken
        t += delta_t;
        values.push((rx, ry, vx, vy, t));
    }
    return values;
}

//𝐶 = 𝐴 ∙ 𝐵 = 𝑎 ∙ 𝑏 ∙ 2−2𝑘

// Input: Two numbers to add or subtract
// sub_flag=0 => Addition, sub_flag=1 => Subtraction
// Return: (number, zero_flag, carry_flag, overflow_flag, negative_flag)
fn add_sub(num1: u32, num2: u32, sub_flag: u8) -> (u32, u8, u8, u8, u8){

    // Wo ist der unterschied zwischen carry und overflow? Wozu ist die zero flag?
    let mut result:u32 = 0;
    let mut zero_flag: u8 = 0;
    let mut carry_flag: u8 = 0;
    let mut overflow_flag: u8 = 0;
    let mut negativ_flag: u8 = 0;
    if sub_flag == 0 {
        let ret = num1.overflowing_add(num2);
        result = ret.0;
        if ret.1{
            overflow_flag = 1;
            carry_flag = 1;
        }
    }else if sub_flag == 1 {
        let ret = num1.overflowing_sub(num2);
        result = ret.0;
        if ret.1{
            overflow_flag = 1;
            carry_flag = 1;
            negativ_flag = 1;
        }
    }
    if result == 0{
        zero_flag = 1;
    }
    return (result, zero_flag, carry_flag, overflow_flag, negativ_flag);
}

fn mult(num1: u32, num2: u32) -> u32{
    let big_result = num1 as u64 * num2 as u64;
    let small_result = big_result / 4294967296; // bzw * 2^-32
    return small_result as u32;
}

// Shifts number by 1 bit and adds a zero
// sel=0 => shift left, sel=1 => shift right
fn my_shift(num: u32, sel: u8) -> u32{
    if sel == 0{
        return num << 1;
    }else if sel == 1 {
        return num >> 1;
    }else {
        return 0;
    }
}