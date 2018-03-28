extern crate crossbeam;

fn main() {
    let all_nums: Vec<_> = (0..10).into_iter().collect();

    crossbeam::scope(|scope| {
        for num in all_nums {
            scope.spawn(move || {
                println!("Next number is {}", num);
            });
        }
    });

    println!("Main thread continues :)");
}
