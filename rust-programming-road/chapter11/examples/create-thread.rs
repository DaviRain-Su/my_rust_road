use std::thread;

fn main() {
    let mut v = vec![];

    for id in 0..5 {
        let child = thread::spawn(move || {
            println!("in child: {}", id);
        });

        v.push(child);
    }

    println!("in main: join before");
    for child in v {
        child.join().unwrap();
    }
    println!("in mian: join after");
}
