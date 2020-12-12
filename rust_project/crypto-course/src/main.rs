use std::io;
use crate::crypto::Crypto;


pub mod crypto;

fn input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("cannot read message!!");

    let buffer = buffer.trim();
    let buffer = buffer.replace(" ", "");

    buffer
}

fn main() {

    // shift cipher
    let input_message = input();
    println!("{:?}", input_message);
    let shift = crypto::ShiftCipher;
    let encode_str = shift.encode(&input_message);
    println!("Encode: {}", encode_str);
    let decode_str = shift.decode(&encode_str);
    println!("Decode: {}", decode_str);
    let solution_str = shift.solution(&decode_str);
    for item in solution_str.iter().enumerate() {
        println!("{} -> {}", item.0, item.1);
    }
}
