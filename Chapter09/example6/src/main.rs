#[macro_use]
extern crate getset_derive;

#[derive(Debug, Getters, Setters)]
struct Alice {
    x: String,
    y: u32,
}

fn main() {
    let mut alice = Alice {
        x: "this is a name".to_owned(),
        y: 34,
    };
    println!("Alice: {{ x: {}, y: {} }}", alice.x(), alice.y());

    alice.set_x("testing str");
    alice.set_y(15u8);
    println!("{:?}", alice);
}
