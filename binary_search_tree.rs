use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::Display,
};

struct Node<T>
where
    T: PartialOrd + Ord + Eq + PartialEq + Display,
{
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialOrd + Ord + Eq + PartialEq + Display,
{
    fn new(val: T) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

struct Tree<T>
where
    T: PartialOrd + Ord + Eq + PartialEq + Display,
{
    root: Option<Box<Node<T>>>,
}

impl<T> Tree<T>
where
    T: PartialOrd + Ord + Eq + PartialEq + Display,
{
    fn new() -> Self {
        Tree { root: None }
    }
    fn with_root(val: T) -> Self {
        Tree {
            root: Some(Box::new(Node::new(val))),
        }
    }
    fn add(&mut self, val: T) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node::new(val)));
            }
            _ => {
                Self::add_inner(val, &mut self.root);
            }
        }
    }
    fn add_inner(val: T, root: &mut Option<Box<Node<T>>>) {
        if root.is_none() {
            return;
        }
        let root = root.as_mut();
        match root {
            None => (),
            Some(r) => {
                if val <= r.val {
                    if r.left.is_none() {
                        r.left = Some(Box::new(Node::new(val)));
                    } else {
                        Self::add_inner(val, &mut r.left);
                    }
                } else {
                    if r.right.is_none() {
                        r.right = Some(Box::new(Node::new(val)));
                    } else {
                        Self::add_inner(val, &mut r.right);
                    }
                }
            }
        }
    }
    fn inorder(&self) {
        println!();
        Self::inorder_inner(&self.root);
    }
    fn inorder_inner(root: &Option<Box<Node<T>>>) {
        match root {
            None => (),
            Some(r) => {
                Self::inorder_inner(&r.left);
                print!("{} ", r.val);
                Self::inorder_inner(&r.right);
            }
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.add(5);
    tree.add(1);
    tree.add(7);
    tree.add(9);
    tree.add(0);
    tree.add(2);
    tree.inorder();
}
