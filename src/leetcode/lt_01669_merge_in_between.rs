// Definition for singly-linked list.
// 1669. 合并两个链表
use super::{ListNode, Solution};

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1_curr = list1;
        let mut list2_curr = list2;
        let mut nodes = Vec::new();
        let mut flag = 0;

        while let Some(node1) = list1_curr {
            if flag < a || flag > b {
                nodes.push(node1.val);
            } else {
                while let Some(node2) = list2_curr {
                    nodes.push(node2.val);
                    list2_curr = node2.next;
                }
            }
            list1_curr = node1.next;
            flag += 1;
        }

        nodes.reverse();

        let mut prev = None;

        for item in nodes {
            let last = item;
            let mut node = ListNode::new(last);
            node.next = prev;
            prev = Some(Box::new(node));
        }

        prev
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn better_maximum_binary_string_should_work() {
        let res = Solution::merge_in_between(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
            1,
            1,
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        );

        println!("{:?}", res)
    }
}
