// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;
use futures::join;

/// async/.await 是Rust内置的语法，用于让异步函数编写的像
/// 同步代码。async将代码块转化成实现了Future Trait的状态机。
/// 使用同步方法调用阻塞函数会阻塞整个线程，但阻塞Future只会出让
/// (yield)线程控制权，让其他Future继续执行。
fn main() {
    let future = do_something();
    block_on(future);

    // example-1
    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());

    // example-2
    block_on(async_main());

    // 在这个示例中，唱歌之前必须要学习唱这首歌，但是学习唱歌和唱歌
    // 都可以和跳舞同时发生，如果我们用了block_on(learning_song())
    // 而不是learn_and_sing中的learn_song().await, 那么当learn_song在
    // 执行时线程将无法做的别的事情，这也使得无法同时跳舞。
    // 但是通过.await执行learn_song的future， 我们就可以在learn_song阻塞时
    // 让其他任务来掌握当前线程、这样可以做到在单线程并发执行多个future到完成状态.
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    join!(f1, f2);
}

async fn do_something() {
    let (a, b) = (2, 3);
    let sum = sum(a, b).await;
    let mul = mul(a, b).await;
    let div = div(a, b).await;
    let mod_val = mod_compute(a, b).await;
    println!(
        "sum = {},\nmul = {},\ndiv = {},\nmod_val = {}\n",
        sum, mul, div, mod_val
    );
}

// 在async fn 函数中，可以使用.await来等待其他实现了Future Trait的类型完成。
// 和block_on不同，.await不会阻塞当前线程，而是异步等待future完成，在当前
// future无法进行下去时，允许其他任务运行。
async fn sum(a: i32, b: i32) -> i32 {
    a + b
}

async fn mul(a: i32, b: i32) -> i32 {
    a * b
}

async fn div(a: i32, b: i32) -> i32 {
    a / b
}

async fn mod_compute(a: i32, b: i32) -> i32 {
    a % b
}

async fn learn_song() -> String {
    println!("song");
    format!("song")
}

async fn sing_song(par: String) {
    println!("{}", par);
    println!("sing song");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance() {
    println!("dance");
}
