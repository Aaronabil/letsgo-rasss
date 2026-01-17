struct Person{
    name: String,
    age: u32,
    is_active: bool,
}

struct Color(i32, i32, i32);

struct Marker;

fn main(){
    let mut user1 = Person {
        name: String::from("Nabil"),
        age: 19,
        is_active: true,
    };

    user1.age = 20;

    let user2 = Person {
        name: String::from("Haya"),
        ..user1
    };

    println!("Nama: {}, Umur Terbaru: {}, Status: {}", user1.name, user1.age, user1.is_active);

    println!("Nama: {}, Umur Terbaru: {}, Status: {}", user2.name, user2.age, user2.is_active);

    let name = String::from("Kagura");
    let age = 19;
    let is_active = true;

    let user3 = Person {
        name,
        age,
        is_active,
    };

    println!("Nama: {}, Umur: {}, Status: {}", user3.name, user3.age, user3.is_active);

    let black = Color(0, 0, 0);
    println!("Warna: {}, {}, {}", black.0, black.1, black.2);

    let _m = Marker;
}