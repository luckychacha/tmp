use tmp::leetcode::Solution;

#[test]
fn _00191_hamming_weight_should_work() {
    assert_eq!(Solution::hamming_weight(11), 3);
    assert_eq!(Solution::hamming_weight(8), 1);
    // assert_eq!(
    //     Solution::hamming_weight(11111111111111111111111111111101 as u32),
    //     31
    // );
}
