#[cfg(test)]
mod fibonacci {
    use crate::fibonacci::*;

    #[test]
    fn test_fibonacci() {
        let x: [u8; 8] = [0, 1, 1, 2, 3, 5, 8, 13];

        for (i, val) in x.iter().enumerate() {
            assert_eq!(*val as u64, fibonacci(i as u8))
        }
    }

    #[test]
    fn test_fibonacci2() {
        let x: [u8; 8] = [0, 1, 1, 2, 3, 5, 8, 13];

        for (i, val) in x.iter().enumerate() {
            assert_eq!(*val as u64, fibonacci2(i as u8))
        }
    }

    #[test]
    fn test_big_fibonacci() {
        assert_eq!(23416728348467685, big_fibonacci(80));
        assert_eq!(659034621587630041982498215, big_fibonacci(130));
    }
}
