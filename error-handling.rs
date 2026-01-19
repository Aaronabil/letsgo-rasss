use std::fs::File;

fn main(){
    // panic!("Terjadi error fatal")
    let file = File::open("data.txt");

    match file {
        Ok(_f) => println!("File berhasil terbuka"),
        Err(e) => println!("Terjadi error: {}", e),
    }
}