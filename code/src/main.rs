mod simplethrow;
mod fileprinter;
mod throw3d;
mod stokesthrow;
mod orbit2bodies;

fn main() {
    // initial_tests();
    // test_3d();
    // test_stokes();
    test_orbit_2_bodies();
}

#[allow(dead_code)]
fn initial_tests(){
    let data = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 1.0);
    fileprinter::write_to_file("v100100s1.txt", data);
    let data = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 0.6);
    fileprinter::write_to_file("v100100s06.txt", data);
    let data = simplethrow::get_list((0.0, 0.0), (80.0, 100.0), 0.6);
    fileprinter::write_to_file("v80100s06.txt", data);
}

#[allow(dead_code)]
fn test_3d(){
    let data3d = throw3d::get_3d_pos_list((0.0, 0.0, 0.0), (100.0, 100.0, 100.0), 1.0);
    fileprinter::write_to_3d_file("v111s1.txt", data3d);
}

#[allow(dead_code)]
fn test_stokes(){
    let data = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 0.5);
    fileprinter::write_to_file("v100100s05.txt", data);
    let data = simplethrow::get_list((0.0, 0.0), (1000.0, 1000.0), 5.0);
    fileprinter::write_to_file("v1T1Ts5.txt", data);
    let stokes_data = stokesthrow::get_stokes_list((0.0, 0.0), (100.0, 100.0), 1.0, 0.5);
    fileprinter::write_to_file("v100100r1s05.txt", stokes_data);
    let stokes_data = stokesthrow::get_stokes_list((0.0, 0.0), (1000.0, 1000.0), 1.0, 5.0);
    fileprinter::write_to_file("v1T1Tr1s5.txt", stokes_data);
}

#[allow(dead_code)]
fn test_orbit_2_bodies(){
    // 5,9722 · 10^24 kg
    // 384400000 m => ca 1023 m/s
    // oder 384400 km => ca. 1,022 km/s
    // 1.082

    // Masse Mond: 7,346 · 10^22 kg, Masse Erde: 5,9724 · 10^24 kg

    // Mittelwerte: Semimajor axis (106 km)	0.3844, Mean orbital velocity (km/s)	1.022
    // Nächste: Perigee (106 km)*0.3633, Max. orbital velocity (km/s)	1.082
    // Weiteste: Apogee (106 km)*	0.4055, Min. orbital velocity (km/s)	0.970
    let list = orbit2bodies::get_orbit_2_bodies_2d(5972400000000000000000000.0,
                                                   (0.0, 0.0), (0.0, 0.0),
                                                   73460000000000000000000.0,
                                                   (384400000.0, 0.0), (0.0, 1022.0,),
                                                   1000.0, 2390);
    fileprinter::write_to_orbit_2d_file("Monat", "Erde", "Mond", list);
    // Versuch mit kleineren Objekten
    //let list = orbit2bodies::get_orbit_2_bodies_2d()
}