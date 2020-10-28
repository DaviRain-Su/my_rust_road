use tokio::task;
use tokio::task::yield_now;
use std::rc::Rc;

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

    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Her'e vec is {:?}", v);
    })
    .await
    .unwrap();

    tokio::spawn(async  {
        // The scope forces 'rc' to drop before '.await' 
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // 'rc' is no longer used. It is **not** persisted when 
        // the task yeilds to the scheduler 
        yield_now().await;
    }).await.unwrap();
    
    // Tasks are Send when all data that is held across .await calls is Send. 
    /* 
     * tokio::spawn(async {
     *    let rc = Rc::new("hello");
     *        // has type std::rc::Rc<&str> which is not 'Send' 
     *   yield_now().await; 
     *        // await occurs here, with 'rc' maybe used later 
     *
     *    println!("{}", rc);
     *}).await.unwrap(); // 'rc' is later dropped here 
     */
}
