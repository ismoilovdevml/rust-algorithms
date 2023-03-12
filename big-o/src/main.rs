use std::io;

fn main() {
    println!("CPU Test");
    println!("Qancha ma\'lumotlar bilan sinab ko'rilsin mlrd:\n");
    println!("CPU GHzsini kiriting:\n");

    let mut malumot = String::new();
    io::stdin().read_line(&mut malumot).unwrap();
    let malumot = malumot.trim().parse::<f64>().unwrap();

    let data_size = malumot * 1e9;

    let mut ghz = String::new();
    io::stdin().read_line(&mut ghz).unwrap();
    let ghz = ghz.trim().parse::<f64>().unwrap();

    let bitta_ghz = ghz * 10e9;
    let bitta_ghz_mlrd = bitta_ghz / 10e9;
    let bitta_o_ns = 1.0 / bitta_ghz * 1e9;
    let bitta_o_s = 1.0 / bitta_ghz;

    let binary_search_v_ns = bitta_o_ns * (data_size * (f64::log2(data_size) / data_size));
    let binary_search_v_s = binary_search_v_ns / 1e9;
    let linear_search_v_ns = bitta_o_ns * data_size;
    let linear_search_v_s = linear_search_v_ns / 1e9;

    println!("Sining CPUyingiz {} mlrd malumotlar bilan test qilindi\n", malumot);
    println!("Sizning CPUyingiz 1 sekundda {} ta yoki {} mlrd operatsiya bajaradi\n", bitta_ghz, bitta_ghz_mlrd);
    println!("Bitta Operatsiya uchun {} nanosekund yoki {} sekund vatq sarflaydi\n", bitta_o_ns, bitta_o_s);
    println!("Binary Search uchun {} nanosekund yoki {} sekund vaqt sarflaydi\n", binary_search_v_ns, binary_search_v_s);
    println!("Linear Search Uchun {} nanosekund yoki {} sekund vaqt sarflaydi\n", linear_search_v_ns, linear_search_v_s);
}