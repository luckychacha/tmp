use tmp::leetcode::Solution;

#[test]
fn _00214_shortest_palindrome_should_work() {
    assert_eq!(
        Solution::shortest_palindrome(String::from("aacecaaa")),
        String::from("aaacecaaa")
    );
}
