use std::io::{self, Write};

fn main() {
    println!("--- keygenme ---");

    let username = get_input("Masukkan Username: ");

    if username.is_empty() {
        println!("Username tidak boleh kosong");
        return;
    }

    let total_ascii: u32 = username.chars().map(|c| c as u32).sum();

    let first_char_val = username.chars().next().unwrap() as u32;
    let magic_val = first_char_val * 3;

    let xor_result = total_ascii ^ magic_val;

    let shift_amount = 10;
    let expected_key = xor_result << shift_amount;

    let input_key_str = get_input("Masukkan Serial Key: ");

    let input_key: u32 = match input_key_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Serial Key harus berupa angka");
            return;
        }
    };

    if input_key == expected_key {
        println!("Good Job. this is flag: h4yyuk_g45k4n_y4h4h4_34afd62");
    } else {
        println!("Wrong.");
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); 
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Gagal membaca input");
    buffer.trim().to_string()
}