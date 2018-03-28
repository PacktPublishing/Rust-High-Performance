#[macro_use]
extern crate type_name_derive;

trait TypeName {
    fn type_name() -> &'static str;
}

#[derive(TypeName)]
struct Alice;

#[derive(TypeName)]
enum Bob {}

fn main() {
    println!("Alice's name is {}", Alice::type_name());
    println!("Bob's name is {}", Bob::type_name());
}
