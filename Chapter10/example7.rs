use std::thread;

fn main() {
    let my_vec = vec![10, 33, 54];

    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            println!("This is my vector: {:?}", my_vec);
        })
        .expect("could not create the thread");

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }
}
