//! Count sort
//!
//! Counts the number of each digit and creates an array respecting equality.
//! The algorithm becomes impractical when the maximum number of the array is too large.
//!
//! # Examples
//! ```rust
//! use sort::count::sort;
//!
//! let mut x: [i32; 6] = [6, 2, 4, 1, 8, 2];
//! sort::count::sort(&mut x);
//! assert_eq!(x, [1, 2, 2, 4, 6, 8]);
//! ```

/// Count sort
/// # Big O
/// O(n * max(n))
pub fn sort(a: &mut [i32]) {
    if a.is_empty() {
        return;
    }

    let max_num = a.iter().max().unwrap() + 1;
    let mut b: Vec<i32> = vec![0; max_num as usize];


    for i in a.iter() {
        b[*i as usize] += 1;
    }

    println!("{:?}", b);
    let mut pointer = 0;
    for i in 0..max_num {
        for _ in 0..b[i as usize] {
            a[pointer] = i;
            pointer += 1;
        }
    }
}