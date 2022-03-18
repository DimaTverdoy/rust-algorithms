#[cfg(test)]
mod binary {
    use crate::search::binary::search;

    #[test]
    fn base() {
        let x = [0, 1, 2, 6, 8, 10, 432, 524, 1000, 5892];

        for (i, value) in x.iter().enumerate() {
            assert_eq!(Some(i), search(&x, value))
        }
    }

    #[test]
    fn none() {
        let x = [0, 1, 2, 6, 8, 10, 432, 524, 1000, 5892];

        assert_eq!(None, search(&x, &5));
        assert_eq!(None, search(&x, &5300));
        assert_eq!(None, search(&x, &9421));
    }
}

#[cfg(test)]
mod linear {
    use crate::search::linear::search;

    #[test]
    fn base() {
        let x = [32, 14, 0, 794, 9913, 42, 1, 0];

        for (i, value) in x.iter().enumerate() {
            let index = search(&x, value).unwrap();
            assert_eq!(&x[index], value)
        }
    }

    #[test]
    fn none() {
        let x = [0, 1, 2, 6, 8, 10, 432, 524, 1000, 5892];

        assert_eq!(None, search(&x, &5));
        assert_eq!(None, search(&x, &5300));
        assert_eq!(None, search(&x, &9421));
    }
}