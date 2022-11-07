// use tmp::leetcode::lt_00767_reorganize_string;
use tmp::leetcode::Solution;

#[test]
fn _00767_reorganize_string_should_work() {
    assert_eq!(Solution::reorganize_string(String::from("aab")), "aba");
    assert_eq!(Solution::reorganize_string(String::from("aaab")), "");
}
