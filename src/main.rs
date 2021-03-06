#![feature(box_syntax)]
extern crate core;

mod collections;
mod fibonacci;
mod math;
mod search;
mod sort;
mod test;

fn main() {
    let mut x = [1, 43, 52, 5, 0];

    // Fibonacci
    println!("Fibonacci result is {}", fibonacci::fibonacci(1));
    println!("Fibonacci result is {}", fibonacci::fibonacci2(20));
    println!("Big fibonacci result is {}", fibonacci::big_fibonacci(80));

    // Math
    println!("Fast power result is {}", math::power::fast(2, 4));
    println!("Linear power result is {}", math::power::linear(4, 2));

    // Sort
    let mut x_insert = x;
    sort::insert::sort(&mut x_insert);
    println!("Insert sort result is {:?}", x_insert);

    let mut x_count = x;
    sort::count::sort(&mut x_count);
    println!("Count sort result is {:?}", x_count);

    let mut x_selection = x;
    sort::selection::sort(&mut x_selection);
    println!("Selection sort result is {:?}", x_selection);

    sort::selection::sort(&mut x);

    // Search
    println!(
        "Linear search result is {}",
        search::linear::search(&x, &5).unwrap()
    );
    println!(
        "Binary search result is {}",
        search::binary::search(&x, &5).unwrap()
    );
    println!(
        "Sum search result is {:?}",
        search::sum::search(&x, &57).unwrap()
    );
    println!(
        "Sum search result is {:?}",
        search::sum::binary_search(&x, &57).unwrap()
    );

    println!(
        "Substr search result is {:?}",
        search::substr::search("Hello", "lo").unwrap()
    );
}
