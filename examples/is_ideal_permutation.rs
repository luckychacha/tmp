pub struct Solution();
/// 775. 全局倒置与局部倒置
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        for (idx, num) in nums.iter().enumerate() {
            if (num - (idx as i32) < -1) || num - (idx as i32) > 1 {
                return false;
            }
        }


        true
    }
}

fn main() {
    println!("{:?}", Solution::is_ideal_permutation(vec![2,0,1]));
}