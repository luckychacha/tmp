use crate::leetcode::Solution;

/*
1.val = 0，用于缓存最后一个数字
2.queue 用于存储所有数据
3.sign 用于缓存最后一个出现的符号
4.遍历输入的字符串
    如果是数字，则累加 val
    如果是符号，就对出现的最后一个符号进行处理，清空 val，将 sign 换为当前 item
*/

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut queue = vec![];
        let mut sign = b'+';
        let mut val = 0;

        (s + "+").as_bytes().iter().for_each(|&item| match item {
            b'0'..=b'9' => {
                val = val * 10 + item as i32 - '0' as i32;
            }
            b' ' => {}
            _ => {
                match sign {
                    b'+' => {
                        queue.push(val);
                    }
                    b'-' => {
                        queue.push(-val);
                    }
                    b'*' => {
                        if let Some(last) = queue.pop() {
                            queue.push(last * val);
                        }
                    }
                    b'/' => {
                        if let Some(last) = queue.pop() {
                            queue.push(last / val);
                        }
                    }
                    _ => {}
                }
                sign = item;
                val = 0;
            }
        });
        queue.iter().sum::<i32>()
    }
}
