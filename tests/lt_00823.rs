use tmp::leetcode::Solution;

#[test]
fn _00823_num_factored_binary_trees_should_work() {
    assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
    assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
}
