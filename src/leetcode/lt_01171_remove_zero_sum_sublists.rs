use std::collections::HashMap;

use super::{ListNode, Solution};

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut node_value_sum_map = HashMap::new();
        node_value_sum_map.insert(0, dummy.as_ref());

        let mut sum = 0;
        let mut d = dummy.next.as_ref();
        while let Some(node) = d {
            sum += node.val;
            d = node.next.as_ref();
            node_value_sum_map.insert(sum, node.as_ref());
        }

        let mut result = Box::new(ListNode::new(0));
        let mut d = Some(&mut result);
        sum = 0;
        while let Some(node) = d {
            sum += node.val;
            if let Some(map_node) = node_value_sum_map.get(&sum) {
                node.next = match map_node.next.as_ref() {
                    Some(next) => Some(Box::new(ListNode::new(next.val))),
                    None => None,
                }
            }
            d = node.next.as_mut();
        }

        result.next
    }
}
