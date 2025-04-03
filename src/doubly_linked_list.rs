use std::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>, // this is a raw mutable pointer, only allowed in unsafe blocks
    prev: *mut Node<T>,
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(), // creates a raw mutable null pointer
            tail: ptr::null_mut(),
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn size(&self) -> usize {
        self.length
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
                value,
                next: self.head,
                prev: ptr::null_mut(),
        });

        let new_node_ptr = Box::into_raw(new_node);

        unsafe {
            if !self.head.is_null() {
                (*self.head).prev = new_node_ptr;
            } else {
                self.tail = new_node_ptr;
            }
            self.head = new_node_ptr;
        }

        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            let head_node = Box::from_raw(self.head); // create a Box from the raw pointer
            self.head = head_node.next;

            if !self.head.is_null() {
                (*self.head).prev = ptr::null_mut();
            } else {
                self.tail = ptr::null_mut();
            }

            self.length -= 1;

            Some(head_node.value)
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            Some(&(*self.head).value) // Some reference to the value of the value at the head
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: ptr::null_mut(),
            prev: self.tail,
        });

        let new_node_ptr = Box::into_raw(new_node);

        unsafe {
            if !self.tail.is_null() {
                (*self.tail).next = new_node_ptr;
            } else {
                self.head = new_node_ptr;
            }
            self.tail = new_node_ptr;
        }

        self.length += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            let tail_node = Box::from_raw(self.tail);
            self.tail = tail_node.prev;

            if !self.tail.is_null() {
                (*self.tail).next = ptr::null_mut();
            } else {
                self.head = ptr::null_mut();
            }

            self.length -= 1;

            Some(tail_node.value)
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            Some(&(*self.tail).value)
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> { // the drop trait is like a c++ destructor
    fn drop(&mut self) {
        while !self.is_empty() {
            self.pop_front();
        }
    }
}

#[cfg(test)]
mod doubly_linked_list_tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_push_front_increases_size() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        assert_eq!(list.size(), 1);
        assert!(!list.is_empty());
        
        list.push_front(2);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_push_back_increases_size() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        assert_eq!(list.size(), 1);
        assert!(!list.is_empty());
        
        list.push_back(2);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_pop_front_decreases_size() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        
        let value = list.pop_front();
        assert_eq!(value, Some(2));
        assert_eq!(list.size(), 1);
        
        let value = list.pop_front();
        assert_eq!(value, Some(1));
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_pop_back_decreases_size() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        
        let value = list.pop_back();
        assert_eq!(value, Some(2));
        assert_eq!(list.size(), 1);
        
        let value = list.pop_back();
        assert_eq!(value, Some(1));
        assert_eq!(list.size(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_peek_front() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.peek_front(), None);
        
        list.push_front(1);
        assert_eq!(list.peek_front(), Some(&1));
        
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));
        
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_peek_back() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.peek_back(), None);
        
        list.push_back(1);
        assert_eq!(list.peek_back(), Some(&1));
        
        list.push_back(2);
        assert_eq!(list.peek_back(), Some(&2));
        
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_mixed_operations() {
        let mut list = DoublyLinkedList::new();
        
        list.push_front(2);
        list.push_back(3);
        list.push_front(1);
        
        assert_eq!(list.size(), 3);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));
        
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(1));
        
        assert_eq!(list.size(), 1);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&2));
        
        assert_eq!(list.pop_front(), Some(2));
        assert!(list.is_empty());
        
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);
    }

    #[test]
    fn test_multiple_pops_when_empty() {
        let mut list = DoublyLinkedList::new();
        
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_back(), None);
        
        list.push_front(1);
        list.pop_front();
        
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_with_complex_types() {
        let mut list = DoublyLinkedList::new();
        
        list.push_back(String::from("Hello"));
        list.push_back(String::from("World"));
        
        assert_eq!(list.peek_front(), Some(&String::from("Hello")));
        assert_eq!(list.peek_back(), Some(&String::from("World")));
        
        assert_eq!(list.pop_front(), Some(String::from("Hello")));
        assert_eq!(list.pop_back(), Some(String::from("World")));
    }

    #[test]
    fn test_ordering_is_preserved() {
        let mut list = DoublyLinkedList::new();
        
        for i in 0..5 {
            list.push_back(i);
        }
        
        for i in 0..5 {
            assert_eq!(list.pop_front(), Some(i));
        }
        
        for i in 0..5 {
            list.push_front(i);
        }
        
        for i in 0..5 {
            assert_eq!(list.pop_front(), Some(4 - i));
        }
    }
}