fn divide(dividend: u32, divisor: u32) -> Option<u32> {
    if divisor == 0u32 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    //   let result = divide(100, 0).unwrap();
    //  println!("{:?}", result);
    let result = divide(100, 2).unwrap();
    println!("{:?}", result);
}
