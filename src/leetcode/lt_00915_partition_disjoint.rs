use super::Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = nums.len() - 1;
        let mut left_max = nums[0];

        while i != j {
            if nums[i] <= left_max {
                i += 1;
                continue;
            }
            if nums[j] >= left_max {
                j -= 1;
            } else {
                // println!("{i} {j} {left_max}");

                i += 1;
                left_max = left_max.max(nums[i]);
                j = nums.len() - 1;
            }
        }

        let a = (1..nums.len())
            .fold(
                (1, nums[0], nums[0]),
                |(partition, prev_max, curr_max), i| {
                    if nums[i] < prev_max {
                        (i as i32 + 1, curr_max, curr_max)
                    } else {
                        (partition, prev_max, curr_max.max(nums[i]))
                    }
                },
            )
            .0;
        a as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partition_disjoint_should_work() {
        assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    }
}
