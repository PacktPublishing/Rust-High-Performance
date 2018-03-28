extern crate phf;

include!(concat!(env!("OUT_DIR"), "/phf.rs"));

fn main() {
    println!("{}", MAP.get("key1").unwrap());
}
