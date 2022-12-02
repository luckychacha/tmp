use crate::leetcode::Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut right_index = citations.len();

        let mut ans = 0;

        while left_index <= right_index {
            let mid = left_index + (right_index - left_index) / 2;

            let mut tmp_count = 0;
            for &item in citations.iter() {
                if item >= mid as i32 {
                    tmp_count += 1;
                }
            }

            if tmp_count >= mid {
                left_index = mid + 1;
                ans = mid;
            } else {
                right_index = mid - 1;
            }
        }

        ans as i32
    }
}