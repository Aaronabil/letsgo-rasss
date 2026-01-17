fn main(){
    //String Slice (&str)
    let s: &str = "Cihuy";
    println!("Hasil: {}", s);

    let t = String::from("MangEak");
    let bagian = &t[0..4];
    println!("Slice: {}", bagian);

    let mut x = String::from("Hallo, ");
    x.push_str(bagian);
    println!("{}",x);

    let nama = String::from("Nabil");
    meet(&nama);
    meet("Kagura");

    let angka = [1, 2, 3, 4, 5];
    let potongan = &angka[1..4];

    for n in potongan{
        println!("{}", n);
    }
    
}

fn meet(name: &str){
    println!("Halo, {}", name);
}