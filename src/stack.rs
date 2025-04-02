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
    use super::Stack;

    #[test]
    fn test_stack_new_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.top(), None);
    }

    #[test]
    fn test_stack_push() {
        let mut stack = Stack::new();
        stack.push(10);
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.top(), Some(&10));

        stack.push(20);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.top(), Some(&20));
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.top(), Some(&2));
        assert_eq!(stack.size(), 2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_mixed_operations() {
        let mut stack = Stack::new();
        stack.push(42);
        assert_eq!(stack.top(), Some(&42));
        stack.push(100);
        assert_eq!(stack.pop(), Some(100));
        assert_eq!(stack.pop(), Some(42));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_with_strings() {
        let mut stack = Stack::new();
        stack.push("hello");
        stack.push("world");
        assert_eq!(stack.top(), Some(&"world"));
        assert_eq!(stack.pop(), Some("world"));
        assert_eq!(stack.pop(), Some("hello"));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_with_custom_type() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut stack = Stack::new();
        stack.push(Point { x: 1, y: 2 });
        stack.push(Point { x: 3, y: 4 });

        assert_eq!(stack.top(), Some(&Point { x: 3, y: 4 }));
        assert_eq!(stack.pop(), Some(Point { x: 3, y: 4 }));
        assert_eq!(stack.pop(), Some(Point { x: 1, y: 2 }));
    }
}