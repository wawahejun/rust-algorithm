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

    fn insert(&mut self, value: T) {
        match &mut self.root {
            Some(node) => node.insert(value),
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    fn search(&self, value: T) -> bool {
        match &self.root {
            Some(node) => node.search(value),
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(left_child) = &mut self.left {
                    left_child.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right_child) = &mut self.right {
                    right_child.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {} // Do nothing for duplicates
        }
    }

    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(left_child) = &self.left {
                    left_child.search(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(right_child) = &self.right {
                    right_child.search(value)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
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


