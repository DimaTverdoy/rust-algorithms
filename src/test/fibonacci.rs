#[cfg(test)]
mod fibonacci {
    use crate::fibonacci::*;

    #[test]
    fn test_fibonacci() {
        let x = [0, 1, 1, 2, 3, 5, 8, 13];

        for (i, val) in x.iter().enumerate() {
            assert_eq!(*val, fibonacci(i as i32))
        }
    }

    #[test]
    fn test_fibonacci2() {
        let x = [0, 1, 1, 2, 3, 5, 8, 13];

        for (i, val) in x.iter().enumerate() {
            assert_eq!(*val, fibonacci2(i as i32))
        }
    }
}