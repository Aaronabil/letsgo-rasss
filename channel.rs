use std::sync::mpsc;
use std::thread;

fn main(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let pesan = String::from("Hallo coy dari thread");
        tx.send(pesan).unwrap();
    });

    let diterima = rx.recv().unwrap();
    println!("Pesan diterima: {}", diterima);
}