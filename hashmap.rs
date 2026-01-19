use std::collections::HashMap;

fn main(){
    let mut skor = HashMap::new();
    skor.insert(String::from("Nabil"), 90);
    skor.insert(String::from("Haya"), 80);

    println!("{:?}", skor);

    if let Some(nilai) = skor.get("Nabil") {
        println!("Nilai Nabil: {}", nilai);
    } 

    for (nama, nilai) in &skor {
        println!("{} nilainya {}", nama, nilai);
    }
}