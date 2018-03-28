union Plant {
    g: Geranium,
    c: Carnation,
}

#[derive(Copy, Clone)]
struct Geranium {
    height: u32,
}

#[derive(Copy, Clone)]
struct Carnation {
    flowers: u8,
}

fn main() {
    let mut my_plant = Plant {
        c: Carnation { flowers: 15 },
    };
    my_plant.g = Geranium { height: 300 };
    let height = unsafe { my_plant.g }.height;

    println!("Height: {}", height);
}
