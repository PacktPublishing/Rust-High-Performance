// Should warn missing crate documentation.

#![allow(dead_code)] // Just to avoid some warnings
#![warn(anonymous_parameters)]
#![warn(box_pointers)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_results)]
#![warn(unused_qualifications)]
#![warn(unused_import_braces)]
#![warn(variant_size_differences)]

// Would stop compilation.
// #[deny(warnings)]

#[derive(Default, Debug, Clone, Copy)]
struct MyStruct {
    a: i32,
    b: i32,
}

// Should warn anonymous parameters.
trait MyTrait {
    fn check_this(String);
}

fn main() {
    use Test::*;
    // Should warn unused import braces.
    use Test2::{C};

    // Should warn box pointers.
    let mut int = Box::new(5);
    *int += 5;
    println!("int: {}", int);

    // Should warn trivial casts.
    let test = MyStruct::default();
    println!("{:?}", (test as MyStruct).a as i32);

    // Should warn unsafe code
    let test = vec![1, 2, 3];
    println!("{}", unsafe { test.get_unchecked(2) });

    // Should not warn.
    let test = vec![1, 2, 3];
    println!("{}", get_second(&test));

    // Should not warn.
    let _ = write_hello();

    // Should warn unused qualifications.
    println!("{:?}", Test::A);
    println!("{:?}", B);

    // Just to avoid another unused warning.
    println!("{:?}, {:?}, {:?}", A, B, C);
}

// Should warn missing debug and copy implementations and missing documentation.
pub struct SomeStruct {
    field1: u8,
}

#[allow(unsafe_code)]
fn get_second(slice: &[i32]) -> i32 {
    *unsafe { slice.get_unchecked(1) }
}

fn write_hello() -> usize {
    unimplemented!()
}

#[derive(Debug, Clone, Copy)]
enum Test {
    A,
    B,
}

#[derive(Debug, Clone, Copy)]
enum Test2 {
    C,
    D,
}

// Should warn variant size differences.
#[derive(Debug, Clone, Copy)]
enum Test3 {
    A(u8),
    B(u32),
}
