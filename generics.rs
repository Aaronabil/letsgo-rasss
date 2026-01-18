struct Point<T> {
    x: T,
    y: T,
}

enum MyOption<T> {
    Some(T),
    None
}

struct Pair<T, U> {
    first: T,
    second: U,
}

fn main(){
    let p1 = Point{x:5, y:10};
    let p2 = Point{x:10, y:5};

    println!("p1 = {}, {}", p1.x, p1.y);
    println!("p2 = {}, {}", p2.x, p2.y);

    let angka = MyOption::Some(100);
    let kosong: MyOption<i32> = MyOption::None;

    match angka {
        MyOption::Some(n) => println!("Angka: {}", n),
        MyOption::None => println!("Tidak ada nilai"),
    }

    
    match kosong {
        MyOption::Some(n) => println!("Angka: {}", n),
        MyOption::None => println!("Kosong"),
    }

    let data1 = Pair{first: 10, second:"Coy"};
    let data2 = Pair{first: 20, second:"Cuy"};

    println!("Pair1: {}, {}", data1.first, data1.second);
    println!("Pair2: {}, {}", data2.first, data2.second);

    cetak(42);
    cetak("Halo");
}

fn cetak<T: std::fmt::Display>(x: T){
    println!("Isi: {}", x)
}