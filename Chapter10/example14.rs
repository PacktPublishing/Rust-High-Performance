use std::thread;
use std::sync::mpsc::*;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel();

    let handles: Vec<_> = (1..6)
        .map(|i| {
            let t_sender = sender.clone();
            thread::Builder::new()
                .name(format!("sender-{}", i))
                .spawn(move || {
                    t_sender.send(format!("Hello from sender {}!", i)).unwrap();
                })
                .expect("could not create the thread")
        })
        .collect();

    while let Ok(message) = receiver.recv_timeout(Duration::from_secs(1)) {
        println!("{}", message);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Finished");
}
