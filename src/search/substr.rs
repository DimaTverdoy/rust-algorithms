//! Substr search
//!
//! Iterates over the pattern in each row indexes,
//! if they are equal then returns the index at the beginning of the searched pattern in the row,
//! otherwise if the pattern was not found returns [None](core::option::Option::None)
//!
//! # Example work algorithm
//! Finding the substring abba in the text aababba
//!
//! ```
//! a b
//!   a b b
//!     a
//!       a b b a
//! -------------
//! a a b a b b a
//! ```
//! # Examples
//!
//! ```rust
//! use search::substr::search;
//!
//! assert_eq!(Some(2), search("Hello", "llo"));
//! assert_eq!(None, search("From", "asd"));
//! ```

/// Substr search
/// # Big O
/// O(n^2)
pub fn search(t: &str, p: &str) -> Option<usize> {
    let mut j;

    for i in 0..(t.len() - p.len() + 1) {
        j = 0;
        while (j < p.len()) && (t.chars().nth(i + j) == p.chars().nth(j)) {
            j += 1
        }

        if j == p.len() {
            return Some(i);
        }
    }

    None
}
