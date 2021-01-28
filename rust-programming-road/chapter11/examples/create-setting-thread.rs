use std::panic;
use std::thread::{current, Builder};
fn main() {
    let mut v = vec![];

    for id in 0..5 {
        let thread_name = format!("child-{}", id);
        let size: usize = 3 * 1024;
        let builder = Builder::new().name(thread_name).stack_size(size);

        let child = builder
            .spawn(move || {
                println!("in child: {}", id);
                if id == 3 {
                    panic::catch_unwind(|| {
                        panic!("oh no!");
                    })
                    .unwrap();
                    println!("in {} do sm", current().name().unwrap());
                }
            })
            .unwrap();
        v.push(child);
    }

    for child in v {
        child.join().unwrap();
    }
}
