async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // Calling 'say_world()' does not execute the body of "say_world"
    let op = say_world();

    // This println! comes first
    println!("Hello, world!");

    // Calling '.await?' on 'Op' starts executing 'say_world'
    op.await;
}
