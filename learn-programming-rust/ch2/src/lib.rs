pub mod gcd;

#[test]
fn test_overflow() {
    let big_val = std::i32::MAX;

    let x = big_val.checked_add(1).expect("add overflow!");
    // let x = big_val.wrapping_add(1);

    println!("x = {}", x);
}

#[test]
fn test_type_inference() {
    fn build_vector() -> Vec<i16> {
        let mut v = Vec::new();
        v.push(10);
        v.push(20);
        v
    }

    let ret = build_vector();
    println!("ret = {:?}", ret);
}
