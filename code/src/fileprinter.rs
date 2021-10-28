use std::fmt;
use std::fs;

pub fn print_result(descriptor: &str, result: Vec<(f64, f64, f64, f64)>){
    println!("{}", descriptor);
    for i in result {
        println!("{}, {}", i.0 as i32, i.1 as i32);
    }
}

pub fn write_to_file(name: &str, data: Vec<(f64, f64, f64, f64)>){
    let mut content = String::new();
    for position in data {
        content.push_str(&format!("{}, {}\n", position.0, position.1));
    }
    fs::write(format!("{}", name), content).expect("Unable to write file");
}