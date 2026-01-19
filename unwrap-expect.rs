use std::fs::File;
use std::io::{self, Read};

fn baca_file() -> Result<String, io::Error>{
    let mut file = File::open("dat.txt")?;
    let mut isi = String::new();
    file.read_to_string(&mut isi)?;
    Ok(isi)
}

fn main(){
    // let file = File::open("data.txt").unwrap(); 
    // let file = File::open("dat.txt").expect("Gagal membuka file"); // jadi expect disini biar keliatan errornya apa, contohnya di unwrap kalo gapake dia bakal panic!

    match baca_file() {
        Ok(text) => println!("Isi file: {}", text),
        Err(e) => println!("Terjadi error: {}", e)
    }

    // println!("File berhasil terbuka: {:?}", file)
}