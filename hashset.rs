use std::collections::HashSet;

fn main(){
    let mut buah = HashSet::new();
    buah.insert("jeruk");
    buah.insert("apel");
    buah.insert("melon");

    println!("{:?}", buah);

    if buah.contains("jeruk"){
        println!("Jeruk ada");
    } else {
        println!("Jeruk gada")
    }

    for n in buah {
        println!("Ada buah {}", n);
    }
}