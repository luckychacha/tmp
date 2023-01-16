use super::Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut prev_max = nums[0];
        let mut total_max = nums[0];

        let mut answer = 0;

        for (i, _) in nums.iter().enumerate().skip(1) {
            total_max = total_max.max(nums[i]);

            if nums[i] < prev_max {
                prev_max = total_max;
                answer = i;
            }
        }

        answer as i32 + 1

        // let a = (1..nums.len())
        //     .fold(
        //         (1, nums[0], nums[0]),
        //         |(partition, prev_max, curr_max), i| {
        //             if nums[i] < prev_max {
        //                 (i as i32 + 1, curr_max, curr_max)
        //             } else {
        //                 (partition, prev_max, curr_max.max(nums[i]))
        //             }
        //         },
        //     )
        //     .0;
        // a as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partition_disjoint_should_work() {
        assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
        assert_eq!(Solution::partition_disjoint(vec![1, 1]), 1);
    }
}
