use std::thread;

struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        if thread::panicking() {
            println!("The thread is panicking with the {} struct!", self.name);
        } else {
            println!("The {} struct is out of scope :(", self.name);
        }
    }
}

fn main() {
    let my_struct = MyStruct {
        name: "whole program".to_owned(),
    };

    {
        let scoped_struct = MyStruct {
            name: "scoped".to_owned(),
        };
    }

    let handle = thread::Builder::new()
        .name("bad thread".to_owned())
        .spawn(|| {
            let thread_struct = MyStruct {
                name: "thread".to_owned(),
            };
            panic!("Panicking inside the thread!");
        })
        .expect("could not create the thread");
    println!("After thread spawn!");

    if handle.join().is_err() {
        println!("Something bad happened :(");
    }
    println!("After everything!");
}
