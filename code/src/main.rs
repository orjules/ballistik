mod simplethrow;
mod fileprinter;

fn main() {
    let data = simplethrow::get_vec();
    fileprinter::write_to_file("First Try", data);
}
