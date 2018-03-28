fn main() {
    let mut fruits = vec!["Orange", "Pome", "Peach", "Banana", "Kiwi", "Pear"];
    fruits.sort_by(|a, b| a.chars().next().cmp(&b.chars().next()));

    println!("{:?}", fruits);

    // ------------
    let mut fruits = vec!["Orange", "Pome", "Peach", "Banana", "Kiwi", "Pear"];
    fruits.sort_unstable_by(|a, b| a.chars().next().cmp(&b.chars().next()));

    println!("{:?}", fruits);
}
