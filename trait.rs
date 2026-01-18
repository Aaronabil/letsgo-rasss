use std::fmt::{Display, Debug};
use std::ops::Add;

struct Person {
    nama: String,
    age: i32,
}

trait Deskripsi {
    fn deskripsi(&self) -> String;
}

struct Robot;

trait Greet {
    fn say_hello(&self){
        println!("Halo");
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Deskripsi for Person {
    fn deskripsi(&self) -> String {
        format!("{} berusia {}", self.nama, self. age)
    }
}

impl Greet for Robot {}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main(){
    let user = Person {
        nama: String::from("Nabil"),
        age: 19,
    };

    println!("{}", user.deskripsi());

    let r = Robot;
    r.say_hello();

    cetak(10);
    cetak("Coy");

    let angka = 100;
    cetak_info(angka);

    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: 3, y:4};
    let hasil = p1 + p2;

    println!("Hasil: {:?}", hasil)
}

fn cetak<T: Display>(x: T) {
    println!("Isi: {}", x);
}

fn cetak_info<T: Display + Debug>(x: T){
    println!("Display: {}", x);
    println!("Debug: {:?}", x);
}