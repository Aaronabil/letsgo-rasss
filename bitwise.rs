fn main(){
    let a = 0b1010; // 10
    let b = 0b1100; // 12

    println!("AND: {:b}", a & b);  
    println!("OR: {:b}", a | b);   
    println!("XOR: {:b}", a ^ b);  
    println!("Shift Left: {:b}", a << 1);
    println!("Shift Right: {:b}", b >> 2);
}

//     println!("{}", x);      // default
//     println!("{:b}", x);    // binary
//     println!("{:o}", x);    // octal
//     println!("{:x}", x);    // hex
