extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::NaiveDateTime;
use serde::{Deserializer, Serializer};

#[derive(Debug)]
enum DateType {
    FirstType,
    SecondType,
}

#[derive(Debug, Serialize, Deserialize)]
struct MyDate {
    timestamp: NaiveDateTime,
    #[serde(rename = "type", deserialize_with = "deserialize_date_type")]
    #[serde(serialize_with = "serialize_date_type")]
    date_type: DateType,
}

fn main() {
    let json = r#"{
        "timestamp": "2018-01-16T15:43:04",
        "type": 1
    }"#;

    let in_rust: MyDate = serde_json::from_str(json).expect("JSON parsing failed");
    println!("In Rust: {:?}", in_rust);

    let back_to_json = serde_json::to_string_pretty(&in_rust).expect("Rust to JSON failed");
    println!("In JSON: {}", back_to_json);
}

fn deserialize_date_type<'de, D>(deserializer: D) -> Result<DateType, D::Error>
where
    D: Deserializer<'de>,
{
    use std::fmt;
    use serde::de::{self, Visitor};

    struct DateTypeVisitor;

    impl<'de> Visitor<'de> for DateTypeVisitor {
        type Value = DateType;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer between 1 and 2")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                1 => Ok(DateType::FirstType),
                2 => Ok(DateType::SecondType),
                _ => {
                    let error = format!("type out of range: {}", value);
                    Err(E::custom(error))
                }
            }
        }

        // Similar for other methods, if you want:
        //   - visit_i8
        //   - visit_i16
        //   - visit_i32
        //   - visit_i64
        //   - visit_u8
        //   - visit_u16
        //   - visit_u32
    }

    deserializer.deserialize_u64(DateTypeVisitor)
}

fn serialize_date_type<S>(date_type: &DateType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(match *date_type {
        DateType::FirstType => 1,
        DateType::SecondType => 2,
    })
}
