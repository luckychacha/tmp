use super::Solution;

impl Solution {

    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        // let n = 200_000_000;
        let mut memo = vec![vec![-1; 200]; 200];
        let mut total = vec![vec![-1; 200]; 200];

        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            let mut mx = 0;
            for j in i..nums.len() {
                mx = mx.max(nums[j]);
                total[i][j] = mx * (j - i + 1) as i32;
            }
        }

        Solution::dp(0, k, &nums, &total, &mut memo) - sum
    }

    pub fn dp(idx: i32, k: i32, nums: &Vec<i32>, total: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if idx as usize == nums.len() {
            return 0;
        }

        if k == 0 {
            return total[idx as usize][nums.len() - 1];
        }
        if memo[idx as usize][k as usize] >= 0 {
            return memo[idx as usize][k as usize];
        }

        let mut answer = 0;
        for i in idx..nums.len() as i32 {
            
            answer = answer.min(total[idx as usize][i as usize] + Solution::dp(i + 1, k - 1, nums, total, memo));
        }
        memo[idx as usize][k as usize] = answer;

        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min_space_wasted_k_resizing_should_work() {
        assert_eq!(
            10,
            Solution::min_space_wasted_k_resizing(vec![10, 20], 0),
        );

        // assert_eq!(
        //     10,
        //     Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1),
        // );

        assert_eq!(
            15,
            Solution::min_space_wasted_k_resizing(vec![10,20,20,30,30], 2),
        );

        // assert_eq!(
        //     100,
        //     Solution::min_space_wasted_k_resizing(vec![438,575,10,115,78,968,905,708,926,153,559,37,855,379,597,566,152,441,131,43,316,374,921,956,816,996,882,41,921,367,216,699,256,683,106,363,799,574,906,896,417,998,636,768,159,948], 45),
        // )
    }

}