macro_rules! add_to_vec {
    ($( $x:expr; [ $( $y:expr ),* ]);* ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

fn main() {
    println!("{:?}", add_to_vec![10; [5, 5, 10]]);
}
