fn main() {
    let mut times = 2;
    {
        // This is in new scope
        let mut borrow = |x| times += x;
        borrow(5);
    }

    println!("times = {}", times);

    let mut own = move |x| {
        times += x;
        println!("times = {}", times);
    };
    own(5);

    assert_eq!(times, 7);
    println!("times = {}", times);
}
