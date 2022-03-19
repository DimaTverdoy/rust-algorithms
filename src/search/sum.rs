//! Finding 2 array indexes that add up to the object you are looking for.
//!
//! # Examples
//! ```rust
//! use search::sum::search;
//!
//! let x: [i32; 6] = [6, 2, 4, 1, 8, 2];
//! assert_eq!(Some(1, 2), search(&x, &6);
//! assert_eq!(Some(3, 4), search(&mut x, &9));
//! ```

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