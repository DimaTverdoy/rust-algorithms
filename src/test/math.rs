#[cfg(test)]
mod binary {
    use crate::math::power::fast_power;

    #[test]
    fn base() {
        let data = [(2, 3, 8), (21, 4, 194481), (0, 5, 0), (32, 1, 32)];

        for (a, n, result) in data {
            assert_eq!(result, fast_power(a, n));
        }
    }
}
