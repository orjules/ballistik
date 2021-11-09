use crate::optimizedheunstokes::{add_sub, mult, my_shift, scale};

mod simplethrow;
mod fileprinter;
mod throw3d;
mod stokesthrow;
mod orbit2bodies;
mod simpleheun;
mod heunstokesthrow;
mod optimizedheunstokes;

fn main() {
    // initial_tests();
    // test_3d();
    // test_stokes();
    // test_orbit_2_bodies();
    // test_simple_heun();
    // test_heun_stokes();
    // test_heun_ohne_stokes();
    // test_heun_stokes_massen();
    // test_optimized();

    assert_eq!(add_sub(5,2,0), (7,0,0,0,0));
    assert_eq!(add_sub(2,5,1), (4294967293,0,1,1,1));
    assert_eq!(scale(5), 5);
    assert_eq!(scale(u32::MAX as u64), u32::MAX);
    assert_eq!(scale(u64::MAX), u32::MAX);
    assert_eq!(scale((u32::MAX as u64)+1), 0);
    assert_eq!(my_shift(1, 0), 2);
    assert_eq!(my_shift(2, 1), 1);
    assert_eq!(my_shift(1, 1), 0);
    assert_eq!(my_shift(u32::MAX, 0), u32::MAX - 1);
    assert_eq!(my_shift(u32::MAX, 1), u32::MAX - 2147483648);
    assert_eq!(mult(3, 5), 15);
    // assert_eq!(mult(u32::MAX, u32::MAX), u64::MAX); // Müsste das nicht gleich sein?
}

fn test_optimized(){
    // Erster Test ist der simpelste Mögliche Wurf:
    // Pos 0,0; Vel 100,100; radius 1; mass 1; delta_t 1
    // TODO aufrufen
    optimizedheunstokes::get_list(0, 0, 100, 100, 1, 1, 1);

    // Zweiter Test verändert nur delta_t was zu mehr genauigkeit führen sollte:
    // Pos 0,0; Vel 100,100; radius 1; mass 1; delta_t 0.1

    // Dritter Test ändert die Velocity Parameter für einen höheren Wurf
    // Pos 0,0; Vel 100,200; radius 1; mass 1; delta_t 1

    // Vierter Test ändert den Radius, was den Drag erhöht
    // Pos 0,0; Vel 100,100; radius 10; mass 1; delta_t 1

    // Fünfter Test ändert die Masse, was die Weite erhöht
    // Pos 0,0; Vel 100,100; radius 1; mass 10; delta_t 1
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
    // Versuch mit Erde und Mond
    let list = orbit2bodies::get_orbit_2_bodies_2d(5972400000000000000000000.0,
                                                   (0.0, 0.0), (0.0, 0.0),
                                                   73460000000000000000000.0,
                                                   (384400000.0, 0.0), (0.0, 1022.0,),
                                                   1000.0, 2390);
    fileprinter::write_to_orbit_2d_file("Monat", "Erde", "Mond", list);
    // Versuch mit Erde und Sonne
    let list2 = orbit2bodies::get_orbit_2_bodies_2d(5972400000000000000000000.0,
                                                   (147100000000.0, 0.0), (0.0, 30290.0),
                                                   1988400000000000000000000000000.0,
                                                   (0.0, 0.0), (0.0, 0.0,),
                                                   1000.0, 31536);
    fileprinter::write_to_orbit_2d_file("Jahr", "Erde", "Sonne", list2);
}

#[allow(dead_code)]
fn test_simple_heun(){
    // Vergleich Heun und Euler
    let list = simpleheun::get_list((0.0, 0.0), (100.0, 100.0), 1.0);
    fileprinter::write_to_file("heun.txt", list);
    let data = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 1.0);
    fileprinter::write_to_file("euler.txt", data);
    let list2 = simpleheun::get_list((0.0, 0.0), (100.0, 100.0), 0.5);
    fileprinter::write_to_file("heun05.txt", list2);
    let data2 = simplethrow::get_list((0.0, 0.0), (100.0, 100.0), 0.5);
    fileprinter::write_to_file("euler05.txt", data2);
}

#[allow(dead_code)]
fn test_heun_stokes(){
    let list = heunstokesthrow::get_list((0.0,0.0), (100.0,100.0), 1.0, 1.0, 1.0);
    fileprinter::write_to_file("heunstokes1.txt", list);
    let list2 = heunstokesthrow::get_list((0.0,0.0), (100.0,100.0), 1.0, 0.5, 1.0);
    fileprinter::write_to_file("heunstokes05.txt", list2);
    let list3 = heunstokesthrow::get_list((0.0,0.0), (100.0,100.0), 1.0, 2.0, 1.0);
    fileprinter::write_to_file("heunstokes2.txt", list3);

    let list = heunstokesthrow::get_list((0.0,0.0), (2000.0,2000.0), 1.0, 0.1, 1.0);
    fileprinter::write_to_file("heunstokes01.txt", list);
    let list2 = heunstokesthrow::get_list((0.0,0.0), (2000.0,2000.0), 1.0, 1.0, 1.0);
    fileprinter::write_to_file("heunstokes1.txt", list2);
    let list3 = heunstokesthrow::get_list((0.0,0.0), (2000.0,2000.0), 1.0, 10.0, 1.0);
    fileprinter::write_to_file("heunstokes10.txt", list3);
    let list3 = heunstokesthrow::get_list((0.0,0.0), (2000.0,2000.0), 1.0, 100.0, 1.0);
    fileprinter::write_to_file("heunstokes100.txt", list3);
}

#[allow(dead_code)]
fn test_heun_ohne_stokes(){
    let list = heunstokesthrow::get_list((0.0,0.0), (1000.0,1000.0), 1.0, 5.0, 1.0);
    fileprinter::write_to_file("heunUndStokes.txt", list);
    let list = simpleheun::get_list((0.0, 0.0), (1000.0, 1000.0), 5.0);
    fileprinter::write_to_file("nurHeun.txt", list);
}

#[allow(dead_code)]
fn test_heun_stokes_massen(){
    let list = heunstokesthrow::get_list((0.0,0.0), (1000.0,1000.0), 1.0, 5.0, 1.0);
    fileprinter::write_to_file("heunStokes1kg.txt", list);
    let list = heunstokesthrow::get_list((0.0,0.0), (1000.0,1000.0), 1.0, 5.0, 10.0);
    fileprinter::write_to_file("heunStokes10kg.txt", list);
    let list = simpleheun::get_list((0.0, 0.0), (1000.0, 1000.0), 5.0);
    fileprinter::write_to_file("nurHeun.txt", list);
}