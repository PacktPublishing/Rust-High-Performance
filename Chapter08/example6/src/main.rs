#[macro_use]
extern crate derive_builder;

use std::path::PathBuf;

#[derive(Default, Debug, Builder)]
#[builder(setter(into), default)]
struct MyData {
    field1: u8,
    field2: PathBuf,
    field3: String,
}

fn main() {
    let data = MyDataBuilder::default()
        .field2("path/to/file.png")
        .field3("Some string")
        .build()
        .unwrap();

    println!("{:?}", data);
}
