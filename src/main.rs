mod fibonacci;
mod search;
mod sort;

fn main() {
    let mut x: [i32; 6] = [6, 2, 4, 1, 8, 2];

    println!("Result fibonacci is {}", fibonacci::fibonacci(2));
    println!("Result fibonacci2 is {}", fibonacci::fibonacci2(6));

    sort::count::sort(&mut x);
    println!("Result count sort is {:?}", x);

    println!(
        "Result search linear is {}",
        search::linear::search(&mut x, 4).unwrap()
    );

    println!("Result binary searhc is {}",
        search::binary::search(&mut x, 4).unwrap()
    );
}
