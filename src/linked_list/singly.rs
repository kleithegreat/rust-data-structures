struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn size(&self) -> usize {
        self.length
    }

    pub fn push_front(&mut self, value: T) {
        let new_head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = new_head;
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            Some(node) => {
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
        let new_node = Some(Box::new(Node {
            value,
            next: None,
        }));

        let mut curr = &mut self.head;

        while let Some(node) = curr {
            curr = &mut node.next;
        }

        *curr = new_node;
        self.length += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        
    }

    pub fn peek_back(&self) -> Option<&T> {

    }
}