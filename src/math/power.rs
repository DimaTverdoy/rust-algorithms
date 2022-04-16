pub fn fast_power(a: i32, n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let x = fast_power(a, n / 2);
    if n % 2 == 0 {
        return x * x
    } else {
        return a * (x * x)
    }
}