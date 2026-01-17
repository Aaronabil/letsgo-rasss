fn main(){
    let hasilkali = kali(5, 2);
    println!("Hasil perkalian: {}", hasilkali);

    let hasilbagi = bagi(10, 0);
    println!("Hasil pembagian: {}", hasilbagi);

    let luas = hitung_luas(10, 5);
    println!("Hasil hitung luas: {}", luas);
}

fn kali(a: i32, b: i32) -> i32 {
    a * b
} // return value

fn bagi(a:i32, b:i32) -> i32 {
    if b == 0 {
        return 0;
    }

    a / b
} // keyword return

fn hitung_luas(panjang: i32, lebar:i32) -> i32 {
    panjang * lebar
}

