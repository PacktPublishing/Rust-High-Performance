fn main() {
    let opt = Some(123);
    let non_opt = opt.unwrap_or(some_complex_function());

    let opt = Some(123);
    let non_opt = opt.unwrap_or_else(some_complex_function);

    let opt = Some(123);
    let non_opt = opt.unwrap_or_else(|| even_more_complex_function(get_argument()));
}

fn some_complex_function() -> i32 {
    unimplemented!()
}

fn even_more_complex_function(a: i32) -> i32 {
    a
}

fn get_argument() -> i32 {
    unimplemented!()
}
