mod math;
mod utils;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let hasil_tambah = math::tambah(10, 2);
    let hasil_kali = math::kali(10, 3);
    println!("Hasil tambah: {}", hasil_tambah);
    println!("Hasil kali: {}", hasil_kali);

    utils::string_utils::cetak("Coy");

    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(1..=10);
    println!("Angka acak: {}", n);
}
