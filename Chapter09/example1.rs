macro_rules! add {
    {one to $input:expr} => ($input + 1);
    {two to $input:expr} => ($input + 2);
}

fn main() {
    println!("Add one: {}", add!(one to 25/5));
    println!("Add two: {}", add!(two to 25/5));
}
