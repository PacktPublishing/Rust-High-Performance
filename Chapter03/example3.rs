fn change_third(slice: &mut [u32]) {
    if let Some(item) = slice.get_mut(2) {
        *item += 1
    }
}

fn print_third(slice: &[u32]) {
    if let Some(item) = slice.get(2) {
        println!("Third element: {}", item);
    }
}

fn main() {
    let mut my_vector = vec![73, 55, 33];
    print_third(&my_vector);
    change_third(&mut my_vector[..]);
    print_third(&my_vector);
}
