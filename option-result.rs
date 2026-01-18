fn main(){
    let angka: Option<i32> = Some(10);
    let kosong: Option<i32> = None;

    match angka {
        Some(n) => println!("Angka: {}", n),
        None  => println!("Tidak ada angka"),
    }

    match kosong {
        Some(n) => println!("Angka: {}", n),
        None  => println!("Kosong"),
    }

    let nama: Option<&str> = Some("Coy");

    if let Some(n) = nama {
        println!("Text: {}", n)
    }

    let hasil = bagi(10, 2);
    let gagal = bagi(10, 0);

    match hasil {
        Ok(n) => println!("Hasil: {}", n),
        Err(e) => println!("Error: {}", e)
    }

    match gagal {
        Ok(n) => println!("Hasil: {}", n),
        Err(e) => println!("Error: {}", e)
    }

    let res = hitung();
    println!("{:?}", res);
}

fn bagi(a:i32, b:i32) -> Result<i32, String>{
    if b == 0 {
        Err(String::from("Tidak bisa dibagi dengan 0"))
    } else {
        Ok(a/b)
    }
}

fn hitung() -> Result<(), String> {
    let n1 = bagi(20, 2)?;
    let n2 = bagi(20, 0)?;
    println!("Hasi total: {}", n1 + n2);
    Ok(())
}

