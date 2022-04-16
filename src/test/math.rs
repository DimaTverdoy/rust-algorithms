#[cfg(test)]
mod power {
    use crate::math::power::fast;
    use crate::math::power::linear;

    #[test]
    fn fast_test() {
        let data = [(2, 3, 8), (21, 4, 194481), (0, 5, 0), (32, 1, 32)];

        for (a, n, result) in data {
            assert_eq!(result, fast(a, n));
        }
    }

    #[test]
    fn linear_test() {
        let data = [(2, 3, 8), (21, 4, 194481), (0, 5, 0), (32, 1, 32)];

        for (a, n, result) in data {
            assert_eq!(result, linear(a, n));
        }
    }
}
