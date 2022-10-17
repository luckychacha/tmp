use super::Solution;
// 变量
//  total[i][j]: 从 i 到 j 为 1 段时，占用的最小总空间
//  memo[i][j]：缓存计算的第 i 个元素且有 j 次机会的值
//  actual_space_used：nums 所有元素之和
// 递归函数
//  参数 idx：nums 中的第 idx 个元素
//  参数 k：剩余的调整次数
//  返回：nums 中的第 idx 个元素配合 k 次调整次数的最小空间

// 解题思路
// 计算第 idx 个元素配合 k 次调整的最小空间可以理解为：
//      total[idx][i] + 递归(idx+i, k-1)
//  i 的取值范围： [ idx, nums.len() )
//  tmp = idx ~ i 占用的最小空间（从 total 中取） + 递归计算 第 (idx + i) 个元素配合 (k - 1) 次调整的最小值
//  answer 是所有 tmp 中最小的结果

impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut total = vec![vec![-1; n]; n];
        let mut memo = vec![vec![-1; k as usize + 1]; n];

        let mut actual_space_used = 0;
        for i in 0..n {
            let mut max_value_in_i_to_j = 0;
            actual_space_used += nums[i];
            for (j, &item) in nums.iter().enumerate().skip(i) {
                max_value_in_i_to_j = max_value_in_i_to_j.max(item);
                total[i][j] = max_value_in_i_to_j * (j as i32 - i as i32 + 1);
            }
        }

        Solution::cal_min_space_used(0, k, &nums, &total, &mut memo) - actual_space_used
    }

    pub fn cal_min_space_used(
        idx: usize,
        k: i32,
        nums: &Vec<i32>,
        total: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        let n = nums.len();
        if idx == n {
            return 0;
        }
        if k == 0 {
            return total[idx][n - 1];
        }
        if memo[idx][k as usize] != -1 {
            return memo[idx][k as usize];
        }

        let mut answer = i32::MAX;
        for i in idx..n {
            let min_space_used = Solution::cal_min_space_used(i + 1, k - 1, nums, total, memo);
            answer = answer.min(total[idx][i] + min_space_used);
        }
        memo[idx][k as usize] = answer;

        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min_space_wasted_k_resizing_should_work() {
        assert_eq!(10, Solution::min_space_wasted_k_resizing(vec![10, 20], 0),);

        assert_eq!(
            10,
            Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1),
        );

        assert_eq!(
            15,
            Solution::min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2),
        );

        // assert_eq!(
        //     100,
        //     Solution::min_space_wasted_k_resizing(vec![438,575,10,115,78,968,905,708,926,153,559,37,855,379,597,566,152,441,131,43,316,374,921,956,816,996,882,41,921,367,216,699,256,683,106,363,799,574,906,896,417,998,636,768,159,948], 45),
        // )
    }
}
