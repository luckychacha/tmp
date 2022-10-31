use super::Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let one_count = nums.iter().fold(0, |count, &item| count + item) as usize;

        let nums = nums.repeat(2);
        let mut window_one_count = nums
            .iter()
            .take(one_count)
            .fold(0, |count, &item| count + item);
        let mut answer = window_one_count;

        for i in 1..=nums.len() / 2 {
            window_one_count += nums[i - 1 + one_count] - nums[i - 1];
            answer = answer.max(window_one_count);
        }

        one_count as i32 - answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min_swaps_should_work() {
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1,);

        assert_eq!(Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2,);

        assert_eq!(Solution::min_swaps(vec![1, 1, 0, 0, 1]), 0);
    }
}
