use super::Solution;
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
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        // borrow 可以得到 RefCell<TreeNode>>
        // 因为 RefCell 是智能指针，已经实现了自动解引用（Deref）
        // 所以 dfs 函数接收的 &TreeNode 参数就是引用 borrow 的结果。
        if let Some(node) = root {
            Solution::dfs(&node.clone().borrow(), false, false, &mut ans);
        }
        ans
    }

    fn dfs(current_node: &TreeNode, parent: bool, grandparent: bool, ans: &mut i32) {
        if grandparent {
            *ans += current_node.val;
        }
        let me = current_node.val % 2 == 0;

        if let Some(left) = current_node.left.as_ref() {
            Solution::dfs(&left.clone().borrow(), me, parent, ans);
        }

        if let Some(right) = current_node.right.as_ref() {
            Solution::dfs(&right.clone().borrow(), me, parent, ans);
        }
    }
}
