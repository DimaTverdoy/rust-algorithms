//! Selection sort
//!
//! The smallest unsorted element is determined and placed at the end of the sorted list.
//!
//! The algorithm is similar to [insertion sort](super::insert),
//! but more efficient since the call to the [swap](core::mem::swap) function will be n times
//!
//! # Examples
//! ```rust
//! use sort::selection::sort;
//!
//! let mut x: [i32; 6] = [6, 2, 4, 1, 8, 2];
//! sort(&mut x);
//! assert_eq!(x, [1, 2, 2, 4, 6, 8]);
//! ```

/// Selection sort
/// # Big O
/// O(n^2)
pub fn sort<T: std::cmp::PartialOrd>(a: &mut [T]) {
    let mut min;

    for i in 0..a.len() {
        min = i;
        for j in (i + 1)..a.len() {
            if a[min] > a[j] {
                min = j;
            }
        }

        a.swap(i, min);
    }
}
