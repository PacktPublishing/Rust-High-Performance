use std::fmt;
use std::cmp::Ordering;

// #[derive(Debug)]
struct MyData {
    field1: String,
    field2: u64,
}

impl fmt::Debug for MyData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MyData {{ field1: \"{}\", field2: {} }}",
            self.field1, self.field2
        )
    }
}

fn main() {
    let data = MyData {
        field1: "This is my string".to_owned(),
        field2: 4402,
    };

    println!("Data: {:?}", data);
}


// ------------
#[derive(Eq)]
struct DateNotes {
    day: u8,
    month: u8,
    year: i32,
    comment: String,
}

impl PartialEq for DateNotes {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}

impl Ord for DateNotes {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => self.day.cmp(&other.day),
                o => o,
            },
            o => o,
        }
    }
}

impl PartialOrd for DateNotes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ------------
// #[derive(Clone, Copy)]
// struct MyData {
//     field1: u64,
//     field2: f64,
//     field3: i32,
// }

// ------------
// #[derive(Clone)]
// struct MyData {
//     field1: String,
//     field2: Vec<u32>,
//     field3: i32,
// }
