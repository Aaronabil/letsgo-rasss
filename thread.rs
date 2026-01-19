use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..=5{
            println!("Hallo Coy dari thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..=5{
        println!("Hallo Coy dari main: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    handle.join().unwrap();
}