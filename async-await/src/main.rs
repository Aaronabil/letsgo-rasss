use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let t1 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Tugas 1 selesai");
    };

    let t2 = async {
        sleep(Duration::from_secs(2)).await;
        println!("Tugas 2 selesai");
    };

    tokio::join!(t1, t2);
    println!("Semua tugas selesai");
}