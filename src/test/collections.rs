#[cfg(test)]
mod linked_list {
    use crate::collections::linked_list;

    #[test]
    fn base() {
        let mut x = linked_list::LinkedList::new();

        x.push(4);
        x.push(32);

        assert_eq!(&4, x.get(0));
        assert_eq!(&32, x.get(1));
        assert_eq!(Some(0), x.index(4));
        assert_eq!(None, x.index(63));
        assert_eq!(2, x.len())
    }
}
