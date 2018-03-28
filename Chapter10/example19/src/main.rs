extern crate num_cpus;
extern crate threadpool;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::with_name("my worker".to_owned(), num_cpus::get());
    println!("Pool threads: {}", pool.max_count());

    let result = Arc::new(AtomicUsize::new(0));

    for i in 0..1_0000_000 {
        let t_result = result.clone();
        pool.execute(move || {
            t_result.fetch_add(i, Ordering::Relaxed);
        });
    }

    pool.join();

    let final_res = Arc::try_unwrap(result).unwrap().into_inner();
    println!("Final result: {}", final_res);
}
