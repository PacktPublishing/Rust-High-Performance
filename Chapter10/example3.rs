use std::thread;

fn main() {
    println!("Before the thread!");

    let handle = thread::spawn(|| {
        println!("Inside the thread!");
    });
    println!("After thread spawn!");

    handle.join().expect("the thread panicked");
    println!("After everything!");
}
