use std::{fmt, mem};
use std::borrow::BorrowMut;

pub struct LinkedList<T> {
    head: Option<Node<T>>,
    len: usize,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq> Node<T> {
    fn new(item: T) -> Node<T> {
        Self { item, next: None }
    }

    fn push_last(&mut self, item: T) {
        match self.next {
            Some(ref mut next_node) => next_node.push_last(item),
            None => self.next = Some(Box::new(Node::new(item))),
        }
    }

    fn search(&self, item: T, index: &mut usize) -> Option<&Self> {
        *index += 1;

        if self.item == item {
            Some(self)
        } else {
            match self.next {
                Some(ref node) => node.search(item, index),
                None => None
            }
        }
    }

    fn get(&self, index: usize, count: &mut usize) -> &Node<T> {
        if *count == index {
            return self
        }
        *count += 1;

        return self.next.as_ref().unwrap().get(index, count);
    }

    fn collect<'a>(&'a self, to: &mut Vec<&'a T>) {
        to.push(&self.item);
        if let Some(ref node) = self.next {
            node.collect(to);
        }
    }
}

impl<T: std::cmp::PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        match self.head {
            Some(ref mut node) => node.push_last(item),
            None => self.head = Some(Node::new(item)),
        }
        self.len += 1
    }

    pub fn get(&self, index: usize) -> &T {
        if self.len - 1 < index {
            panic!(
                "Index out of bounds: the len is {} but the index is {}",
                self.len, index
            )
        }

        &self.head.as_ref().unwrap().get(index, 0.borrow_mut()).item
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn index(&self, item: T) -> Option<usize> {
        let mut index = 0;
        self.head.as_ref()?.search(item, &mut index)?;
        Some(index - 1)

    }
}
