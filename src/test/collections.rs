#[cfg(test)]
mod linked_list {
    use crate::collections::linked_list::LinkedList;

    #[test]
    fn base() {
        let mut x = LinkedList::new();
        x.push(4);
        x.push(5);
        x.push(6);


        //x.remove(0);

        println!("Res: {:?}", x.get(0));
        println!("Res: {:?}", x.first());
    }
}
