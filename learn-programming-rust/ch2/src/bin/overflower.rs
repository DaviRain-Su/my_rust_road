// #[deny(arithmetic_overflow)]
fn main() {
    let big_val = std::i32::MAX;

    let x = big_val.checked_add(1).expect("add overflow!");
    // let x = big_val.wrapping_add(1);

    println!("x = {}", x);
}

