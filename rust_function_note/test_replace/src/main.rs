fn main() {
    let mut v = vec![1, 2, 3];
    let old = std::mem::replace(&mut v, vec![3, 4, 5, 6, 7]);
    println!("v = {:?}, old = {:?}", v, old);
}
