use tmp::leetcode::Solution;


#[test]
fn _01827_min_operations_should_work() {
    assert_eq!(Solution::min_operations(vec![1,5,2,4,1]), 14);
    assert_eq!(Solution::min_operations(vec![8]), 0);
    assert_eq!(Solution::min_operations(vec![1,1,1]), 3);
}
