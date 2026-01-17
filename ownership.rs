fn main(){
    let s1 = String::from("Coy");
    let s2 = s1;

    println!("Hasil: {}", s2);

    let s = String::from("Coy 2");
    ambil_ownership(s);

    let x = kembalikan();
    println!("Hasil ownership return value: {}", x);
}

fn ambil_ownership(str: String){
    println!("Hasil ambil ownership: {}", str);
}

fn kembalikan() -> String{
    let str = String::from("Coy 3");
    str
}
