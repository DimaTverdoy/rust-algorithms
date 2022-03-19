//! Fibonacci numbers
//!
//! Fibonacci numbers are a sequence of numbers in which
//! every 3rd number is equal to the sum of the two previous ones.
//! If the function argument is out of scope, then returns -1.
//!
//! Example fibonacci numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377.
//!
//! # Examples
//! ```rust
//! use rust_algorithms::fibonacci::fibonacci;
//!
//! assert_eq!(fibonacci(5), fibonacci(4) + fibonacci(3));
//! ```


/// Recursive finding
/// # Big O
/// O(n^2)
pub fn fibonacci(n: u8) -> u64 {
    if n <= 1 {
        n as u64
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// Linear finding
/// # Big O
/// O(n)
pub fn fibonacci2(n: u8) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let (mut one, mut two, mut three) = (0, 1, 0);

    for _ in 1..n {
        three = one + two;
        one = two;
        two = three;
    }

    three
}

/// Linear finding
/// # Big O
/// O(n)
pub fn big_fibonacci(n: u8) -> u128 {
    if n <= 1 {
        return n as u128;
    }

    let (mut one, mut two, mut three) = (0, 1, 0);

    for _ in 1..n {
        three = one + two;
        one = two;
        two = three;
    }

    three
}
