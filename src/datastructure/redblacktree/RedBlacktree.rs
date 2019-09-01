use std::cmp;

struct RedBlacktree<T> 
    where T: Ord, {
    root: Option<Box<RedBlacktreeNode<T>>>,
}

impl RedBlacktree<T>{
    fn new() -> Self {
        RedBlacktree<T> {
            root: None,
        }
    }
}

struct RedBlacktreeNode<T>
    where T: Ord, {
    left: Option<Box<RedBlacktreeNode<T>>>,
    right: Option<Box<RedBlacktreeNode<T>>>,
    value: T,
}

impl RedBlacktreeNode<T> {
    pub fn new(e: T) -> Self {
        RedBlacktreeNode<T> {
            left: None,
            right: None,
            value: e,
        }
    }

    pub fn insert(
