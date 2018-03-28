use std::cell::Cell;

fn main() {
    let my_cell = Cell::new(0);
    println!("Initial cell value: {}", my_cell.get());

    my_cell.set(my_cell.get() + 1);
    println!("Final cell value: {}", my_cell.get());
}
