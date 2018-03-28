use std::cell::Cell;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Copy, Clone)]
struct House {
    bedrooms: u8,
}

impl Default for House {
    fn default() -> Self {
        House { bedrooms: 1 }
    }
}

fn main() {
    let my_house = House { bedrooms: 2 };
    let my_dream_house = House { bedrooms: 5 };

    let my_cell = Cell::new(my_house);
    println!("My house has {} bedrooms.", my_cell.get().bedrooms);

    my_cell.set(my_dream_house);
    println!("My new house has {} bedrooms.", my_cell.get().bedrooms);

    let my_new_old_house = my_cell.replace(my_house);
    println!(
        "My house has {} bedrooms, it was better with {}",
        my_cell.get().bedrooms,
        my_new_old_house.bedrooms
    );

    let my_new_cell = Cell::new(my_dream_house);

    my_cell.swap(&my_new_cell);
    println!(
        "Yay! my current house has {} bedrooms! (my new house {})",
        my_cell.get().bedrooms,
        my_new_cell.get().bedrooms
    );

    let my_final_house = my_cell.take();
    println!(
        "My final house has {} bedrooms, the shared one {}",
        my_final_house.bedrooms,
        my_cell.get().bedrooms
    );
}

struct Tree<T> {
  root: Node<T>,
}

struct Node<T> {
  parent: Option<Weak<Node<T>>>,
  left: Option<Rc<RefCell<Node<T>>>>,
  right: Option<Rc<RefCell<Node<T>>>>,
  value: T,
}
