use crate::leetcode::Solution;

pub struct AddTwoListNode {
    pub val: i32,
    pub next: Option<Box<AddTwoListNode>>,
}
impl AddTwoListNode {
    #[inline]
    fn new(val: i32) -> Self {
        AddTwoListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<AddTwoListNode>>,
        l2: Option<Box<AddTwoListNode>>,
    ) -> Option<Box<AddTwoListNode>> {
        let mut v1 = Vec::with_capacity(100);
        let mut v2 = Vec::with_capacity(100);
        let mut m_l1 = l1;
        let mut m_l2 = l2;

        let mut answer = Box::new(AddTwoListNode::new(0));

        while let Some(t) = m_l1 {
            v1.push(t.val);
            m_l1 = t.next;
        }

        while let Some(t) = m_l2 {
            v2.push(t.val);
            m_l2 = t.next;
        }

        let mut carry = 0;
        while !v1.is_empty() || !v2.is_empty() || carry > 0 {
            let x = if let Some(t) = v1.pop() { t } else { 0 };
            let y = if let Some(t) = v2.pop() { t } else { 0 };
            let tmp = x + y + carry;

            let mut node = AddTwoListNode::new(tmp % 10);

            carry = tmp / 10;
            node.next = answer.next.take();
            answer.next = Some(Box::new(node));
        }

        answer.next
    }
}
