use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T> fmt::Display for LinkedList<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T> Clone for LinkedList<T> where T: Clone {
    fn clone(&self) -> Self {
        let mut copied = Self::new();
        let mut head = &self.head;
        let mut values: Vec<T> = Vec::new();
        while let Some(node) = head.as_ref() {
            values.push(node.value.clone());
            head = &node.next;
        }
        values.reverse();
        for value in values.into_iter() {
            copied.push_front(value);
        }
        copied
    }
}

impl<T> PartialEq for LinkedList<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            false
        } else {
            let head_a = &self.head;
            let head_b = &other.head;
            for _ in 0..self.size {
                let value_a = &head_a.as_ref().unwrap().value;
                let value_b = &head_b.as_ref().unwrap().value;
                if !(value_a == value_b) {
                    return false;
                }
            }
            true
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<Self::Item>;
    fn into_iter(mut self) -> Self::IntoIter {
        Self::IntoIter {
            current: self.head.take(),
        }
    }
}

pub struct LinkedListIterator<T> {
    current: Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current.take() {
            self.current = node.next;
            Some(node.value)
        } else {
            None
        }
    }
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { current: &self.head }
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current {
            self.current = &node.next;
            Some(&node.value)
        } else {
            None
        }
    }
}