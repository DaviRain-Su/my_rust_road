use std::ops::Add;

#[derive(Debug)]
struct Tuple<T> {
    first: T,
    second: T,
}

impl<T> Tuple<T> {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

//We constrain the possible types of T to those which implement the Add trait
fn sum<T: Add<Output = T>>(tuple: Tuple<T>) -> T {
    tuple.first + tuple.second
}

fn main() {
    let tuple_u32 = Tuple::new(1, 2);
    let tuple_u64 = Tuple::new(34, 23);
    let tuple_string = Tuple::new("23".to_owned(), "34".to_owned());
    println!("tuple_u32 = {:?}", tuple_u32);
    println!("tuple_u64= {:?}", tuple_u64);
    println!("tuple_string = {:?}", tuple_string);
    println!("{}", sum(tuple_u32));
    println!("{}", sum(tuple_u64));
    //    println!("{}", sum(tuple_string));
}
