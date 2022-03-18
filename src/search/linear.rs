//! Linear search
//!
//! The simplest search algorithm that linearly peberates
//! an array element and if this element is equal to its value,
//! then returns its index in the array, otherwise none.
//!
//! # Examples
//!
//! ```rust
//! use search::linear::search;
//!
//! let mut x: [i32; 6] = [6, 2, 4, 1, 8, 2];
//!
//! assert_eq!(Some(1), search::linear::search(&mut x, 2));
//  assert_eq!(Some(4), search::linear::search(&mut x, 8));
//! ```

/// Linear search
/// # Big O
/// O(n)
pub fn search<T: Ord>(a: &mut [T], k: T) -> Option<usize> {
    for (i, val) in a.iter().enumerate() {
        if *val == k {
            return Some(i);
        }
    }

    None
}
