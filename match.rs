fn main(){
    let number = 6;

    match number{
        1 => println!{"Satu"},
        2 | 3 => println!{"Dua atau Tiga"},
        4..=10 => println!("Dari 4 hingga 10"),
        _ => println!("Selain itu")
    }
}