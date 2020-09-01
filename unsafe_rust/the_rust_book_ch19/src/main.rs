fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        // *r1 = 3;
        *r2 = 3;
        println!("{:?}, {:?}", r1, r2);
        println!("{}, {}", *r1, *r2);

        let r3 : *const i32 = std::ptr::null_mut();
        // println!("{}, {:?}",*r3, r3);
        println!("{:?}", r3); //0x0

        let address = 0x012345usize;
        let r = address as *const i32;
        // println!("{}", *r); //不会打印后面的也打印不了, 去掉就可以打印了
        println!("{:?}",r); // 0x12345

    }
    unsafe {
        dangerous();
    }
    // dangerous();

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = my_split_at_mut(r, 3);
    println!("a = {:?}, b = {:?}", a, b);

    // let address = 0x012345usize;
    // let r = address as *mut i32;

    // let slice = unsafe {
    //     std::slice::from_raw_parts(r, 10000)
    // };

    // println!("{:?}", slice);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    
    unsafe {
        println!("COUNT : {}", COUNTER);
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32){
    unsafe {
        COUNTER += inc;
    }
}
unsafe fn dangerous() {
    println!("hello, world!");
}

fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    use std::slice;

    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    
    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..]) // error 

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid), 
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C" {
    fn abs(intput: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

