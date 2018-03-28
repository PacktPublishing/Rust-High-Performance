#[derive(Debug, Default)]
struct MyData {
    field1: String,
    field2: Vec<u32>,
    field3: i32,
}

// impl Default for MyData {
//     fn default() -> Self {
//         Self {
//             field1: Default::default(),
//             field2: Default::default(),
//             field3: Default::default(),
//         }
//     }
// }

fn main() {
    let test1 = MyData {
        field1: "sth".to_owned(),
        ..Default::default()
    };
    let test2 = MyData::default();

    println!("test1: {:?}", test1);
    println!("test2: {:?}", test2);

    let test3: MyData = Default::default();
}
