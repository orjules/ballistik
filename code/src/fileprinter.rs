use std::fs;

#[allow(dead_code)]
pub fn print_exact(descriptor: &str, result: Vec<(f64, f64, f64, f64, f64)>){
    println!("{}", descriptor);
    println!("Ort_x, Ort_y, Geschw_x, Geschw_y, Zeit");
    for i in result {
        println!(" {:.3} , {:.3} , {:.3} , {:.3} , {:.3} ", i.0, i.1, i.2, i.3, i.4);
    }
}

pub fn write_to_file(name: &str, data: Vec<(f64, f64, f64, f64, f64)>){
    let mut content = String::new();
    content.push_str("Ort_x, Ort_y, Geschw_x, Geschw_y, Zeit\n");
    for pos in data {
        content.push_str(&format!("{:.3},{:.3},{:.3},{:.3},{:.3}\n",
                                  pos.0, pos.1, pos.2, pos.3, pos.4));
    }
    fs::write(format!("../TextOutputs/{}", name), content).expect("Unable to write file");
}

pub fn write_to_file_32(name: &str, data: Vec<(u32, u32, u32, u32, u32)>){
    let mut content = String::new();
    content.push_str("Ort_x, Ort_y, Geschw_x, Geschw_y, Zeit\n");
    for pos in data {
        content.push_str(&format!("{:.3},{:.3},{:.3},{:.3},{:.3}\n",
                                  pos.0, pos.1, pos.2, pos.3, pos.4));
    }
    fs::write(format!("../TextOutputs/{}", name), content).expect("Unable to write file");
}

pub fn write_to_3d_file(name: &str, data: Vec<(f64, f64, f64, f64)>){
    let mut content = String::new();
    content.push_str("Ort_x, Ort_y, Ort_z, Zeit\n");
    for pos in data {
        content.push_str(&format!("{:.3},{:.3},{:.3},{:.3}\n",
                                  pos.0, pos.1, pos.2, pos.3));
    }
    fs::write(format!("../TextOutputs/{}", name), content).expect("Unable to write file");
}

pub fn write_to_orbit_2d_file(file_name: &str, body_1_name: &str, body_2_name: &str,
                              data: Vec<(f64, f64, f64, f64, f64, f64, f64, f64, f64)>){
    let mut content = String::new();
    content.push_str(&format!("{}_x, {}_y, {}_v_x, {}_v_y, {}_x, {}_y, {}_v_x, {}_v_y, Zeit\n",
                              body_1_name, body_1_name, body_1_name, body_1_name,
                              body_2_name, body_2_name, body_2_name, body_2_name));
    for pos in data {
        content.push_str(&format!("{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3}\n",
                                  pos.0, pos.1, pos.2, pos.3, pos.4, pos.5, pos.6, pos.7, pos.8));
    }
    fs::write(format!("../TextOutputs/{}.txt", file_name), content).expect("Unable to write file");
}