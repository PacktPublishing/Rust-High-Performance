use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Hash, PartialEq, Eq)]
struct MyData {
    field1: String,
    field2: Vec<u32>,
    field3: i32,
}

fn main() {
    let key1 = MyData {
        field1: "myField".to_owned(),
        field2: vec![0, 1, 2],
        field3: 1898,
    };

    let key2 = key1.clone();

    let key3 = MyData {
        field1: "myField2".to_owned(),
        field2: vec![5, 3, 1],
        field3: 2345,
    };

    let mut map = HashMap::new();
    map.insert(key1, "MyFirst");

    assert!(map.get(&key2).is_some());
    assert!(map.get(&key3).is_none());
}

impl Hash for MyData {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.field1.hash(state);
        self.field2.hash(state);
        self.field3.hash(state);
    }
}
