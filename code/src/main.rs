mod simplethrow;
mod fileprinter;
mod throw3d;
mod stokesthrow;

fn main() {
    let data = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 1.0);
    fileprinter::write_to_file("v100100s1.txt", data);
    let data = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 0.6);
    fileprinter::write_to_file("v100100s06.txt", data);
    let data = simplethrow::get_list((0.0, 0.0), (80.0, 100.0), 0.6);
    fileprinter::write_to_file("v80100s06.txt", data);

    let data3d = throw3d::get_3d_pos_list((0.0, 0.0, 0.0), (100.0, 100.0, 100.0), 1.0);
    fileprinter::write_to_3d_file("v111s1.txt", data3d);


    // Experimente mit Stokes
    let data = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 0.5);
    fileprinter::write_to_file("v100100s05.txt", data);
    let data = simplethrow::get_list((0.0, 0.0), (1000.0, 1000.0), 5.0);
    fileprinter::write_to_file("v1T1Ts5.txt", data);
    let stokes_data = stokesthrow::get_stokes_list((0.0, 0.0), (100.0, 100.0), 1.0, 0.5);
    fileprinter::write_to_file("v100100r1s05.txt", stokes_data);
    let stokes_data = stokesthrow::get_stokes_list((0.0, 0.0), (1000.0, 1000.0), 1.0, 5.0);
    fileprinter::write_to_file("v1T1Tr1s5.txt", stokes_data);
}
