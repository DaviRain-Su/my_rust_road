fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn main() {
    let ret = build_vector();
    println!("ret = {:?}", ret);
}
