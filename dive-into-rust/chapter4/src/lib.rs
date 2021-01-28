
#[test]
fn test_const_fn() {
    const fn cube(num: usize) -> usize {
        num * num * num
    }

    const DIM : usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM];
    println!("{:?}", ARR   );
}