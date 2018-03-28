use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

fn main() {
    let my_val = AtomicUsize::new(0);
    let pointer = Arc::new(my_val);
    let lock = Arc::new(AtomicBool::new(false));

    let t_pointer = pointer.clone();
    let t_lock = lock.clone();
    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            for _ in 0..250_000 {
                while t_lock.compare_and_swap(false, true, Ordering::Relaxed) {}
                let cur_value = t_pointer.load(Ordering::Relaxed);
                let sum = cur_value + 1;
                t_pointer.store(sum, Ordering::Relaxed);
                t_lock.store(false, Ordering::Relaxed);
            }
        })
        .expect("could not create the thread");

    for _ in 0..250_000 {
        while lock.compare_and_swap(false, true, Ordering::Relaxed) {}
        let cur_value = pointer.load(Ordering::Relaxed);
        let sum = cur_value + 1;
        pointer.store(sum, Ordering::Relaxed);
        lock.store(false, Ordering::Relaxed);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let a_int = Arc::try_unwrap(pointer).unwrap();
    println!("Final number: {}", a_int.into_inner());
}
