mod fibonacci;
mod sort;
use sort::count::sort;

fn main() {
    let mut x: [i32; 6] = [6, 2, 4, 1, 8, 2];

    println!("Result fibonacci is {}", fibonacci::fibonacci2(6));

    sort(&mut x);
    println!("Result count sort is {:?}", x);
    assert_eq!(x, [1, 2, 2, 4, 6, 8]);
}
