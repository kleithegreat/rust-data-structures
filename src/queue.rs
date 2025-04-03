use crate::doubly_linked_list::DoublyLinkedList;

pub struct Queue<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            list: DoublyLinkedList::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.list.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.list.peek_front()
    }

    pub fn back(&self) -> Option<&T> {
        self.list.peek_back()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn size(&self) -> usize {
        self.list.size()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_new_is_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_push_increases_size() {
        let mut queue = Queue::new();
        queue.push(10);
        assert!(!queue.is_empty());
        assert_eq!(queue.size(), 1);
        
        queue.push(20);
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn test_peek_operations() {
        let mut queue = Queue::new();
        assert_eq!(queue.front(), None);
        assert_eq!(queue.back(), None);
        
        queue.push(10);
        assert_eq!(queue.front(), Some(&10));
        assert_eq!(queue.back(), Some(&10));
        
        queue.push(20);
        assert_eq!(queue.front(), Some(&10));
        assert_eq!(queue.back(), Some(&20));
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn test_pop_decreases_size() {
        let mut queue = Queue::new();
        queue.push(10);
        queue.push(20);
        queue.push(30);
        
        assert_eq!(queue.pop(), Some(10));
        assert_eq!(queue.size(), 2);
        
        assert_eq!(queue.pop(), Some(20));
        assert_eq!(queue.size(), 1);
        
        assert_eq!(queue.pop(), Some(30));
        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());
        
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = Queue::new();
        for i in 0..5 {
            queue.push(i);
        }
        
        for i in 0..5 {
            assert_eq!(queue.pop(), Some(i));
        }
    }

    #[test]
    fn test_mixed_operations() {
        let mut queue = Queue::new();
        
        queue.push(10);
        queue.push(20);
        assert_eq!(queue.front(), Some(&10));
        assert_eq!(queue.back(), Some(&20));
        
        assert_eq!(queue.pop(), Some(10));
        queue.push(30);
        queue.push(40);
        
        assert_eq!(queue.pop(), Some(20));
        assert_eq!(queue.pop(), Some(30));
        assert_eq!(queue.pop(), Some(40));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_with_complex_types() {
        let mut string_queue = Queue::new();
        string_queue.push(String::from("hello"));
        string_queue.push(String::from("world"));
        
        assert_eq!(string_queue.front(), Some(&String::from("hello")));
        assert_eq!(string_queue.back(), Some(&String::from("world")));
        assert_eq!(string_queue.pop(), Some(String::from("hello")));
        assert_eq!(string_queue.pop(), Some(String::from("world")));
        
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        
        let mut point_queue = Queue::new();
        point_queue.push(Point { x: 1, y: 2 });
        point_queue.push(Point { x: 3, y: 4 });
        
        assert_eq!(point_queue.front(), Some(&Point { x: 1, y: 2 }));
        assert_eq!(point_queue.back(), Some(&Point { x: 3, y: 4 }));
        assert_eq!(point_queue.pop(), Some(Point { x: 1, y: 2 }));
        assert_eq!(point_queue.pop(), Some(Point { x: 3, y: 4 }));
    }
}