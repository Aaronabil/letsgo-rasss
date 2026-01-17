fn main(){
    let s1 = String::from("Hola");
    cetak(&s1);
    println!("Masih bisa pakai s1: {}", s1);

    let mut x = String::from("Hola 2");
    ubah(&mut x);
    println!("Setelah diubah: {}", x);

    // let mut z = String::from("Hola 4");
    // let a1 = &z;
    // let a2 = &z;
    // let a3 = &mut z; // error: tidak boleh ada mutable ref jika masih ada immutable ref

    // println!("{}, {}", a1, a2);
}

fn cetak(s: &String){
    println!("Isi: {}", s)
}

fn ubah(y: &mut String){
    y.push_str(" Hola 3")
}
