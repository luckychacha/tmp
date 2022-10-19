use super::Solution;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut i = 0;
        let mut answer = 0;

        while i < n - 1 {
            if nums[i] == nums[i + 1] {
                answer += 1;
            } else {
                i += 1;
            }
            i += 1;
        }

        if (n as i32 - answer) % 2 != 0 {
            answer += 1;
        }

        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min_deletion_should_work() {
        assert_eq!(Solution::min_deletion(vec![1, 1, 2, 3, 5]), 1);
        assert_eq!(Solution::min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
    }
}
