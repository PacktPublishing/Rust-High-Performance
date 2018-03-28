use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let my_vec = Arc::new(Mutex::new(Vec::new()));

    let t_vec = my_vec.clone();
    let handle = thread::Builder::new()
        .name("my thread".to_owned())
        .spawn(move || {
            for i in 0..10 {
                let mut vec = t_vec.lock().unwrap();
                vec.push(i);
                panic!("Panicking the secondary thread");
            }
        })
        .expect("could not create the thread");

    thread::sleep(Duration::from_secs(1));

    for i in 0..10 {
        let mut vec = match my_vec.lock() {
            Ok(g) => g,
            Err(e) => {
                println!("The secondary thread panicked, recovering…");
                e.into_inner()
            }
        };
        vec.push(i);
    }

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }

    let vec_mutex = Arc::try_unwrap(my_vec).unwrap();
    let f_vec = match vec_mutex.into_inner() {
        Ok(g) => g,
        Err(e) => {
            println!("The secondary thread panicked, recovering…");
            e.into_inner()
        }
    };
    println!("Final vector: {:?}", f_vec);
}
