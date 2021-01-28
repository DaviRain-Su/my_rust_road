extern { fn hello(); }

fn main() {
    unsafe  {
        hello();
    }
    println!("Hello, world!");
}
