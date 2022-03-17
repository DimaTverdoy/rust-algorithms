//! Fibonacci numbers.
//!
//! Fibonacci numbers are a sequence of numbers in which
//! every 3rd number is equal to the sum of the two previous ones.
//! If the function argument is out of scope, then returns -1.
//!
//! Example fibonacci numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377.
//!
//! # Domain of a function
//! `Fib(n) >= 2^(n/2)` [check_domain](check_domain)
//!
//! # Examples
//! ```
//! use rust_algorithms::fibonacci::fibonacci;
//!
//! assert_eq!(fibonacci(5), fibonacci(4) + fibonacci(3));
//! ```

/// Checks if parameter n is in scope
pub fn check_domain(n: i32) -> bool {
    2_i32.pow((n / 2) as u32) < i32::MAX
}

/// Recursive finding
/// # Big O
/// O(n^2)
pub fn fibonacci(n: i32) -> i32 {
    if !check_domain(n) {
        return -1;
    }
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// Linear finding
/// # Big O
/// O(n)
pub fn fibonacci2(n: i32) -> i32 {
    if !check_domain(n) {
        return -1;
    }

    if n <= 1 {
        return n;
    }

    let (mut one, mut two, mut three) = (0, 1, 0);

    for _ in 0..n - 1 {
        three = one + two;
        one = two;
        two = three;
    };

    three
}
