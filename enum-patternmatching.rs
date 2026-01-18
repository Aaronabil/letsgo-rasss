enum OrderStatus {
    Pending,
    Processing,
    Completed,
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move(Point),
    Write(String),
}

fn main(){
    let status = OrderStatus::Processing;

    match status {
        OrderStatus::Pending => println!("Pesanan sedang menunggu"),
        OrderStatus::Processing => println!("Pesanan sedang diproses"),
        OrderStatus::Completed => println!("Pesanan sudah selesai"),
    }

    let c = Shape::Circle(10.0);
    let r = Shape::Rectangle(10.0, 2.0);

    match c {
        Shape::Circle(radius) => println!("Lingkaran dengan jari-jari: {}", radius),
        Shape::Rectangle(_, _) => println!("Bukan lingkaran"),
    }

    match r {
        Shape::Circle(_) => println!("Bukan persegi panjang"),
        Shape::Rectangle(p, l) => println!{"Persegi panjang dengan {} x {}", p, l}
    }

    let msg1 = Message::Quit;
    let msg2 = Message::Move(Point {x: 10, y:20});
    let msg3 = Message::Write(String::from("Coy"));

    match msg2 {
        Message::Quit => println!("Keluar"),
        Message::Move(p) => println!("Pindah ke {}, {}", p.x, p.y),
        Message::Write(text) => println!("Pesan: {}",text),
    }
}