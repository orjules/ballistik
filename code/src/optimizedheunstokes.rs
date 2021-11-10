// Rust hat keine Fixed Point Zahlen aber weil ich das sowieso selber verstehen muss, mache ich es
// mit unsinged ints der länge 32bit
// Dabei denke ich mir den Punkt nach vorne verschoben also 12bit,20bit bzw 3*4,5*4


use std::io::ErrorKind::Other;

/// Constants will be read from memory
// 3,141592025756835
const PI: u32 = 0b0000_0000_0011_0010_0100_0011_1111_0110;
// 0,000018119812011 kg/(m*s)  (Dynamic Viscosity of air at 20 degree Celsius)
const VISC: u32 = 0b0000_0000_0000_0000_0000_0000_0001_0011;
// Multiplication of 6*PI*VISC
const STOKES_FACTOR: u32 = 0b0000_0000_0000_0000_0000_0001_0110_0110;
// Multiplication of 6*PI
const OTHER_STOKES_FACTOR: u32 = 0b0000_0001_0010_1101_1001_0111_1100_0111;

/// position: tuple of x,y in meters
/// velocity: tuple of x,y in meters per second
/// radius: value in meters
/// delta_t: value in seconds for timesteps (smaller means more exact)
/// mass: value in kilogram
/// returns: Vector of tuple of pos_x, pos_y, vel_x, vel_y, time
pub fn get_list(pos_x: u32, pos_y: u32, vel_x: u32, vel_y: u32, radius: u32, mass: u32, delta_t: u32)
                -> Vec<(u32, u32, u32, u32, u32)>{
    // Set initial values
    let mut rx = pos_x;
    let mut ry = pos_y;
    let mut vx = vel_x;
    let mut vy = vel_y;
    let mut t = 0;
    let dt = delta_t;
    let gx = 0;
    let gy = 0b0000_0000_1001_1100_1111_0101_1100_0010; // 9,809999465942382 (m/s^2)


    // Save initial values in new Vector
    let mut vec = vec![(rx, ry, vx, vy, t)];
    loop {
        // x and y can be viewed completely seperate
        let stokes_force_x_mul_1 = mult(radius, vx); // 1 cycle
        let stokes_force_x_mul_1_scaled = scale(stokes_force_x_mul_1); // 2 cycles
        let stokes_force_x_mul_2 = mult(STOKES_FACTOR, stokes_force_x_mul_1_scaled); // 3 cycle
        let stokes_force_x_mul_2_scaled = scale(stokes_force_x_mul_2); // 4 cycles
        let stokes_acc_x = div(stokes_force_x_mul_2_scaled, mass); // 7 cycles
        if ry < 0{
            break;
        }
    }

    // alternativ ohne scaler, sondern nur wegschneiden also kein Takt dafür
    loop {
        // Hier nur x, weil y genau analog funktioniert
        let stokes_force_x_mul_1 = mult(radius, vx); // 1 cycle
        let stokes_force_x_mul_1_scaled = scale(stokes_force_x_mul_1); // 1 cycles
        let stokes_force_x_mul_2 = mult(OTHER_STOKES_FACTOR, stokes_force_x_mul_1_scaled); // 2 cycle
        let stokes_force_x_mul_2_scaled = scale(stokes_force_x_mul_2); // 2 cycles
        let stokes_force_x_mul_3 = mult(VISC, stokes_force_x_mul_2_scaled); // 3 cycle
        let stokes_force_x_mul_3_scaled = scale(stokes_force_x_mul_3); // 3 cycles
        let stokes_acc_x = div(stokes_force_x_mul_3_scaled, mass); // 6 cycles
        let stokes_acc_x_scaled = scale(stokes_acc_x); // 6 cycles
        // Die drei Takte Mult und 3 Takte Div sollten doch gut für Pipelining sein, oder?
        let acc_x = mult(gx, stokes_acc_x_scaled); // 7 cycles
        let acc_x_scaled = scale(acc_x); // 7 cycles
        let vel_x = mult(acc_x_scaled, dt); // 8 cycles
        let vel_x_scaled = scale(vel_x); // 8 cycles
        // Add sub immer als Tuple und dann die wichtigen Sachen rausübernehmen
        let sum1 = add_sub(vx, vel_x_scaled, 1); // 9 cycles
        let next_vx = sum1.0;
        // let newrx = rx + (vx + next_vx)/2.0 * dt;  wird zu:
        let sum2 = add_sub(vx, next_vx, sum1.4); // 10 cycles
        let newrx_added = sum2.0;
        let newrx_halfed = my_shift(newrx_added.0, 1); // 11 cycles
        let newrx_timed = mult(newrx_halfed, dt); // 12 cycles
        let newrx_timed_scaled = scale(newrx_timed); // 12 cycles
        let sum3 = add_sub(rx, newrx_timed_scaled, sum2.4); // 13 cycles
        let newrx = sum3.0;
        // TODO Bei add_sub die anderen flags noch verarbeiten
        // TODO: newrx sinnvoll speicher
        rx = newrx;
        if ry < 0{
            // Abfrage mit check für negativ underflow ersetzten
            break;
        }
        vec.push((rx, ry, vx, vy, t));
    }

    /*
    Original
    loop {
        // Calculate next velocity with gravity and stokes formula in small steps
        // let stokes_force_x = 6 * PI * radius * VISC * vx;
        // let stokes_force_y = 6 * PI * radius * VISC * vy;
        let stokes_force_x = STOKES_FACTOR * radius * vx;
        let stokes_force_y = STOKES_FACTOR * radius * vy;
        let stokes_acc_x = stokes_force_x / mass;
        let stokes_acc_y = stokes_force_y / mass;
        let acc_x = gx + stokes_acc_x;
        let acc_y = gy + stokes_acc_y;
        let vel_x = acc_x * dt;
        let vel_y = acc_y * dt;
        // hier muss ich aufpassen wegen overflow
        let next_vx = vx - vel_x;
        let next_vy = vy - vel_y;

        // Calculate next position with median of the speeds
        let newrx = rx + (vx + next_vx)/2.0 * dt;
        let newry = ry + (vy + next_vy)/2.0 * dt;
        // Assign new values
        t += dt;
        rx = newrx;
        ry = newry;
        vx = next_vx;
        vy = next_vy;
        // break condition is reaching the "ground" aka y==0
        if ry < 0{
            break;
        }
        vec.push((rx, ry, vx, vy, t));
    }

     */
    return vec;
}

/*
fn dec_to_my_bin(number: f64) -> u32{
    return 0;
}

fn my_bin_to_dec(number: u32) -> f64{
    return 0.0;
}
 */

// Input: Two numbers to add or subtract
// sub_flag=0 => Addition, sub_flag=1 => Subtraction
// Return: (number, zero_flag, carry_flag, overflow_flag, negative_flag)
pub fn add_sub(num1: u32, num2: u32, sub_flag: u8) -> (u32, u8, u8, u8, u8){

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

pub fn mult(num1: u32, num2: u32) -> u64{
    return num1 as u64 * num2 as u64;
}

pub fn div(num1: u32, num2: u32) -> u64{
    return 0;
}

pub fn scale(num: u64) -> u32{
    return num as u32;
}

// Shifts number by 1 bit and adds a zero
// sel=0 => shift left, sel=1 => shift right
pub fn my_shift(num: u32, sel: u8) -> u32{
    if sel == 0{
        return num << 1;
    }else if sel == 1 {
        return num >> 1;
    }else {
        return 0;
    }
}
