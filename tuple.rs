fn main() {
    let person: (&str, i32, bool) = ("Nabil", 19, true);
    println!("Nama: {}, Umur: {}, Aktif: {}", person.0, person.1, person.2);
}
