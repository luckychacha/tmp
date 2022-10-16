use super::Solution;

impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        println!("nums:{:?} k:{:?}", nums, k);
        let mut memo = vec![vec![-1; k as usize + 1]; nums.len()];

        let mut total = vec![vec![-1; nums.len()]; nums.len()];

        // sum 是 nums 所有元素的和
        // total[i][j] 是如果 nums 中的 i～j 元素为 1 段时，这段的"总空间"。
        // 因为空间需要覆盖段内所有元素，且要最小，那么只能为这段空间的最大元素，占用的空间最小，再小则无法覆盖所有元素。
        // 可以按照修补木桶来类比，nums[i] ~ nums[j] 是木桶长度不一的木片，若想不漏水，则需要把木片补齐，最小修补方案就是都按照最长的木片的长度来补齐。
        // total[i][j] = max{i, j} * (j-i+1)
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            let mut mx = 0;
            for (j, &item) in nums.iter().enumerate().skip(i) {
                mx = mx.max(item);
                total[i][j] = mx * (j - i + 1) as i32;
            }
        }
        println!("total: {:?}", total);

        // 递归计算
        let res = Solution::dfs(0, k, &nums, &total, &mut memo);
        println!("memo: {:?}", memo);
        res - sum
    }

    pub fn dfs(
        idx: i32,
        k: i32,
        nums: &Vec<i32>,
        total: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        // nums 的所有元素都遍历完了，只能返回 0。
        if idx as usize == nums.len() {
            return 0;
        }

        // 调整次数用完了，只能从当前 idx 开始到最后一个元素作为一段，返回 total[idx][nums.len()-1]
        if k == 0 {
            return total[idx as usize][nums.len() - 1];
        }

        // 之前计算过的中间值，返回之前计算的结果
        if memo[idx as usize][k as usize] >= 0 {
            return memo[idx as usize][k as usize];
        }

        // 
        let mut answer = i32::MAX;
        // nums:[10, 20, 15, 30, 20] k:2
        // total: [[10, 40, 60, 120, 150], [-1, 20, 40, 90, 120], [-1, -1, 15, 60, 90], [-1, -1, -1, 30, 60], [-1, -1, -1, -1, 20]]
        // 1.i=idx=0,k=2 ->
        //  |- i=1,k=1   -> cpr = 10 + 90   ,answer = min(i32.max,100)
        //    |- i=2,k=0    -> 最里层 k == 0 => total[2][nums.len() - 1] = 90, return 90
        //枚举所有的分段位置，结果为Min（当前确定的分段需要的空间+递归计算后续数组少分一段的需要的空间）
        // i = 0,1,2,3,4
        // i = 0时: 
        //  dfs_res = Solution::dfs(1, 1, ...)
        //      |-> total[1][0] + Solution::dfs(2, 0, ...)
        //          |-> Solution::dfs(2, 0, ...)
        //  cpr = total[0][0]+dfs_res
        // total[0][1] + Solution::dfs(1, 1, nums, total, memo);
        // total[0][2] + Solution::dfs(2, 1, nums, total, memo);
        // total[0][3] + Solution::dfs(3, 1, nums, total, memo);
        // total[0][4] + Solution::dfs(4, 1, nums, total, memo);
        for i in idx..nums.len() as i32 {
            let dfs_res = Solution::dfs(i + 1, k - 1, nums, total, memo);
            let cpr = total[idx as usize][i as usize] + dfs_res;
            println!("i: {i} idx: {idx} dfs_res:{dfs_res} cpr: {cpr}");
            answer = answer.min(cpr);
        }
        println!("memo idx:{idx} k: {k} value:{answer}");
        memo[idx as usize][k as usize] = answer;

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
            16,
            Solution::min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2),
        );

        assert_eq!(
            100,
            Solution::min_space_wasted_k_resizing(vec![438,575,10,115,78,968,905,708,926,153,559,37,855,379,597,566,152,441,131,43,316,374,921,956,816,996,882,41,921,367,216,699,256,683,106,363,799,574,906,896,417,998,636,768,159,948], 45),
        )
    }
}
