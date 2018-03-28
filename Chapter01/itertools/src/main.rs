extern crate itertools;
use itertools::Itertools;

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
}

fn example_1() {
    let arr = [
        10u32, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    for tuple in arr.iter().batching(|it| match it.next() {
        None => None,
        Some(x) => match it.next() {
            None => None,
            Some(z) => match it.next() {
                None => None,
                Some(y) => Some((x, y, z)),
            },
        },
    }) {
        println!("{:?}", tuple);
    }
}

fn example_2() {
    let arr = [
        10u32, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    for tuple in arr.iter().tuples::<(_, _, _)>() {
        println!("{:?}", tuple);
    }
}

fn example_3() {
    let arr = [
        10u32, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    for (a, b, c) in arr.iter().tuples() {
        println!("({}, {}, {})", a, b, c);
    }
}

fn example_4() {
    let arr1 = [10u32, 14, 5];
    let arr2 = [192u32, 73, 44];

    for row in arr1.iter().cartesian_product(arr2.iter()) {
        print!("{:?}, ", row);
    }
}
