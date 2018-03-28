macro_rules! my_vec {
    ($($x: expr),*) => {{
        let mut vector = Vec::new();
        $(vector.push($x);)*
        vector
    }}
}

fn main() {
    let my_vector = my_vec![4, 8, 15, 16, 23, 42];
    println!("Vector test: {:?}", my_vector);
}
