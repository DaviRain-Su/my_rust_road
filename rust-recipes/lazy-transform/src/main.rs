pub mod lazy_transfrom;
fn main() {
    let fn_cal = |val: u32| Some(val + 1);
    let mut call = lazy_transfrom::LazyTransform::new(fn_cal);
    // println!("Hello, world!");
    std::thread::spawn(move || { // call moved here 
        for i in 0..10_000 { // value moved into closure here
            call.set_source(i); // variable moved due to use in closure
        }
    });

    while call.get_transformed().is_none() { //  call used after move 
        // value borrowed here after move
    }

    let val = call.get_transformed().unwrap();
    assert!(val >= 0 && val < 10_000);
}
