// Note: requires nightly Rust
#![feature(conservative_impl_trait)]
#![feature(const_fn)]
#![feature(i128_type)]

const FIRST_CONST: MyData = MyData::new(23, 275);
const SECOND_CONST: MyData = MyData::new(336, 7);

#[derive(Debug)]
struct MyData {
    field1: u32,
    field2: f32,
}

impl MyData {
    pub const fn new(a: u32, b: u32) -> MyData {
        MyData {
            field1: a / b,
            field2: b as f32 / a as f32,
        }
    }
}

fn main() {
    println!("FIRST_CONST: {:?}", FIRST_CONST);
    println!("SECOND_CONST: {:?}", SECOND_CONST);

    let third = MyData::new(78, 22);
    println!("third: {:?}", third);

    assert_eq!(1u128 + 1u128, 2u128);
    assert_eq!(u128::min_value(), 0);
    assert_eq!(u128::max_value(), 340282366920938463463374607431768211455);

    assert_eq!(1i128 - 2i128, -1i128);
    assert_eq!(i128::min_value(), -170141183460469231731687303715884105728);
    assert_eq!(i128::max_value(), 170141183460469231731687303715884105727);
}

fn iterate_something() -> impl Iterator<Item = u32> {
    (0..3).into_iter()
}
