fn main() {
    let mut my_vector = vec![73, 55];
    println!(
        "length: {}, capacity: {}",
        my_vector.len(),
        my_vector.capacity()
    );

    my_vector.push(25);
    println!(
        "length: {}, capacity: {}",
        my_vector.len(),
        my_vector.capacity()
    );

    my_vector.push(33);
    my_vector.push(24);
    println!(
        "length: {}, capacity: {}",
        my_vector.len(),
        my_vector.capacity()
    );
}
