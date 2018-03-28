extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    field1: String,
    field2: u32,
    field3: Vec<u8>,
}

fn main() {
    let example = MyData {
        field1: "Test field".to_owned(),
        field2: 33_940,
        field3: vec![65, 22, 96, 43],
    };

    let json = serde_json::to_string_pretty(&example).expect("could not generate JSON string");
    println!("JSON:");
    println!("{}", json);

    let example_back: MyData = serde_json::from_str(&json).expect("could not parse JSON string");
    println!("Back from JSON:");
    println!("{:?}", example_back);
}
