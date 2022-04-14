#[cfg(test)]
mod count {
    use crate::sort::count::sort;

    #[test]
    fn test() {
        let mut x = [4, 1, 65, 1, 0, 0, 8543];

        sort(&mut x);
        assert_eq!([0, 0, 1, 1, 4, 65, 8543], x)
    }
}

#[cfg(test)]
mod insert {
    use crate::sort::insert::sort;

    #[test]
    fn test() {
        let mut x = [4, 1, 65, 1, 0, 0, 8543];

        sort(&mut x);
        assert_eq!([0, 0, 1, 1, 4, 65, 8543], x)
    }
}