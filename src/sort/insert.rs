//! Insert sort
//!
//! Iterates over each element of the array and moves that
//! element to the right until the next element is less than
//!
//! # Examples
//! ```rust
//! use sort::insert::sort;
//!
//! let mut x: [i32; 6] = [6, 2, 4, 1, 8, 2];
//! sort(&mut x);
//! assert_eq!(x, [1, 2, 2, 4, 6, 8]);
//! ```

/// Insert sort
/// # Big O
/// O(n^2)
pub fn sort<T: Ord>(a: &mut [T]) {
    let mut j;
    for i in 1..a.len() {
        // No reason to start sorting from element 0
        j = i;
        while j > 0 && a[j] < a[j - 1] {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}
