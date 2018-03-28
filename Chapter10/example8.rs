use std::thread;
use std::sync::Arc;

fn main() {
    let my_vec = vec![10, 33, 54];
    let pointer = Arc::new(my_vec);

    let t_pointer = pointer.clone();
    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            println!("Vector in second thread: {:?}", t_pointer);
        })
        .expect("could not create the thread");

    println!("Vector in main thread: {:?}", pointer);

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }
}
