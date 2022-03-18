//! Binary search
//!
//! Quick search for the index of the array element,
//! but a mandatory condition: the array in which the
//! search will take place must be sorted.
//!
//! The algorithm drops half of the possible element index
//! each iteration until it finds it.
//!
//! There are 3 variables:
//! * `left` - The left side of the array border where the searched element can be.
//!                 Default is length array - 1.
//!  * `right` - The left side of the array border where the searched element can be.
//!                 Default is 0.
//!  * `middle` - The middle of the array which is calculated by the formula: (left + right) / 2
//!
//! Each iteration, get an element from the array at index
//! middle and compare it with the element we are looking for.
//! If they are equal then return the index (middle). If less than
//! this means that the element should be searched on the right side
//! of the array, then we increase the border of the left array by middle + 1.
//! If it is more than this means that the element should be searched on
//! the left side of the array, then we decrease the right border of the array by middle - 1

use std::cmp::Ordering;

/// Binary search
/// # Big O
/// O(log(n))
pub fn search<T: Ord>(a: &mut [T], k: T) -> Option<usize> {
    let (mut left, mut right) = (0, a.len() - 1);
    let mut mid;
    while left <= right {
        mid = (left + right) / 2;
        match a[mid].cmp(&k) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => right = mid - 1
        }
    }

    None
}