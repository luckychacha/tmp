use tmp::leetcode::Solution;

#[test]
fn _02300_successful_pairs_should_work() {
    assert_eq!(
        Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
        vec![4, 0, 3]
    );

    assert_eq!(
        Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
        vec![2, 0, 2]
    );
}
