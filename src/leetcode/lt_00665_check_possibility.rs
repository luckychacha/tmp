use super::Solution;

impl Solution {
    // 需要任意两个连续元素之间是升序关系：i < i + 1, nums[i] < nums[i + 1]
    // 若 nums[idx - 2] > nums[idx]，则 idx - 1 无法修改
    // 若 nums[idx - 1] > nums[idx + 1]，则 idx 无法修改
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut rest_chance = 1;
        for idx in 1..nums.len() {
            if nums[idx - 1] > nums[idx] {
                rest_chance -= 1;
                if rest_chance < 0 {
                    return false;
                }
                if idx > 1
                    && idx < (nums.len() - 1)
                    && nums[idx - 2] > nums[idx]
                    && nums[idx - 1] > nums[idx + 1]
                {
                    return false;
                }
            }
            
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_possibility_should_work() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![3, 4, 2, 3]), false);
        assert_eq!(Solution::check_possibility(vec![1, 4, 1, 2]), true);
        assert_eq!(Solution::check_possibility(vec![1, 2, 5, 3, 3]), true);
    }
}
