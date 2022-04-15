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

        for i in x {
            let index = search(&x, &i).unwrap();
            assert_eq!(x[index], i)
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
mod substr {
    use crate::search::substr::search;

    #[test]
    fn base() {
        let data = [
            ("Hello", "llo", 2),
            ("Test string", "rin", 7),
            ("From", "F", 0),
        ];

        for (t, p, result) in data {
            assert_eq!(search(t, p), Some(result))
        }
    }

    #[test]
    fn none() {
        let data = [("Hello", "Hlo"), ("Test string", "qew"), ("From", "f")];

        for (t, p) in data {
            assert_eq!(search(t, p), None)
        }
    }
}
