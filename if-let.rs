fn main(){
    let status = Some(5);

    if let Some(x) = status {
        println!("Angka: {}", x)
    } else {
        println!("Bukan angka")
    }
}