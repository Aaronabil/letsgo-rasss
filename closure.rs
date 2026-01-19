fn main(){
    let tambah = |a: i32, b: i32| -> i32 { a + b };
    println!("Hasil: {}", tambah(2, 3));

    let kali = |x, y| x * y;
    println!("2 kali 3 = {}", kali(2, 3));

    let factor = 10;
    let multiply = |x| x * factor;

    println!("2 * 10 = {}", multiply(2));

    let angka = vec![1, 2, 3, 4, 5];
    
    let hasil: Vec<i32> = angka.iter()
    .map(|x| x * 2)
    .filter(|x| x > &5)
    .collect();

    println!("Hasil {:?}", hasil);

    let pair = |a, b| (a, b);

    let hasil2 = pair(1, "Halo");
    println!("Pair: {:?}", hasil2);

    let tambah_satu = |x| x + 1;
    let kali_dua = |x| x * 2;

    println!("4 + 1 = {}", tambah_satu(4));
    println!("5 * 2 = {}", kali_dua(5));
}