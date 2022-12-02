// Definition for a binary tree node.
// https://leetcode.cn/problems/dui-cheng-de-er-cha-shu-lcof/
use crate::leetcode::Solution;

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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        Solution::is_symmetric_check(
            &root.as_ref().unwrap().borrow().left,
            &root.as_ref().unwrap().borrow().right
        )
    }

    fn is_symmetric_check(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_none() || right.is_none() {
            return false;
        }
        let left_borrow = left.as_ref().unwrap().borrow();
        let right_borrow = right.as_ref().unwrap().borrow();
        left_borrow.val == right_borrow.val
            && Solution::is_symmetric_check(
                &left_borrow.left,
                &right_borrow.right,
            )
            && Solution::is_symmetric_check(
            &left_borrow.right,
            &right_borrow.left
        )
    }
}