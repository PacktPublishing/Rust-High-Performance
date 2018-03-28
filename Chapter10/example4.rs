use std::thread;

fn main() {
    println!("Before the thread!");

    let handle = thread::Builder::new()
        .name("bad thread".to_owned())
        .spawn(|| {
            panic!("Panicking inside the thread!");
        })
        .expect("could not create the thread");
    println!("After thread spawn!");

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }
    println!("After everything!");
}
