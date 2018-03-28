use std::fmt::Display;

struct StringData {
    data: String,
}

struct NumberData {
    data: i32,
}

trait ShowInfo {
    type Out: Display;
    fn info(&self) -> Self::Out;
}

impl<'sd> ShowInfo for &'sd StringData {
    type Out = &'sd str;
    fn info(&self) -> Self::Out {
        self.data.as_str()
    }
}

impl ShowInfo for NumberData {
    type Out = i32;
    fn info(&self) -> Self::Out {
        self.data
    }
}

fn print<I: ShowInfo>(data: I) {
    println!("{}", data.info());
}

fn main() {
    let str_data = StringData {
        data: "This is my data".to_owned(),
    };
    let num_data = NumberData { data: 34 };

    print(&str_data);
    print(num_data);
}
