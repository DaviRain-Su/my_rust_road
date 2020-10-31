use std::thread;
static mut V: i32 = 0;
fn unsafe_seq() -> i32 {
    unsafe {
        V += 1;
        V
    }
}

fn main() {
    let child = thread::spawn(move || {
        for _ in 0..10 {
            unsafe_seq();
            unsafe {
                println!("child: {}", V);
            }
        }
    });

    for _ in 0..10 {
        unsafe_seq();
        unsafe {
            println!("main: {}", V);
        }
    }
    child.join().unwrap();
}
