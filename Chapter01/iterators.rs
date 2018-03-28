fn main() {
    // ------------
    let arr = ['a', 'b', 'c', 'd', 'e', 'f'];

    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    // ------------ PANICS
    // let arr = ['a', 'b', 'c', 'd', 'e', 'f'];
    //
    // for i in 0..arr.len() + 1 {
    //     println!("{}", arr[i]);
    // }

    // ------------
    struct Request {
        content_length: usize,
        data: Vec<u8>,
    }

    fn print_request(req: Request) {
        for i in 0..req.content_length {
            println!("{}", req.data[i]);
        }
    }

    // ------------
    let arr = ['a', 'b', 'c', 'd', 'e', 'f'];

    for c in &arr {
        println!("{}", c);
    }

    // ------------
    let array_of_arrays = &[[3, 4, 5], [5, 8, 9], [1, 2, 3]];

    for arr in array_of_arrays {
        let last_index = arr.len() - 1;
        println!("{}", arr[last_index]);
    }

    // ------------
    for arr in array_of_arrays {
        if let Some(elt) = arr.iter().rev().next() {
            println!("{}", elt);
        }
    }

    // ------------
    for arr in array_of_arrays {
        if let Some(elt) = arr.get(2) {
            println!("{}", elt);
        }
    }

    // ------------
    for arr in array_of_arrays {
        println!("{}", unsafe { arr.get_unchecked(2) });
    }

    // ------------
    let arr = [10u8, 14, 5, 76, 84];
    let mut iter = arr.iter();

    while let Some(elm) = iter.next() {
        println!("{}", elm);
    }

    // ------------
    let arr = [10u8, 14, 5, 76, 84];

    for elm in &arr {
        println!("{}", elm);
    }

    // ------------
    let arr = [
        10u8, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    let collection: Vec<_> = arr.iter().cloned().skip(2).take(8).collect();

    for elm in collection {
        println!("{}", elm);
    }

    // ------------
    let arr = [
        10u8, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    let collection: Vec<_> = arr.iter()
        .cloned()
        .skip_while(|&elm| elm < 25)
        .take_while(|&elm| elm <= 100)
        .collect();

    for elm in collection {
        println!("{}", elm);
    }

    // ------------
    let arr = [
        10u8, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    let collection: Vec<_> = arr.iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0)
        .map(|(_, elm)| elm)
        .collect();

    for elm in collection {
        println!("{}", elm);
    }

    // ------------
    let arr = [
        10u8, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    let collection: Vec<_> = arr.iter()
        .enumerate()
        .filter_map(|(i, elm)| if i % 2 != 0 { Some(elm) } else { None })
        .collect();

    for elm in collection {
        println!("{}", elm);
    }

    // ------------
    let arr = [
        10u32, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    let sum = arr.iter().fold(0u32, |acc, elm| acc + elm);
    println!("{}", sum);

    // ------------
    fn calculate_var(t: f64, var: &[(f64, f64, f64)]) -> f64 {
        var.iter()
            .fold(0_f64, |term, &(a, b, c)| term + a * (b + c * t).cos())
    }

    // ------------
    fn calculate_var2(t: f64, var: &[(f64, f64, f64)]) -> f64 {
        let mut term = 0_f64;
        for &(a, b, c) in var {
            term += a * (b + c * t).cos();
        }
        term
    }

    // ------------
    let arr = [
        10u32, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    let sum: u32 = arr.iter().sum();
    println!("{}", sum);

    // ------------
    let arr = [
        10u32, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];

    let prod = arr.iter().fold(0u32, |acc, elm| acc * elm);
    println!("{}", prod);

    // ------------
    let arr1 = [
        10u32, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72
    ];
    let arr2 = [
        25u32, 12, 73, 2, 98, 122, 213, 22, 39, 300, 144, 163, 127, 3, 56
    ];

    let collection: Vec<_> = arr1.iter()
        .zip(arr2.iter())
        .map(|(elm1, elm2)| elm1 + elm2)
        .collect();
    println!("{:?}", collection);

    // ------------
    let my_int = 76_u32;
    println!("{}", { my_int });
}
