mod simplethrow;
mod fileprinter;

fn main() {
    let data = simplethrow::get_vec();
    fileprinter::write_to_file("Output.txt", data);
    // fileprinter::print_exact("Ergebnis", data);
}
