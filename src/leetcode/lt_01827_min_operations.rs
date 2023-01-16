use crate::leetcode::Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let (total, _) = nums
            .iter()
            .fold((0, nums[0] - 1), |(mut total, mut last), &item| {
                if item <= last {
                    total += last - item + 1;
                    last += 1;
                } else {
                    last = item;
                }
                (total, last)
            });

        total
    }
}
