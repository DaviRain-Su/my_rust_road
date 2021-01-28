use async_std::task;

async fn say_hello() {
    println!("hello, wrold");
}
fn main() {
    task::block_on(say_hello());
    println!("Hello, world!");
}
