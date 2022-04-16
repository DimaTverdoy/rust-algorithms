//! Power
//!
//! Linear exponentiation is very simple,
//! but there is a faster algorithm based on this formula:
//! `n = n/2 + n/2`. if n is even then `a^n = (a^(n/2))^2`.
//! If n is odd, then `a^n = a * (a^(n/2))^2`
//!


/// Fast power
/// # Big O
/// O(log(n))
pub fn fast(a: i32, n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let x = fast(a, n / 2);
    if n % 2 == 0 {
        x * x
    } else {
        a * (x * x)
    }
}

/// Linear power
/// # Big O
/// O(n)
pub fn linear(a: i32, n: i32) -> i32 {
    let mut result = a;

    for _ in 0..n - 1 {
        result *= a;
    }

    result
}