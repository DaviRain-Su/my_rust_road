use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use log::{debug, error, log_enabled, info, Level};
mod threadloop;
mod cli;


fn main() {
    env_logger::init();
    let config = cli::Config::new("./Cargo.toml");
    // Use RUST_LOG=debug cargo run
    debug!("config = {:?}", config);

    
    let pool = threadloop::ThreadPool::new(8);
    let test_count = Arc::new(AtomicUsize::new(0));
    for id in 0..42 {
        // let test_count = test_count.clone();
        pool.execute(move || {
            // test_count.fetch_add(1, Ordering::Relaxed);
            debug!("-{:02} -> Hello world", id);
        });
    }

    pool.join();
    // assert_eq!(42, test_count.load(Ordering::Relaxed));
    debug!("result = {}", test_count.load(Ordering::Relaxed));
}
