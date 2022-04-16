use std::fmt;

pub struct LinkedList<T> {
    head: Option<Node<T>>,
    length: usize,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Self { item, next: None }
    }

    fn push_last(&mut self, item: T) {
        match self.next {
            Some(ref mut next_node) => next_node.push_last(item),
            None => self.next = Some(Box::new(Node::new(item))),
        }
    }

    fn collect<'a>(&'a self, to: &mut Vec<&'a T>) {
        to.push(&self.item);
        if let Some(ref node) = self.next {
            node.collect(to);
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        match self.head {
            Some(ref mut node) => node.push_last(item),
            None => self.head = Some(Node::new(item)),
        }
        self.length += 1
    }

    pub fn get(&self, index: usize) -> &T {
        if self.length < index {
            panic!(
                "Index out of bounds: the len is {} but the index is {}",
                self.length, index
            )
        }

        let mut j = self.head.as_ref().unwrap();

        for i in 0..index {
            match j.next {
                Some(ref node) => j = node,
                None => panic!("Error get by index {}", index),
            }
        }

        &j.item
    }
}
