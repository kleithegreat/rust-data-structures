use crate::doubly_linked_list::DoublyLinkedList;

pub struct Deque<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            list: DoublyLinkedList::new(),
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.list.push_front(value);
    }

    pub fn push_back(&mut self, value: T) {
        self.list.push_back(value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn size(&self) -> usize {
        self.list.size()
    }

    pub fn front(&self) -> Option<&T> {
        self.list.peek_front()
    }

    pub fn back(&self) -> Option<&T> {
        self.list.peek_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let deque: Deque<i32> = Deque::new();
        assert!(deque.is_empty());
        assert_eq!(deque.size(), 0);
    }

    #[test]
    fn test_push_increases_size() {
        let mut deque = Deque::new();
        
        deque.push_front(10);
        assert!(!deque.is_empty());
        assert_eq!(deque.size(), 1);
        
        deque.push_front(20);
        assert_eq!(deque.size(), 2);
        
        let mut deque = Deque::new();
        
        deque.push_back(10);
        assert!(!deque.is_empty());
        assert_eq!(deque.size(), 1);
        
        deque.push_back(20);
        assert_eq!(deque.size(), 2);
    }

    #[test]
    fn test_peek_operations() {
        let mut deque = Deque::new();
        assert_eq!(deque.front(), None);
        assert_eq!(deque.back(), None);
        
        deque.push_front(10);
        assert_eq!(deque.front(), Some(&10));
        assert_eq!(deque.back(), Some(&10));
        
        deque = Deque::new();
        deque.push_front(10);
        deque.push_front(20);
        assert_eq!(deque.front(), Some(&20));
        assert_eq!(deque.back(), Some(&10));
        
        deque = Deque::new();
        deque.push_back(10);
        deque.push_back(20);
        assert_eq!(deque.front(), Some(&10));
        assert_eq!(deque.back(), Some(&20));
        
        deque = Deque::new();
        deque.push_back(20);
        deque.push_front(10);
        assert_eq!(deque.front(), Some(&10));
        assert_eq!(deque.back(), Some(&20));
    }

    #[test]
    fn test_pop_decreases_size() {
        let mut deque = Deque::new();
        deque.push_back(10);
        deque.push_back(20);
        deque.push_back(30);
        
        assert_eq!(deque.pop_front(), Some(10));
        assert_eq!(deque.size(), 2);
        
        assert_eq!(deque.pop_front(), Some(20));
        assert_eq!(deque.size(), 1);
        
        assert_eq!(deque.pop_front(), Some(30));
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
        
        assert_eq!(deque.pop_front(), None);
        
        deque = Deque::new();
        deque.push_back(10);
        deque.push_back(20);
        deque.push_back(30);
        
        assert_eq!(deque.pop_back(), Some(30));
        assert_eq!(deque.size(), 2);
        
        assert_eq!(deque.pop_back(), Some(20));
        assert_eq!(deque.size(), 1);
        
        assert_eq!(deque.pop_back(), Some(10));
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
        
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn test_deque_ordering() {
        let mut deque = Deque::new();
        for i in 0..5 {
            deque.push_back(i);
        }
        
        for i in 0..5 {
            assert_eq!(deque.pop_front(), Some(i));
        }
        
        deque = Deque::new();
        for i in 0..5 {
            deque.push_front(i);
        }
        
        for i in 0..5 {
            assert_eq!(deque.pop_front(), Some(4 - i));
        }
        
        deque = Deque::new();
        for i in 0..5 {
            deque.push_back(i);
        }
        
        for i in 0..5 {
            assert_eq!(deque.pop_back(), Some(4 - i));
        }
    }

    #[test]
    fn test_mixed_operations() {
        let mut deque = Deque::new();
        
        deque.push_front(10);
        deque.push_back(20);
        assert_eq!(deque.front(), Some(&10));
        assert_eq!(deque.back(), Some(&20));
        
        deque.push_front(5);
        deque.push_back(25);
        assert_eq!(deque.size(), 4);
        
        assert_eq!(deque.pop_front(), Some(5));
        assert_eq!(deque.pop_back(), Some(25));
        
        assert_eq!(deque.front(), Some(&10));
        assert_eq!(deque.back(), Some(&20));
        
        deque.push_front(15);
        assert_eq!(deque.pop_back(), Some(20));
        assert_eq!(deque.pop_front(), Some(15));
        assert_eq!(deque.pop_front(), Some(10));
        assert!(deque.is_empty());
    }

    #[test]
    fn test_alternate_operations() {
        let mut deque = Deque::new();
        
        deque.push_front(1);
        deque.push_back(2);
        deque.push_front(3);
        deque.push_back(4);
        
        assert_eq!(deque.size(), 4);
        assert_eq!(deque.front(), Some(&3));
        assert_eq!(deque.back(), Some(&4));
        
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_back(), Some(4));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        
        assert!(deque.is_empty());
    }

    #[test]
    fn test_with_complex_types() {
        let mut string_deque = Deque::new();
        string_deque.push_front(String::from("world"));
        string_deque.push_front(String::from("hello"));
        
        assert_eq!(string_deque.front(), Some(&String::from("hello")));
        assert_eq!(string_deque.back(), Some(&String::from("world")));
        assert_eq!(string_deque.pop_front(), Some(String::from("hello")));
        assert_eq!(string_deque.pop_back(), Some(String::from("world")));
        
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        
        let mut point_deque = Deque::new();
        point_deque.push_back(Point { x: 1, y: 2 });
        point_deque.push_back(Point { x: 3, y: 4 });
        
        assert_eq!(point_deque.front(), Some(&Point { x: 1, y: 2 }));
        assert_eq!(point_deque.back(), Some(&Point { x: 3, y: 4 }));
        assert_eq!(point_deque.pop_front(), Some(Point { x: 1, y: 2 }));
        assert_eq!(point_deque.pop_back(), Some(Point { x: 3, y: 4 }));
    }
}