use std::io::{self, Read};

fn main() -> io::Result<()> {

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let buffer: Vec<&str> = buffer.trim().split('\n').collect();
    println!("{:?}", buffer);
    Ok(())
}