mod fibonacci;
mod search;
mod sort;
mod test;

fn main() {
    let mut x = [1, 43, 52, 5];

    println!("Fibonacci result is {}", fibonacci::fibonacci(4));
    println!("Fibonacci result is {}", fibonacci::fibonacci2(4));

    sort::count::sort(&mut x);
    println!("Count sort result is {:?}", x);

    println!("Linear seach result is {}", search::linear::search(&x, &5).unwrap());
    println!("Binary seach result is {}", search::binary::search(&x, &5).unwrap());
}
