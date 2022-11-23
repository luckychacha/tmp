use tmp::leetcode::Solution;

#[test]
fn _00227_calculate_should_work() {
    assert_eq!(Solution::calculate(String::from(" 3+5 / 2 ")), 5);
    assert_eq!(Solution::calculate(String::from("3+2*2")), 7);
    // assert_eq!(
    //     Solution::hamming_weight(11111111111111111111111111111101 as u32),
    //     31
    // );
}
