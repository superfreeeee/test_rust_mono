// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// Constructs a binary tree from a Vec<Option<i32>>.
    /// `vec[i]` represents the value of the node at index `i`, or `None` if the node is absent.
    pub fn from_vec(data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = data.len();
        if n == 0 || data[0].is_none() {
            return None; // Empty tree or root is None
        }

        // Create nodes for all valid entries
        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = data
            .into_iter()
            .map(|opt| opt.map(|val| Rc::new(RefCell::new(TreeNode::new(val)))))
            .collect();

        for i in 0..n {
            if let Some(node) = &nodes[i] {
                let left_index = 2 * i + 1;
                let right_index = 2 * i + 2;

                // Link left child
                if left_index < n {
                    node.borrow_mut().left = nodes[left_index].clone();
                }

                // Link right child
                if right_index < n {
                    node.borrow_mut().right = nodes[right_index].clone();
                }
            }
        }

        nodes[0].clone() // Return the root node
    }
}

use std::cell::RefCell;
use std::rc::Rc;

/// https://leetcode.cn/problems/binary-tree-inorder-traversal/description/
struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }

        let root = root.unwrap();
        res.extend(Self::inorder_traversal(root.borrow().left.clone()));
        res.push(root.borrow().val);
        res.extend(Self::inorder_traversal(root.borrow().right.clone()));

        res
    }
}

#[test]
fn test_1() {
    let root = TreeNode::from_vec(vec![Some(1), None, Some(2), None, None, Some(3)]);
    assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
}
