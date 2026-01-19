fn main(){
    let mut angka = vec![1, 2, 3, 4, 5];

    for n in angka.iter() {
        println!("Angka {}", n);
    }

    for n in angka.iter_mut(){
        *n *= 2;
    }

    println!("{:?}", angka);

    let angka2 = vec![1, 2, 3];

    let total: i32 = angka2.iter().sum();
    println!("Total hasil: {}", total);

    let hasil: Vec<i32> = angka2.iter().map(|x| x * 2).collect();
    println!("Hasil map: {:?}", hasil);

    let genap: Vec<&i32> = angka2.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Angka genap: {:?}", genap);

    let mut ratusan = vec![100, 200, 300].into_iter();

    println!("{:?}", ratusan.next());
    println!("{:?}", ratusan.next());
    println!("{:?}", ratusan.next());
    println!("{:?}", ratusan.next());

    let angka3 = vec![1, 2, 3];

    let iter = angka3.iter().map(|x| {
        println!("Proses {}", x);
        x * 2
    });

    let hasil: Vec<i32> = iter.collect();
    println!("Hasil: {:?}", hasil);
}