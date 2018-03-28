// Note: it needs GEN_MAP=true to generate the HashMap.

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref MY_MAP: HashMap<&'static str, &'static str> = {
        use std::env;

        let mut map = HashMap::new();
        if let Ok(val) = env::var("GEN_MAP") {
            if val == "true" {
                map.insert("firstKey", "firstValue");
                map.insert("secondKey", "secondValue");
            }
        }

        map
    };
}

fn main() {
    for (key, value) in MY_MAP.iter() {
        println!("{}: {}", key, value);
    }
}
