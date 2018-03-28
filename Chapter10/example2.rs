use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    let hm = HashMap::new();
    let my_cell = RefCell::new(hm);
    println!("Initial cell value: {:?}", my_cell.borrow());

    my_cell.borrow_mut().insert("test_key", "test_value");
    println!("Final cell value: {:?}", my_cell.borrow());
}
