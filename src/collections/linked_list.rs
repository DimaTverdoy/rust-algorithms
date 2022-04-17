use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    len: usize
}

#[derive(Debug)]
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    element: T
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self {next: None, element}
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

// Public methods
impl<T> LinkedList<T> where T: std::cmp::PartialEq {
    pub fn push(&mut self, element: T) {
        self.push_back_node(box Node::new(element))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<Box<T>> {
        unsafe {
            let node = self.get_node(index)?;
            Some(box Box::from_raw(node.as_ptr()).into_element())
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.remove_node(index)
    }

    pub fn index(&self, element: T) -> Option<usize> {
        self.index_node(element)
    }

    pub fn first(&self) -> Option<Box<T>> {
        self.get(0)
    }

    pub fn last(&self) -> Option<Box<T>> {
        self.get(self.len() - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

/// Private methods
impl<T> LinkedList<T> where T: std::cmp::PartialEq {
    pub fn new() -> Self {
        Self {head: None, len: 0}
    }
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = None;
            let node = Some(Box::leak(node).into());

            match self.head {
                Some(_) => (*self.get_node(self.len() - 1).unwrap().as_ptr()).next = node,
                None => self.head = node
            }

            self.len += 1
        }
    }

    fn get_node(&self, index: usize) -> Option<NonNull<Node<T>>> {
        assert!(index < self.len(), "Cannot get at an index outside of the list bounds");

        None
    }


    fn index_node(&self, element: T) -> Option<usize> {
        unsafe {
            let mut j = self.head?;
            for i in 0..self.len() {
                if (*j.as_ptr()).element == element {
                    return Some(i)
                }
                j = (*j.as_ptr()).next?
            }

            None
        }
    }


    fn remove_node(&mut self, index: usize) {
        unsafe {
            let remove_node = match self.get_node(index) {
                Some(node) => node,
                None => panic!("Element was not found")
            };

            let next_node = match (*remove_node.as_ptr()).next {
                Some(node) => node,
                None => {
                    if index == 0 {
                        self.head = None
                    } else {
                        let pre = self.get_node(index - 1).unwrap();
                        (*pre.as_ptr()).next = None;
                    }

                    self.len -= 1;
                    return;
                }
            };

            if index == 0 {
                self.head = Some(next_node);
            } else {
                let pre = self.get_node(index - 1).unwrap();
                (*pre.as_ptr()).next = Some(next_node)
            }

            self.len -= 1;
        };
    }
}