// akin to
// data Node a = nil | Node a (Node a)
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
// Option<T> like Maybe a in Haskell
// Box<T> is pointer that *uniquely owns* some heap-allocated data

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize, // usize: pointer sized unsigned integer
}

impl<T> SinglyLinkedList<T> { // in Rust, the `impl` keyword is where we define methods for a struct
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            length: 0,
        } // note the lack of a return statement or semicolon here
          // this is the idiomatic way to return something; it is just the last expression in the function
    }

    pub fn is_empty(&self) -> bool { // &self is an *immutable* reference to the linked list
        self.head.is_none() // is_none() is a method on Option<T> that returns true if the Option is None
    }

    pub fn size(&self) -> usize {
        self.length
    }

    pub fn push_front(&mut self, value: T) { // &mut self is a *mutable* reference to the linked list
                                             // **important**: in Rust, you can either have as many immutable references as you want,
                                             // OR only one mutable reference, but never both at the same time
        let new_head = Some(Box::new(Node { // `Some` is like `Just` in Haskell
            value, // this desugars into value: value; the corresponding names being the same lets us do this
            next: self.head.take(), // take() is an interesting function: it essentially transfers both the value and ownership of the value to the caller
                                    // and replaces the original location with None
        }));
        self.head = new_head;
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            Some(node) => { // like `case Node a of` in Haskell
                self.head = node.next;
                self.length -= 1;
                Some(node.value)
            }
            None => None,
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        if self.is_empty() {
            self.push_front(value);
            return;
        }

        let new_node = Some(Box::new(Node {
            value,
            next: None,
        }));

        let mut curr = &mut self.head;

        while let Some(node) = curr { // this is a common pattern in Rust: while let Some(x) = y { ... }
                                      // basically saying "while curr matches the pattern Some(node), do this"
            curr = &mut node.next;
        }

        *curr = new_node; // * dereferences the location of current tail's next pointer
        self.length += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.length == 1 {
            return self.pop_front();
        }

        let mut curr = &mut self.head;

        for _ in 0..self.length - 2 { // the _ is just like a Haskell wildcard
                                      // also the 0..self.length - 2 is a range, like [0..n] in Haskell
            if let Some(node) = curr {
                curr = &mut node.next;
            }
        }

        if let Some(node) = curr {
            let last_node = node.next.take();
            self.length -= 1;
            last_node.map(|node| node.value) // a bit to break down here:
                                             // last_node is an Option<Box<Node<T>>>, and map is actually Option::map
                                             // we want to get the value out of the Box, so we map the "closure" defined as |node| node.value over last_node
                                             // closures are literally just lambdas, and they're defined with the |args| body syntax
        } else {
            None // technically unreachable
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let mut curr = &self.head;

        while let Some(node) = curr {
            if node.next.is_none() {
                return Some(&node.value);
            }
            curr = &node.next;
        }

        None
    }
}

#[cfg(test)] // this syntax is called an "attribute", kind of like python decorators but not exactly
             // they can be used for a lot of things, but here it just means "this module is only compiled when running tests"
             // Rust has first class support for testing, so you can run `cargo test` to run all tests in the project
mod tests {
    use super::*; // this imports everything from the parent module, which is the current file

    fn create_test_list<T: Clone>(values: &[T]) -> SinglyLinkedList<T> {
        let mut list = SinglyLinkedList::new();
        for value in values.iter().rev() {
            list.push_front(value.clone());
        }
        list
    }

    #[test]
    fn test_new() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
        assert!(list.head.is_none());
    }

    #[test]
    fn test_is_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert!(list.is_empty());
        
        list.push_front(1);
        assert!(!list.is_empty());
        
        list.pop_front();
        assert!(list.is_empty());
    }

    #[test]
    fn test_size() {
        let mut list = SinglyLinkedList::new();
        assert_eq!(list.size(), 0);
        
        list.push_front(1);
        assert_eq!(list.size(), 1);
        
        list.push_front(2);
        assert_eq!(list.size(), 2);
        
        list.pop_front();
        assert_eq!(list.size(), 1);
        
        list.pop_front();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_push_front() {
        let mut list = SinglyLinkedList::new();
        
        list.push_front(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.size(), 1);
        
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.size(), 2);
        
        let mut string_list = SinglyLinkedList::new();
        string_list.push_front(String::from("hello"));
        assert_eq!(string_list.peek_front(), Some(&String::from("hello")));
        
        string_list.push_front(String::from("world"));
        assert_eq!(string_list.peek_front(), Some(&String::from("world")));
    }

    #[test]
    fn test_pop_front() {
        let mut list = create_test_list(&[1, 2, 3]);
        
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.size(), 2);
        assert_eq!(list.peek_front(), Some(&2));
        
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.size(), 1);
        assert_eq!(list.peek_front(), Some(&3));
        
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.size(), 0);
        assert_eq!(list.peek_front(), None);
        
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_peek_front() {
        let mut list = SinglyLinkedList::new();
        assert_eq!(list.peek_front(), None);
        
        list.push_front(1);
        assert_eq!(list.peek_front(), Some(&1));
        
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));
        
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_push_back() {
        let mut list = SinglyLinkedList::new();
        
        list.push_back(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));
        assert_eq!(list.size(), 1);
        
        list.push_back(2);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&2));
        assert_eq!(list.size(), 2);
        
        list.push_back(3);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));
        assert_eq!(list.size(), 3);
        
        let mut string_list = SinglyLinkedList::new();
        string_list.push_back(String::from("hello"));
        string_list.push_back(String::from("world"));
        assert_eq!(string_list.peek_front(), Some(&String::from("hello")));
        assert_eq!(string_list.peek_back(), Some(&String::from("world")));
    }

    #[test]
    fn test_pop_back() {
        let mut list = create_test_list(&[1, 2, 3]);
        
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.size(), 2);
        assert_eq!(list.peek_back(), Some(&2));
        
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.size(), 1);
        assert_eq!(list.peek_back(), Some(&1));
        
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.size(), 0);
        assert_eq!(list.peek_back(), None);
        
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_peek_back() {
        let mut list = SinglyLinkedList::new();
        assert_eq!(list.peek_back(), None);
        
        list.push_front(1);
        assert_eq!(list.peek_back(), Some(&1));
        
        list.push_back(2);
        assert_eq!(list.peek_back(), Some(&2));
        
        assert_eq!(list.peek_back(), Some(&2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_mixed_operations() {
        let mut list = SinglyLinkedList::new();
        
        list.push_front(2);
        list.push_back(3);
        list.push_front(1);
        
        assert_eq!(list.size(), 3);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));
        
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        
        assert_eq!(list.size(), 1);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&2));
        
        list.push_back(4);
        list.push_front(0);
        
        assert_eq!(list.size(), 3);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }
}