use crate::singly_linked_list::SinglyLinkedList;

pub struct Stack<T> {
    list: SinglyLinkedList<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            list: SinglyLinkedList::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.list.push_front(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn top(&self) -> Option<&T> {
        self.list.peek_front()
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
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push_increases_size() {
        let mut stack = Stack::new();
        stack.push(10);
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 1);
        
        stack.push(20);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_peek_operations() {
        let mut stack = Stack::new();
        assert_eq!(stack.top(), None);
        
        stack.push(10);
        assert_eq!(stack.top(), Some(&10));
        
        stack.push(20);
        assert_eq!(stack.top(), Some(&20));
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_pop_decreases_size() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);
        
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.size(), 2);
        
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.size(), 1);
        
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
        
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_lifo_order() {
        let mut stack = Stack::new();
        for i in 0..5 {
            stack.push(i);
        }
        
        for i in (0..5).rev() {
            assert_eq!(stack.pop(), Some(i));
        }
    }

    #[test]
    fn test_mixed_operations() {
        let mut stack = Stack::new();
        
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.top(), Some(&20));
        
        assert_eq!(stack.pop(), Some(20));
        stack.push(30);
        stack.push(40);
        
        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(10));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_with_complex_types() {
        let mut string_stack = Stack::new();
        string_stack.push(String::from("hello"));
        string_stack.push(String::from("world"));
        
        assert_eq!(string_stack.top(), Some(&String::from("world")));
        assert_eq!(string_stack.pop(), Some(String::from("world")));
        assert_eq!(string_stack.pop(), Some(String::from("hello")));
        
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        
        let mut point_stack = Stack::new();
        point_stack.push(Point { x: 1, y: 2 });
        point_stack.push(Point { x: 3, y: 4 });
        
        assert_eq!(point_stack.top(), Some(&Point { x: 3, y: 4 }));
        assert_eq!(point_stack.pop(), Some(Point { x: 3, y: 4 }));
        assert_eq!(point_stack.pop(), Some(Point { x: 1, y: 2 }));
    }
}