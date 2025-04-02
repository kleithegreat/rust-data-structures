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

        while let Some(node) = curr {
            if let Some(next_node) = &node.next { // note the use of this pattern again except in an if statement
                if next_node.next.is_none() {
                    break;
                }
            }
            curr = &mut node.next;
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
    }
}