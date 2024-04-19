/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if let None = self.root {
            self.root = Some(Box::new(TreeNode::new(value)));
        } else {
            let mut cur = &mut self.root;
            while cur.is_some() {
                let ptr = cur.as_mut().unwrap().as_mut();
                if value < ptr.value {
                    if ptr.left.is_none() {
                        ptr.insert(value);
                        return ();
                    }
                    cur = &mut ptr.left;
                } else if value > ptr.value {
                    if ptr.right.is_none() {
                        ptr.insert(value);
                        return();
                    }
                    cur = &mut ptr.right;
                } else {
                    return ();
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut cur = &self.root;
        while cur.is_some() {
            let ptr = cur.as_ref().unwrap().as_ref(); //为什么要把引用cur as_ref()为引用再解Option?
            if ptr.value == value {
                return true;
            } else if ptr.value < value {
                cur = &ptr.right;
            } else {
                cur = &ptr.left;
            }
        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value <= self.value {
            self.left = Some(Box::new(TreeNode::new(value)));
        } else {
            self.right = Some(Box::new(TreeNode::new(value)));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    

