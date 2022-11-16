use super::Solution;
impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        // return n.count_ones() as i32
        let mut n = n;
        let mut ret = 0;
        while n != 0 {
            if n & 1 == 1 {
                ret += 1;
            }
            n = n >> 1;
        }
        ret
    }
}
