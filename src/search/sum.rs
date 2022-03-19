//! Finding 2 array indexes that add up to the object you are looking for.
//!
//! # Examples
//! ```rust
//! use search::sum::*;
//!
//! let x: [i32; 6] = [6, 2, 4, 1, 8, 2];
//! assert_eq!(Some((1, 2)), search(&x, &6));
//! assert_eq!(Some((3, 4)), binary_search(&mut x, &9));
//! ```

use std::cmp::Ordering;
use std::ops::Add;

/// Brute force search
/// # Big O
/// O(n^2)
pub fn search<T: Ord + Add<Output = T> + Copy>(a: &[T], s: &T) -> Option<(usize, usize)> {
    for (i, val) in a.iter().enumerate() {
        for (j, j_val) in a[i + 1..].iter().enumerate() {
            if &(*val + *j_val) == s {
                return Some((i, i + j + 1));
            }
        }
    }

    None
}

/// Binary search. The array must be sorted
/// # Big O
/// O(n)
pub fn binary_search<T: Ord + Add<Output = T> + Copy>(a: &[T], s: &T) -> Option<(usize, usize)> {
    let (mut left, mut right) = (0, a.len() - 1);
    while left < right {
        match (a[left] + a[right]).cmp(s) {
            Ordering::Less => left += 1,
            Ordering::Equal => return Some((left, right)),
            Ordering::Greater => right -= 1
        }
    }

    None
}
