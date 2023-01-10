use std::ops::Drop;
use std::mem;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, val: T) {
        let node = Box::new(Node {
            val,
            next: None,
        });

        self.push_node(node);
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head.take();
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| node.val)
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node| {
            self.len -= 1;
            self.head = node.next.take();
            node
        })
    }

    pub fn replace(&mut self, other: &mut Self) {
        self.head = other.head.take();
        self.len = other.len;
        other.len = 0;
    }

    pub fn swap(&mut self, other: &mut Self) {
        mem::swap(&mut self.head, &mut other.head);
        mem::swap(&mut self.len, &mut other.len);
    }

    pub fn merge(&mut self, mut other: LinkedList<T>) {
        while let Some(val) = other.pop() {
            self.push(val);
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn msort_by<F>(&mut self, f: &F)
        where F: Fn(&T, &T) -> Ordering
    {
        if self.len > 1 {
            let mut left = LinkedList::new();
            let mut right = LinkedList::new();

            let mut next_node = &mut self.head;

            right.head = next_node.take();

            for _ in 0..(right.len / 2) {
                left.push_node(right.pop_node().unwrap());
            }

            left.msort_by(f);
            right.msort_by(f);

            while left.head.is_some() && right.head.is_some() {
                let list =
                    match f(&left.head.as_ref().unwrap().val, &right.head.as_ref().unwrap().val) {
                        Ordering::Less | Ordering::Equal => &mut left,
                        Ordering::Greater => &mut right,
                    };

                *next_node = list.pop_node();
                next_node = &mut next_node.as_mut().unwrap().next;
            }

            *next_node = if left.head.is_some() {
                left.head.take()
            } else {
                right.head.take()
            };
        }
    }
}

impl<T: std::cmp::PartialOrd + std::fmt::Debug> LinkedList<T> {
    pub fn msort(&mut self) {
        if self.len > 1 {
            let mut left = LinkedList::new();
            let mut right = LinkedList::new();

            let mut next_node = &mut self.head;

            right.head = next_node.take();

            for _ in 0..(right.len / 2) {
                left.push_node(right.pop_node().unwrap());
            }

            left.msort();
            right.msort();

            while left.head.is_some() && right.head.is_some() {
                let list =
                    if left.head.as_ref().unwrap().val <= right.head.as_ref().unwrap().val {
                        &mut left
                    } else {
                        &mut right
                    };

                *next_node = list.pop_node();
                next_node = &mut next_node.as_mut().unwrap().next;
            }

            *next_node = if left.head.is_some() {
                left.head.take()
            } else {
                right.head.take()
            };
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();

        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> std::iter::Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();

            &node.val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(657);
        list.push(10);
        list.push(8);
        list.push(45);
        list.push(1);

        let mut sorted_list = LinkedList::new();
        sorted_list.push(657);
        sorted_list.push(45);
        sorted_list.push(10);
        sorted_list.push(8);
        sorted_list.push(3);
        sorted_list.push(2);
        sorted_list.push(1);
        sorted_list.push(1);
    
        list.msort();

        while list.head.is_some() && sorted_list.head.is_some() {
            assert_eq!(list.pop().unwrap(), sorted_list.pop().unwrap());
        }
    }
}