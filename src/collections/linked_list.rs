use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Option<Box<Node<T>>>>,
}

struct Node<T> {
    previous: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    element: T,
}

struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

struct IntoIter<T> {
    list: LinkedList<T>
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self { previous: None, next: None, element }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}


impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

// Public methods
impl<T> LinkedList<T> {
    pub fn push_back(&mut self, element: T) {
        self.push_back_node(box Node::new(element))
    }

    pub fn push_front(&mut self, element: T) {
        self.push_front_node(box Node::new(element))
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData
        }
    }
}

// Private methods
impl<T> LinkedList<T> {
    #[inline]
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = None;
            node.previous = self.tail;

            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                Some(tail_node) => (*tail_node.as_ptr()).next = node
            }

            self.tail = node;
            self.len += 1;
        }
    }

    #[inline]
    pub fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.previous = None;
            node.next = self.head;

            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                Some(head_node) => (*head_node.as_ptr()).previous = node
            }

            self.head = node;
            self.len += 1;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
          None
        } else {
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.head = node.next;
                self.len -= 1;
                &node.element
            })
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}