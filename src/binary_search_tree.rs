struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {

    }

    pub fn is_empty(&self) -> bool {

    }

    pub fn size(&self) -> usize {

    }

    pub fn clear(&mut self) {

    }

    pub fn height(&self) -> usize {

    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn insert(&mut self, value: T) -> bool {

    }

    pub fn contains(&self, value: &T) -> bool {

    }

    pub fn remove(&mut self, value: &T) -> Option<T> {

    }

    pub fn min(&self) -> Option<&T> {

    }

    pub fn max(&self) -> Option<&T> {

    }

    pub fn in_order<F>(&self, mut visit: F)
    where
        F: Fn(&T),
    {

    }

    pub fn pre_order<F>(&self, mut visit: F)
    where
        F: Fn(&T),
    {

    }

    pub fn post_order<F>(&self, mut visit: F)
    where
        F: Fn(&T),
    {

    }

    pub fn level_order<F>(&self, mut visit: F)
    where
        F: Fn(&T),
    {

    }
}