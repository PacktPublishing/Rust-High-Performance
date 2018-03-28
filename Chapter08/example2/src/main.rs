extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MyData {
    first_data: u32,
    second_data: String,
    third_data: f32,
}

fn main() {
    let json = r#"{
        "FirstData": 56,
        "SecondData": "hello, world",
        "ThirdData": -1.23
    }"#;

    let in_rust: MyData = serde_json::from_str(json).expect("JSON parsing failed");
    println!("In Rust: {:?}", in_rust);

    let back_to_json = serde_json::to_string_pretty(&in_rust).expect("Rust to JSON failed");
    println!("In JSON: {}", back_to_json);
}
