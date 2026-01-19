fn main(){
    let mut angka: Vec<i32> = Vec::new();
    angka.push(10);
    angka.push(20);
    angka.push(30);

    println!("Isi vektor: {:?}", angka);

    println!("Elemen pertama: {}", angka[0]);

    for n in &angka {
        println!("Angka: {}", n);
    } 

    // vector bisa dibuat dengan macro vec! agar lebih singkat
    let angka2 = vec![10, 20, 30, 40, 50];
    println!("{:?}", angka2)
}