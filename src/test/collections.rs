#[cfg(test)]
mod linked_list {
    use crate::collections::linked_list;

    #[test]
    fn base() {
        let mut x = linked_list::LinkedList::new();

        x.push(4);
        x.push(32);

        println!("{}", x.get(0));
    }
}
